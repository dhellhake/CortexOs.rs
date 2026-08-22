use core::{arch::asm, ptr};

use task::{empty, Task, TaskCycleTime, TaskRole, TaskStackStorage, TaskStatus};

use crate::mcu::{McuManager, Os, SCB, SYSTICK};

pub mod intercom;
pub mod isr;
pub mod task;

const INITIAL_XPSR: u32 = 0x0100_0000;
const STACK_PATTERN: u32 = 0xE25A_2EA5;
const STACK_GUARD: u32 = 0xDEAD_BEEF;
const STACK_GUARD_WORDS: usize = 4;
const INITIAL_FRAME_WORDS: usize = 16;

#[no_mangle]
pub extern "C" fn task_return_trap() -> ! {
    loop {
        core::hint::spin_loop();
    }
}

#[repr(C, align(8))]
pub struct Application<const TASK_COUNT: usize, const STACK_SIZE: usize> {
    tasks: [Task; TASK_COUNT],
    taskIdx: u32,
    elapsedMillis: u64,
    stacks: &'static TaskStackStorage<TASK_COUNT, STACK_SIZE>,
}

impl<const TASK_COUNT: usize, const STACK_SIZE: usize> Application<TASK_COUNT, STACK_SIZE> {
    /// Creates scheduler metadata bound to dedicated static task-stack storage.
    ///
    /// # Safety
    ///
    /// `stacks` must be owned exclusively by this `Application` for the full
    /// lifetime of the application. No second application may use it.
    #[inline]
    pub const unsafe fn new(stacks: &'static TaskStackStorage<TASK_COUNT, STACK_SIZE>) -> Self {
        assert!(TASK_COUNT > 0);
        assert!(STACK_SIZE >= INITIAL_FRAME_WORDS + STACK_GUARD_WORDS);
        assert!((STACK_SIZE & 1) == 0);

        Application {
            taskIdx: (TASK_COUNT - 1) as u32,
            tasks: [Task {
                sp: 0,
                status: TaskStatus::PreInit,
                cycletime: TaskCycleTime::NonCyclic,
                cyclic: empty,
                role: TaskRole::Background,
                id: 0,
                timestamp_us: 0,
                next_release_us: 0,
                missed_releases: 0,
            }; TASK_COUNT],
            elapsedMillis: 0,
            stacks,
        }
    }

    pub fn InvokeSchedule(&mut self, now_us: u64) {
        let mut need_switch = false;
        let mut next_wakeup_us = u64::MAX;

        for tIdx in 0..self.tasks.len() {
            let Some(period_us) = self.tasks[tIdx].cycletime.period_us() else {
                continue;
            };

            if now_us >= self.tasks[tIdx].next_release_us {
                let release_us = self.tasks[tIdx].next_release_us;
                let due_count = ((now_us - release_us) / period_us) + 1;

                self.tasks[tIdx].next_release_us =
                    release_us.saturating_add(due_count.saturating_mul(period_us));

                match self.tasks[tIdx].status {
                    TaskStatus::Ready => {
                        self.tasks[tIdx].timestamp_us = release_us;
                        self.PrepareTaskStack(tIdx);
                        self.tasks[tIdx].status = TaskStatus::Pending;
                        need_switch = true;

                        if due_count > 1 {
                            self.tasks[tIdx].missed_releases = self.tasks[tIdx]
                                .missed_releases
                                .saturating_add((due_count - 1) as u32);
                        }
                    }

                    TaskStatus::Pending
                    | TaskStatus::Active
                    | TaskStatus::Suspended
                    | TaskStatus::Finished => {
                        self.tasks[tIdx].missed_releases = self.tasks[tIdx]
                            .missed_releases
                            .saturating_add(due_count as u32);
                    }

                    _ => {}
                }
            }

            if matches!(self.tasks[tIdx].status, TaskStatus::Pending) {
                need_switch = true;
            }

            next_wakeup_us = next_wakeup_us.min(self.tasks[tIdx].next_release_us);
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
        if need_switch || matches!(self.tasks[current].status, TaskStatus::Finished) {
            SCB.with(|scb| scb.SetPendSV());
        }
    }

    #[inline]
    pub fn SetTask(
        &mut self,
        tIdx: usize,
        func: fn(u64),
        cycletime: TaskCycleTime,
        role: TaskRole,
    ) {
        assert!(matches!(self.tasks[tIdx].status, TaskStatus::PreInit));

        self.tasks[tIdx].id = tIdx as u32;
        self.tasks[tIdx].cyclic = func;
        self.tasks[tIdx].cycletime = cycletime;
        self.tasks[tIdx].role = role;
        self.tasks[tIdx].timestamp_us = 0;
        self.tasks[tIdx].next_release_us = cycletime.period_us().unwrap_or(0);
        self.tasks[tIdx].missed_releases = 0;
        self.PrepareTaskStack(tIdx);
        self.tasks[tIdx].status = TaskStatus::Ready;
    }

    /// Selects the configured last task as the initial background context.
    pub fn ActivateBackgroundTask(&mut self) -> u32 {
        let background = TASK_COUNT - 1;
        assert!(matches!(self.tasks[background].role, TaskRole::Background));
        assert!(matches!(self.tasks[background].status, TaskStatus::Ready));

        self.tasks[background].status = TaskStatus::Active;
        self.taskIdx = background as u32;
        self.tasks[background].sp
    }

    #[inline]
    pub fn SetCyclicReleaseBase(&mut self, base_us: u64) {
        for task in self.tasks.iter_mut() {
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
        let stack = unsafe { self.stacks.stack_ptr(tIdx) };

        for idx in 0..STACK_SIZE {
            unsafe { ptr::write(stack.add(idx), STACK_PATTERN) };
        }

        for idx in 0..STACK_GUARD_WORDS {
            unsafe { ptr::write(stack.add(idx), STACK_GUARD) };
        }

        let frame_base = STACK_SIZE - INITIAL_FRAME_WORDS;

        self.tasks[tIdx].sp = unsafe { stack.add(frame_base) } as u32;

        debug_assert_eq!(self.tasks[tIdx].sp & 0x7, 0);

        // Software-saved frame.
        // Your context switcher restores this as r8-r11 first, then r4-r7.
        unsafe {
            ptr::write(stack.add(STACK_SIZE - 16), STACK_PATTERN); // r8
            ptr::write(stack.add(STACK_SIZE - 15), STACK_PATTERN); // r9
            ptr::write(stack.add(STACK_SIZE - 14), STACK_PATTERN); // r10
            ptr::write(stack.add(STACK_SIZE - 13), STACK_PATTERN); // r11
            ptr::write(stack.add(STACK_SIZE - 12), STACK_PATTERN); // r4
            ptr::write(stack.add(STACK_SIZE - 11), STACK_PATTERN); // r5
            ptr::write(stack.add(STACK_SIZE - 10), STACK_PATTERN); // r6
            ptr::write(stack.add(STACK_SIZE - 9), STACK_PATTERN); // r7
        }

        // Hardware exception frame consumed by Cortex-M exception return.
        unsafe {
            ptr::write(stack.add(STACK_SIZE - 8), tIdx as u32); // r0: task ID
            ptr::write(stack.add(STACK_SIZE - 7), 0); // r1
            ptr::write(stack.add(STACK_SIZE - 6), 0); // r2
            ptr::write(stack.add(STACK_SIZE - 5), 0); // r3
            ptr::write(stack.add(STACK_SIZE - 4), 0); // r12
            ptr::write(
                stack.add(STACK_SIZE - 3),
                (task_return_trap as *const ()) as u32,
            ); // lr
            ptr::write(
                stack.add(STACK_SIZE - 2),
                ((cyclic as *const ()) as u32) | 1,
            ); // pc
            ptr::write(stack.add(STACK_SIZE - 1), INITIAL_XPSR); // xPSR
        }
    }

    fn GetNextTask(&mut self) -> u32 {
        // Run newly released work first.
        for tIdx in 0..self.tasks.len() {
            if matches!(self.tasks[tIdx].status, TaskStatus::Pending) {
                return tIdx as u32;
            }
        }

        // Then resume a preempted task, including the idle/background task.
        for tIdx in 0..self.tasks.len() {
            if matches!(self.tasks[tIdx].status, TaskStatus::Suspended) {
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
        self.tasks[current].sp = current_sp;

        match self.tasks[current].status {
            TaskStatus::Active => {
                self.tasks[current].status = TaskStatus::Suspended;
            }
            TaskStatus::Finished => {
                self.ResetTask(current);
                self.tasks[current].status = TaskStatus::Ready;
            }
            _ => {}
        }

        let next = self.GetNextTask() as usize;

        self.tasks[next].status = TaskStatus::Active;
        self.taskIdx = next as u32;

        self.tasks[next].sp
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
    let (fun, tstmp, cycletime, role) = Os.with(|os| {
        let task = &os.tasks[taskId as usize];
        (task.cyclic, task.timestamp_us, task.cycletime, task.role)
    });

    if role.ReportsProgramFlowCheckpoints(cycletime) {
        McuManager::ProgramFlow_ReportTaskStart(taskId);
    }
    fun(tstmp);
    if role.ReportsProgramFlowCheckpoints(cycletime) {
        McuManager::ProgramFlow_ReportTaskEnd(taskId);
    }

    Os.with(|os| os.tasks[taskId as usize].status = TaskStatus::Finished);

    unsafe { asm!("svc 0") };

    loop {
        core::hint::spin_loop();
    }
}
