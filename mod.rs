use core::{
    arch::asm,
    cell::UnsafeCell,
    mem
};

use task::{
    empty,
    Task,
    TaskCycleTime,
    TaskStatus
};

use crate::{
    mcu::{
        SCB,
        SYSTICK
    },
    os::task::Stack
};

pub mod task;
pub mod isr;
pub mod intercom;

const INITIAL_XPSR: u32 = 0x0100_0000;
const STACK_PATTERN: u32 = 0xE25A_2EA5;
const STACK_GUARD: u32 = 0xDEAD_BEEF;
const STACK_GUARD_WORDS: usize = 4;
const INITIAL_FRAME_WORDS: usize = 16;

#[no_mangle]
pub extern "C" fn task_return_trap() -> ! {
    loop {}
}

#[repr(C, align(8))]
pub struct Application<const TASK_COUNT: usize, const STACK_SIZE: usize> {
    pub tasks: [Task<STACK_SIZE>; TASK_COUNT],
    pub taskIdx: u32,
    pub elapsedMillis: u64,
}

impl<const TASK_COUNT: usize, const STACK_SIZE: usize> Application<TASK_COUNT, STACK_SIZE> {

    #[inline]
    pub fn new() -> Self {
        Application {
            taskIdx: (TASK_COUNT - 1) as u32,
            tasks: [Task { 
                sp: 0,
                status: TaskStatus::PreInit,
                cycletime: TaskCycleTime::NonCyclic,
                cyclic: empty,
                id: 0,
                timestamp_us: 0,
                stack: Stack::new(),
            }; TASK_COUNT],
            elapsedMillis: 0,
        }
    }

    pub fn InvokeSchedule(&mut self, elapsed_us: u64)
    {
        let mut earliestTime = u64::max_value();
        for tIdx in 0..self.tasks.len() {
            match self.tasks[tIdx].cycletime {
                TaskCycleTime::NonCyclic => {},
                _ => {
                    match self.tasks[tIdx].status {
                        TaskStatus::Active | TaskStatus::Suspended | TaskStatus::Finished => {
                            // MTA
                        },
                        TaskStatus::Pending => {
                            earliestTime = 0;
                        },
                        _ => {
                            let cycletime_us = self.tasks[tIdx].cycletime as u64 * 1000;
                            let deadline = cycletime_us - (elapsed_us % cycletime_us);
                            let time = elapsed_us - (elapsed_us % cycletime_us) + cycletime_us;

                            if deadline >= (cycletime_us - 100) {
                                self.tasks[tIdx].status = TaskStatus::Pending;
                                earliestTime = 0;
                            } else {
                                if time < earliestTime {
                                    earliestTime = time; 
                                }
                            }
                        }
                    }
                }
            }
        }
        
        if earliestTime == 0 {
            
        SCB.with(|scb| scb.SetPendSV());
        } else {
            let armed = SYSTICK.with(|syst| syst.SetTimerAt(earliestTime));
            if !armed {
                SCB.with(|scb| scb.SetPendSV());
            }
        }
    }

    #[inline]
    pub fn SetTask(&mut self, tIdx: usize, func: fn(u32), cycletime: TaskCycleTime) {
        self.tasks[tIdx].id = tIdx as u32;
        self.tasks[tIdx].cyclic = func;
        self.tasks[tIdx].cycletime = cycletime;
        self.tasks[tIdx].timestamp_us = 0;
        self.PrepareTaskStack(tIdx);
        self.tasks[tIdx].status = TaskStatus::Ready;
    }

    #[inline]
    pub fn ResetTask(&mut self, tIdx: usize) {
        self.PrepareTaskStack(tIdx);
    }

    #[inline]
    fn PrepareTaskStack(&mut self, tIdx: usize) {
        assert!(STACK_SIZE >= INITIAL_FRAME_WORDS + STACK_GUARD_WORDS);

        // Required for 8-byte stack alignment because each stack entry is 4 bytes.
        assert!((STACK_SIZE & 1) == 0);

        let task_ptr =
            ((self.tasks.as_ptr() as usize)
                + (mem::size_of::<Task<STACK_SIZE>>() * tIdx)) as u32;

        let stack = &mut self.tasks[tIdx].stack.0;

        for word in stack.iter_mut() {
            *word = STACK_PATTERN;
        }

        for idx in 0..STACK_GUARD_WORDS {
            stack[idx] = STACK_GUARD;
        }

        let frame_base = STACK_SIZE - INITIAL_FRAME_WORDS;

        self.tasks[tIdx].sp = (&stack[frame_base] as *const u32) as u32;

        debug_assert_eq!(self.tasks[tIdx].sp & 0x7, 0);

        // Software-saved frame.
        // Your context switcher restores this as r8-r11 first, then r4-r7.
        stack[STACK_SIZE - 16] = STACK_PATTERN; // r8
        stack[STACK_SIZE - 15] = STACK_PATTERN; // r9
        stack[STACK_SIZE - 14] = STACK_PATTERN; // r10
        stack[STACK_SIZE - 13] = STACK_PATTERN; // r11
        stack[STACK_SIZE - 12] = STACK_PATTERN; // r4
        stack[STACK_SIZE - 11] = STACK_PATTERN; // r5
        stack[STACK_SIZE - 10] = STACK_PATTERN; // r6
        stack[STACK_SIZE - 9]  = STACK_PATTERN; // r7

        // Hardware exception frame consumed by Cortex-M exception return.
        stack[STACK_SIZE - 8] = task_ptr;                              // r0
        stack[STACK_SIZE - 7] = self.tasks[tIdx].timestamp_us;         // r1
        stack[STACK_SIZE - 6] = 0;                                     // r2
        stack[STACK_SIZE - 5] = 0;                                     // r3
        stack[STACK_SIZE - 4] = 0;                                     // r12
        stack[STACK_SIZE - 3] = (task_return_trap as *const ()) as u32; // lr
        stack[STACK_SIZE - 2] = ((cyclic::<STACK_SIZE> as *const ()) as u32) | 1; // pc
        stack[STACK_SIZE - 1] = INITIAL_XPSR;                          // xPSR
    }

    pub fn GetNextTask(&mut self) -> u32 {
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
    #[inline(never)]
    pub fn PendSVSelectNext(&mut self, current_sp: u32) -> u32 {
        let current = self.taskIdx as usize;

        // Save outgoing task's PSP.
        self.tasks[current].sp = current_sp;

        match self.tasks[current].status {
            TaskStatus::Active => {
                self.tasks[current].status = TaskStatus::Suspended;
            },
            TaskStatus::Finished => {
                self.ResetTask(current);
                self.tasks[current].status = TaskStatus::Ready;
            },
            _ => {}
        }

        let next = self.GetNextTask() as usize;

        self.tasks[next].status = TaskStatus::Active;
        self.taskIdx = next as u32;

        self.tasks[next].sp
    }
}


pub fn cyclic<const M: usize>(task: *mut Task<M>, _tstmp: u32) -> ! {
    let fun: fn(u32) = unsafe { (*task).cyclic };

    fun(_tstmp);
    
    unsafe { 
        (*task).status = TaskStatus::Finished;
        asm!("svc 0");
    }

    loop { }
}


pub struct Mutex<T> {
    inner: UnsafeCell<T>,
}

impl<T> Mutex<T> {
    /// Creates a new Mutex
    pub const fn new(value: T) -> Self {
        Mutex {
            inner: UnsafeCell::new(value),
        }
    }

    pub fn borrow<'st>(&'st self) -> &'st T {
        unsafe { &*self.inner.get() }
    }
}
unsafe impl<T> Sync for Mutex<T> where T: Send {}