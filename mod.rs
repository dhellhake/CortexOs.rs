use core::{arch::asm, ptr};

use task::{
    TaskControl, TaskCycleTime, TaskFunction, TaskHandle, TaskRole, TaskStatus,
    INITIAL_FRAME_WORDS, STACK_GUARD_WORDS,
};

use crate::mcu::{McuManager, SCB, SCHEDULER, SYSTICK};

pub mod intercom;
pub mod isr;
pub mod task;

const INITIAL_XPSR: u32 = 0x0100_0000;
const STACK_PATTERN: u32 = 0xE25A_2EA5;
const STACK_GUARD: u32 = 0xDEAD_BEEF;

#[no_mangle]
pub extern "C" fn task_return_trap() -> ! {
    loop {
        core::hint::spin_loop();
    }
}

#[repr(C, align(8))]
pub struct Scheduler<const TASK_COUNT: usize> {
    tasks: [TaskHandle; TASK_COUNT],
    taskIdx: u32,
    elapsedMillis: u64,
}

impl<const TASK_COUNT: usize> Scheduler<TASK_COUNT> {
    /// Creates a scheduler bound to statically pinned task objects.
    ///
    /// # Safety
    ///
    /// Every handle must identify a distinct task and remain exclusively bound
    /// to this `Scheduler` for its full lifetime. No task may be registered
    /// with another scheduler or accessed outside this scheduler's
    /// interrupt-serialized control path.
    #[inline]
    pub const unsafe fn new(tasks: [TaskHandle; TASK_COUNT]) -> Self {
        assert!(TASK_COUNT > 0);

        Scheduler {
            taskIdx: (TASK_COUNT - 1) as u32,
            tasks,
            elapsedMillis: 0,
        }
    }

    /// Returns an immutable control-block reference tied to this scheduler
    /// borrow, never to the complete task or its live stack.
    #[inline]
    fn control(&self, tIdx: usize) -> &TaskControl {
        let control = self.tasks[tIdx].control_ptr();
        unsafe { &*control }
    }

    /// Returns the uniquely borrowed control block for one owned task.
    #[inline]
    fn control_mut(&mut self, tIdx: usize) -> &mut TaskControl {
        let control = self.tasks[tIdx].control_ptr();
        unsafe { &mut *control }
    }

    pub fn InvokeSchedule(&mut self, now_us: u64) {
        let mut need_switch = false;
        let mut next_wakeup_us = u64::MAX;

        for tIdx in 0..self.tasks.len() {
            let Some(period_us) = self.control(tIdx).cycletime.period_us() else {
                continue;
            };

            let mut prepare_stack = false;

            {
                let task = self.control_mut(tIdx);

                if now_us >= task.next_release_us {
                    let release_us = task.next_release_us;
                    let due_count = ((now_us - release_us) / period_us) + 1;

                    task.next_release_us =
                        release_us.saturating_add(due_count.saturating_mul(period_us));

                    match task.status {
                        TaskStatus::Ready => {
                            task.timestamp_us = release_us;
                            prepare_stack = true;

                            if due_count > 1 {
                                task.missed_releases =
                                    task.missed_releases.saturating_add((due_count - 1) as u32);
                            }
                        }

                        TaskStatus::Pending
                        | TaskStatus::Active
                        | TaskStatus::Suspended
                        | TaskStatus::Finished => {
                            task.missed_releases =
                                task.missed_releases.saturating_add(due_count as u32);
                        }

                        _ => {}
                    }
                }
            }

            if prepare_stack {
                self.PrepareTaskStack(tIdx);
                self.control_mut(tIdx).status = TaskStatus::Pending;
                need_switch = true;
            }

            let task = self.control(tIdx);

            if matches!(task.status, TaskStatus::Pending) {
                need_switch = true;
            }

            next_wakeup_us = next_wakeup_us.min(task.next_release_us);
        }

        if next_wakeup_us != u64::MAX {
            let armed = SYSTICK.with(|syst| syst.SetTimerAt(next_wakeup_us));
            if !armed {
                // The requested deadline is already due or too close to arm
                // safely. SetTimerAt installed a long safety interval; pend a
                // fresh SysTick pass instead of relying on a nanosecond retry.
                SCB.with(|scb| scb.SetSysTickPending());
            }
        }

        let current = self.taskIdx as usize;
        if need_switch || matches!(self.control(current).status, TaskStatus::Finished) {
            SCB.with(|scb| scb.SetPendSV());
        }
    }

    #[inline]
    pub fn SetTask(
        &mut self,
        tIdx: usize,
        func: TaskFunction,
        cycletime: TaskCycleTime,
        role: TaskRole,
    ) {
        assert!(matches!(self.control(tIdx).status, TaskStatus::PreInit));

        {
            let task = self.control_mut(tIdx);
            task.id = tIdx as u32;
            task.cyclic = Some(func);
            task.cycletime = cycletime;
            task.role = role;
            task.timestamp_us = 0;
            task.next_release_us = cycletime.period_us().unwrap_or(0);
            task.missed_releases = 0;
        }

        self.PrepareTaskStack(tIdx);
        self.control_mut(tIdx).status = TaskStatus::Ready;
    }

    /// Selects the configured last task as the initial background context.
    pub fn ActivateBackgroundTask(&mut self) -> u32 {
        let background = TASK_COUNT - 1;
        assert!(matches!(
            self.control(background).role,
            TaskRole::Background
        ));
        assert!(matches!(self.control(background).status, TaskStatus::Ready));

        self.control_mut(background).status = TaskStatus::Active;
        self.taskIdx = background as u32;
        self.control(background).sp
    }

    #[inline]
    pub fn SetCyclicReleaseBase(&mut self, base_us: u64) {
        for tIdx in 0..self.tasks.len() {
            let task = self.control_mut(tIdx);
            let Some(period_us) = task.cycletime.period_us() else {
                continue;
            };

            task.timestamp_us = base_us;
            task.next_release_us = base_us.saturating_add(period_us);
            task.missed_releases = 0;
        }
    }

    #[inline]
    fn ResetTask(&mut self, tIdx: usize) {
        self.PrepareTaskStack(tIdx);
    }

    #[inline]
    fn PrepareTaskStack(&mut self, tIdx: usize) {
        // SAFETY: callers prepare only pre-init, ready, or finished tasks. The
        // selected PSP stack is therefore not executing while it is rebuilt.
        debug_assert!(matches!(
            self.control(tIdx).status,
            TaskStatus::PreInit | TaskStatus::Ready | TaskStatus::Finished
        ));

        let handle = self.tasks[tIdx];
        let stack = handle.stack_ptr();
        let stack_size = handle.stack_size_words();

        for idx in 0..stack_size {
            unsafe { ptr::write(stack.add(idx), STACK_PATTERN) };
        }

        for idx in 0..STACK_GUARD_WORDS {
            unsafe { ptr::write(stack.add(idx), STACK_GUARD) };
        }

        let frame_base = stack_size - INITIAL_FRAME_WORDS;
        let sp = unsafe { stack.add(frame_base) } as u32;

        debug_assert_eq!(sp & 0x7, 0);

        // Software-saved frame.
        // Your context switcher restores this as r8-r11 first, then r4-r7.
        unsafe {
            ptr::write(stack.add(stack_size - 16), STACK_PATTERN); // r8
            ptr::write(stack.add(stack_size - 15), STACK_PATTERN); // r9
            ptr::write(stack.add(stack_size - 14), STACK_PATTERN); // r10
            ptr::write(stack.add(stack_size - 13), STACK_PATTERN); // r11
            ptr::write(stack.add(stack_size - 12), STACK_PATTERN); // r4
            ptr::write(stack.add(stack_size - 11), STACK_PATTERN); // r5
            ptr::write(stack.add(stack_size - 10), STACK_PATTERN); // r6
            ptr::write(stack.add(stack_size - 9), STACK_PATTERN); // r7
        }

        // Hardware exception frame consumed by Cortex-M exception return.
        unsafe {
            ptr::write(stack.add(stack_size - 8), tIdx as u32); // r0: task ID
            ptr::write(stack.add(stack_size - 7), 0); // r1
            ptr::write(stack.add(stack_size - 6), 0); // r2
            ptr::write(stack.add(stack_size - 5), 0); // r3
            ptr::write(stack.add(stack_size - 4), 0); // r12
            ptr::write(
                stack.add(stack_size - 3),
                (task_return_trap as *const ()) as u32,
            ); // lr
            ptr::write(
                stack.add(stack_size - 2),
                ((cyclic as *const ()) as u32) | 1,
            ); // pc
            ptr::write(stack.add(stack_size - 1), INITIAL_XPSR); // xPSR
        }

        self.control_mut(tIdx).sp = sp;
    }

    fn GetNextTask(&mut self) -> u32 {
        // Run newly released work first.
        for tIdx in 0..self.tasks.len() {
            if matches!(self.control(tIdx).status, TaskStatus::Pending) {
                return tIdx as u32;
            }
        }

        // Then resume a preempted task, including the idle/background task.
        for tIdx in 0..self.tasks.len() {
            if matches!(self.control(tIdx).status, TaskStatus::Suspended) {
                return tIdx as u32;
            }
        }

        // Last task is configured as the background/idle task in main().
        (self.tasks.len() - 1) as u32
    }

    /// Called from the PendSV assembly handler after r4-r11 of the outgoing task
    /// have already been saved to `current_sp`.
    ///
    /// Returns the saved stack pointer of the task that shall be restored.
    ///
    /// # Safety
    ///
    /// `current_sp` must point to the complete software frame of the active
    /// task, and this must run from PendSV on MSP with interrupts masked.
    #[inline(never)]
    pub(crate) unsafe fn PendSVSelectNext(&mut self, current_sp: u32) -> u32 {
        let current = self.taskIdx as usize;

        // Save outgoing task's PSP.
        self.control_mut(current).sp = current_sp;

        match self.control(current).status {
            TaskStatus::Active => {
                self.control_mut(current).status = TaskStatus::Suspended;
            }
            TaskStatus::Finished => {
                self.ResetTask(current);
                self.control_mut(current).status = TaskStatus::Ready;
            }
            _ => {}
        }

        let next = self.GetNextTask() as usize;

        self.control_mut(next).status = TaskStatus::Active;
        self.taskIdx = next as u32;

        self.control(next).sp
    }
}

/// Executes a task from the synthetic exception frame built by
/// `PrepareTaskStack`.
///
/// # Safety
///
/// `taskId` must identify the task whose prepared PSP context entered this
/// function.
pub unsafe extern "C" fn cyclic(taskId: u32) -> ! {
    let (fun, tstmp, cycletime, role) = SCHEDULER.with(|scheduler| {
        let task = scheduler.control(taskId as usize);
        let fun = match task.cyclic {
            Some(fun) => fun,
            None => task_return_trap(),
        };

        (fun, task.timestamp_us, task.cycletime, task.role)
    });

    if role.ReportsProgramFlowCheckpoints(cycletime) {
        McuManager::ProgramFlow_ReportTaskStart(taskId);
    }
    fun(tstmp);
    if role.ReportsProgramFlowCheckpoints(cycletime) {
        McuManager::ProgramFlow_ReportTaskEnd(taskId);
    }

    SCHEDULER.with(|scheduler| {
        scheduler.control_mut(taskId as usize).status = TaskStatus::Finished;
    });

    unsafe { asm!("svc 0") };

    loop {
        core::hint::spin_loop();
    }
}
