use core::{arch::asm, cell::{RefCell, UnsafeCell}, mem, ops::DerefMut};

use task::{Task, TaskStatus};

pub mod task;
pub mod isr;
pub(crate) static Os: OsMutex<RefCell<Option<OperatingSystem>>> = OsMutex::new(RefCell::new(None));

pub(crate) const START_TASK: usize = 0;
pub(crate) const TASK_COUNT: usize = 2;
pub(crate) const STACK_SIZE: usize = 256;

#[repr(C, align(4))]
pub struct OperatingSystem {
    pub tasks: [Task; TASK_COUNT],
    pub taskIdx: u32,
}

impl OperatingSystem {
    #[inline]
    pub fn new() -> Option<Self> {
        let result: bool = OsSection(|| Os.borrow().borrow().is_none());

        if result {
            Some(OperatingSystem {
                taskIdx: START_TASK as u32,
                tasks: [Task { 
                    sp: 0,
                    status: TaskStatus::PreInit,
                    cyclic: empty,
                    stack: [0; STACK_SIZE],
                }; TASK_COUNT],
            })
        } else {
            None
        }
    }
    
    pub fn OsStart() -> ! {
        let mut stack: u32 = 0;
        OsSection(|| {
            if let Some(ref mut os) = Os.borrow().borrow_mut().deref_mut() {
                stack = (&(os.tasks[os.taskIdx as usize].stack[STACK_SIZE - 16]) as *const u32) as u32;
            }
        });

        unsafe {
            asm!("msr psp, {0}", in(reg) stack);
            asm!("msr control, {0}", in(reg) 0x3);
            asm!("isb");
        }

        let mut startTask: *mut Task = (0 as *const u32) as *mut Task;
        OsSection(|| {
            if let Some(ref mut os) = Os.borrow().borrow_mut().deref_mut() {
                startTask = &mut (os.tasks[os.taskIdx as usize]);
            }
        });
        
        cyclic(startTask);
    }


    #[inline]
    pub fn SetTask(&mut self, tIdx: usize, func: fn(u32)) {
        self.tasks[tIdx].cyclic = func;
        self.tasks[tIdx].sp = ((&self.tasks[tIdx].stack[STACK_SIZE - 16]) as *const u32) as u32;
        self.tasks[tIdx].stack[STACK_SIZE - 1] = 0x01000000;
        self.tasks[tIdx].stack[STACK_SIZE - 2] = (cyclic as *const ()) as u32;
        self.tasks[tIdx].stack[STACK_SIZE - 8] = ((self.tasks.as_ptr() as usize) + (mem::size_of::<Task>() * tIdx)) as u32;
        self.tasks[tIdx].status = TaskStatus::Ready;
    }

    #[inline]
    pub fn ResetTask(&mut self, tIdx: usize) {
        let stackIdx = (STACK_SIZE as u32 - 1) - ((((&self.tasks[tIdx].stack[STACK_SIZE - 1]) as *const u32) as u32) - self.tasks[tIdx].sp) / 4;

        self.tasks[tIdx].stack[stackIdx as usize + 14] = (cyclic as *const ()) as u32;
        for stackOffset in 1..6 {
            self.tasks[tIdx].stack[stackIdx as usize + 14 - stackOffset as usize] = 0;
        }
        self.tasks[tIdx].stack[stackIdx as usize + 14 - 6] = (&self.tasks[tIdx] as *const Task) as u32;
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


fn cyclic(task: *mut Task) -> ! {

    unsafe {
        ((*task).cyclic)(123);
        (*task).status = TaskStatus::Finished;
    }

    loop { }
}

fn empty(_tstmp: u32) {
    loop {}
}

/// Execute closure `f` in an interrupt-free context.
#[inline]
pub fn OsSection<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    f()
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