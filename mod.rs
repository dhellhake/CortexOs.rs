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

use crate::mcu::{
    SCB,
    SYSTICK
};

pub mod task;
pub mod isr;
pub mod intercom;

#[repr(C, align(4))]
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
                stack: [0; STACK_SIZE],
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
            SYSTICK.with(|syst| syst.SetTimer(earliestTime as u32));
            if self.taskIdx != (self.tasks.len() - 1) as u32 {
                SCB.with(|scb| scb.SetPendSV());
            }
        }
    }

    #[inline]
    pub fn SetTask(&mut self, tIdx: usize, func: fn(u32), cycletime: TaskCycleTime) {
        self.tasks[tIdx].id = tIdx as u32;
        self.tasks[tIdx].cyclic = func;
        self.tasks[tIdx].cycletime = cycletime;
        self.tasks[tIdx].sp = ((&self.tasks[tIdx].stack[STACK_SIZE - 16]) as *const u32) as u32;
        self.tasks[tIdx].stack[STACK_SIZE - 1] = 0x01000000;
        self.tasks[tIdx].stack[STACK_SIZE - 2] = (cyclic::<STACK_SIZE> as *const ()) as u32;
        self.tasks[tIdx].stack[STACK_SIZE - 8] = ((self.tasks.as_ptr() as usize) + (mem::size_of::<Task<STACK_SIZE>>() * tIdx)) as u32;
        self.tasks[tIdx].status = TaskStatus::Ready;
    }

    #[inline]
    pub fn ResetTask(&mut self, tIdx: usize) {
        self.tasks[tIdx].sp = ((&self.tasks[tIdx].stack[STACK_SIZE - 16]) as *const u32) as u32;
        self.tasks[tIdx].stack[STACK_SIZE - 1] = 0x01000000;
        self.tasks[tIdx].stack[STACK_SIZE - 2] = (cyclic::<STACK_SIZE> as *const ()) as u32;
        self.tasks[tIdx].stack[STACK_SIZE - 8] = ((self.tasks.as_ptr() as usize) + (mem::size_of::<Task<STACK_SIZE>>() * tIdx)) as u32;
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