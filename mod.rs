use core::{
    arch::asm,
    cell::{
        RefCell,
        UnsafeCell
    },
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
pub(crate) static Os: OsMutex<RefCell<Option<OperatingSystem>>> = OsMutex::new(RefCell::new(None));

pub(crate) const TASK_COUNT: usize = 3;
pub(crate) const STACK_SIZE: usize = 256;

#[derive(Copy, Clone, Debug)]
pub enum OsStatus
{
	UnInit		= 0,
	Ready		= 2,
	Running		= 3,
	Stopped 	= 4,
	Unknown		= 255,
}

#[repr(C, align(4))]
pub struct OperatingSystem {
    pub tasks: [Task; TASK_COUNT],
    pub taskIdx: u32,
    pub osStatus: OsStatus,
    pub elapsedMillis: u64,
}

impl OperatingSystem {

    #[inline]
    pub fn new() -> Option<Self> {
        let result: bool = Os.borrow().borrow().is_none();

        if result {
            Some(OperatingSystem {
                taskIdx: (TASK_COUNT - 1) as u32,
                tasks: [Task { 
                    sp: 0,
                    status: TaskStatus::PreInit,
                    cycletime: TaskCycleTime::NonCyclic,
                    cyclic: empty,
                    id: 0,
                    stack: [0; STACK_SIZE],
                }; TASK_COUNT],
                osStatus: OsStatus::UnInit,
                elapsedMillis: 0,
            })
        } else {
            None
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

    pub fn GetNextTask(&mut self) -> u32 {
        let mut nextTaskIndex: u32 = 0;
        for tIdx in 0..self.tasks.len() {
            match self.tasks[tIdx].status {
                TaskStatus::Active => {
                    break;
                },
                TaskStatus::Pending | TaskStatus::Suspended => {
                    nextTaskIndex = tIdx as u32;
                    break;
                }
                _ => {}
            }
        }
        nextTaskIndex
    }

    #[inline]
    pub fn SetTask(&mut self, tIdx: usize, func: fn(u32), cycletime: TaskCycleTime) {
        self.tasks[tIdx].id = tIdx as u32;
        self.tasks[tIdx].cyclic = func;
        self.tasks[tIdx].cycletime = cycletime;
        self.tasks[tIdx].sp = ((&self.tasks[tIdx].stack[STACK_SIZE - 16]) as *const u32) as u32;
        self.tasks[tIdx].stack[STACK_SIZE - 1] = 0x01000000;
        self.tasks[tIdx].stack[STACK_SIZE - 2] = (cyclic as *const ()) as u32;
        self.tasks[tIdx].stack[STACK_SIZE - 8] = ((self.tasks.as_ptr() as usize) + (mem::size_of::<Task>() * tIdx)) as u32;
        self.tasks[tIdx].status = TaskStatus::Ready;
    }

    #[inline]
    pub fn ResetTask(&mut self, tIdx: usize) {
        self.tasks[tIdx].sp = ((&self.tasks[tIdx].stack[STACK_SIZE - 16]) as *const u32) as u32;
        self.tasks[tIdx].stack[STACK_SIZE - 1] = 0x01000000;
        self.tasks[tIdx].stack[STACK_SIZE - 2] = (cyclic as *const ()) as u32;
        self.tasks[tIdx].stack[STACK_SIZE - 8] = ((self.tasks.as_ptr() as usize) + (mem::size_of::<Task>() * tIdx)) as u32;
    }

    fn ContextSwitch(&mut self, curTIdx: usize, setTIdx: usize)
    {
        let mut t0sp: u32 = ((&self.tasks[curTIdx].sp) as *const u32) as u32;
        let mut t1sp: u32 = ((&self.tasks[setTIdx].sp) as *const u32) as u32;
        unsafe {
            asm!(
                "cpsid i",
                "mrs	r0, psp",
                "subs	r0, #16",
                "stmia	r0!,{{r4-r7}}",
                "mov	r4, r8",
                "mov	r5, r9",
                "mov	r6, r10",
                "mov	r7, r11",
                "subs	r0, #32",
                "stmia	r0!,{{r4-r7}}",
                "subs	r0, #16",
                "str	r0, [r1]",
                inout("r1") t0sp,
            );

            asm!(
                "ldr	r0, [r1]",
                "ldmia	r0!,{{r4-r7}}",
                "mov	r8, r4",
                "mov	r9, r5",
                "mov	r10, r6",
                "mov	r11, r7",
                "ldmia	r0!,{{r4-r7}}",
                "msr	psp, r0",
                "ldr r0, =0xFFFFFFFD",
                "cpsie	i",
                inout("r1") t1sp,
            );
        }
    }
}


pub fn cyclic(task: *mut Task, _tstmp: u32) -> ! {
    let fun: fn(u32) = unsafe { (*task).cyclic };

    fun(_tstmp);
    
    unsafe { 
        (*task).status = TaskStatus::Finished;
        asm!("svc 0");
    }

    loop { }
}


pub struct OsMutex<T> {
    inner: UnsafeCell<T>,
}

impl<T> OsMutex<T> {
    /// Creates a new OsMutex
    pub const fn new(value: T) -> Self {
        OsMutex {
            inner: UnsafeCell::new(value),
        }
    }

    pub fn borrow<'st>(&'st self) -> &'st T {
        unsafe { &*self.inner.get() }
    }
}
unsafe impl<T> Sync for OsMutex<T> where T: Send {}