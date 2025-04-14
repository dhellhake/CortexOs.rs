use core::{arch::asm, cell::{RefCell, UnsafeCell}, mem, ops::DerefMut};

use task::{Task, TaskStatus};

pub mod task;
pub mod isr;
pub(crate) static Os: OsMutex<RefCell<Option<OperatingSystem>>> = OsMutex::new(RefCell::new(None));

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
        let result: bool = OsSection(|st| Os.borrow(st).borrow().is_none());

        if result {
            Some(OperatingSystem {
                taskIdx: 0,
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
        OsSection(|st| {
            if let Some(ref mut os) = Os.borrow(st).borrow_mut().deref_mut() {
                stack = (&(os.tasks[0].stack[STACK_SIZE - 16]) as *const u32) as u32;
            }
        });

        unsafe {
            asm!("msr psp, {0}", in(reg) stack);
            asm!("msr control, {0}", in(reg) 0x3);
            asm!("isb");
        }

        let mut startTask: *mut Task = (0 as *const u32) as *mut Task;
        OsSection(|st| {
            if let Some(ref mut os) = Os.borrow(st).borrow_mut().deref_mut() {
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
        self.tasks[tIdx].status = TaskStatus::Ready;
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


pub struct OsAccessToken {
    _0: (),
}

impl OsAccessToken {
    pub fn new() -> Self {
        OsAccessToken { _0: () }
    }
}

/// Execute closure `f` in an interrupt-free context.
#[inline]
pub fn OsSection<F, R>(f: F) -> R
where
    F: FnOnce(&OsAccessToken) -> R,
{
    f(&OsAccessToken::new())
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

    pub fn borrow<'st>(&'st self, _st: &'st OsAccessToken) -> &'st T {
        unsafe { &*self.inner.get() }
    }
}
unsafe impl<T> Sync for OsMutex<T> where T: Send {}