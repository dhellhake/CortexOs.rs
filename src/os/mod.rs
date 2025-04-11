use core::{arch::asm, cell::{RefCell, UnsafeCell}, mem, ops::DerefMut};

use crate::{cortex, SCB};
 
pub(crate) static Os: OsMutex<RefCell<Option<OperatingSystem>>> = OsMutex::new(RefCell::new(None));

#[repr(C)]
#[derive(Copy, Clone, Debug)]
 pub struct Task 
 {
    pub sp: u32,
    pub cyclic: fn(u32),
    pub stack: [u32; 256],
 }
 
 fn empty(_tstmp: u32) {
    loop {}
 }

 pub fn ContextSwitch(task0: &mut Task, task1: &mut Task)
{
    let mut t0sp: u32 = ((&task0.sp) as *const u32) as u32;
    let mut t1sp: u32 = ((&task1.sp) as *const u32) as u32;
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

 #[derive(Copy, Clone, Debug)]
pub enum TaskStatus
{
	PreInit		= 0,
	Suspended	= 1,
	Ready		= 2,
	Active		= 3,
	Finished	= 4,
	Unknown		= 255,
}

#[repr(C, align(4))]
pub struct OperatingSystem {
    pub tasks: [Task; 2],
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
                    cyclic: empty,
                    stack: [0; 256],
                }; 2],
            })
        } else {
            None
        }
    }

    #[inline]
    pub fn SetTask(&mut self, tIdx: usize, func: fn(u32)) {
        self.tasks[tIdx].cyclic = func;
        self.tasks[tIdx].sp = ((&self.tasks[tIdx].stack[256 - 16]) as *const u32) as u32;
        self.tasks[tIdx].stack[256 - 1] = 0x01000000;
        self.tasks[tIdx].stack[256 - 2] = (cyclic as *const ()) as u32;
        self.tasks[tIdx].stack[256 - 8] = ((self.tasks.as_ptr() as usize) + (mem::size_of::<Task>() * tIdx)) as u32;
    }
}

#[no_mangle]
pub unsafe extern "C" fn SysTick_Isr() {  
    cortex::CriticalSection(|st| {
        if let Some(ref mut scb) = SCB.borrow(st).borrow_mut().deref_mut() {
            scb.Set_PendSV();
        }
    });
}

#[no_mangle]
pub unsafe extern "C" fn PendSV() {
    OsSection(|st| {
        if let Some(ref mut os) = Os.borrow(st).borrow_mut().deref_mut() {
            let [ref mut a, ref mut b, ..] = os.tasks;
            if os.taskIdx == 0 {
                ContextSwitch(a, b);

                let stackIdx = 255 - ((((&a.stack[255]) as *const u32) as u32) - a.sp) / 4;

                a.stack[stackIdx as usize + 14] = (cyclic as *const ()) as u32;
                for stackOffset in 1..6 {
                    a.stack[stackIdx as usize + 14 - stackOffset as usize] = 0;
                }
                a.stack[stackIdx as usize + 14 - 6] = (a as *const Task) as u32;

                os.taskIdx = 1;
            } else {
                ContextSwitch(b, a);

                let stackIdx = 255 - ((((&b.stack[255]) as *const u32) as u32) - b.sp) / 4;

                b.stack[stackIdx as usize + 14] = (cyclic as *const ()) as u32;
                for stackOffset in 1..6 {
                    b.stack[stackIdx as usize + 14 - stackOffset as usize] = 0;
                }
                b.stack[stackIdx as usize + 14 - 6] = (b as *const Task) as u32;

                os.taskIdx = 0;
            }
        }
    });
}


pub fn cyclic(task: *const Task) {
    unsafe {
        ((*task).cyclic)(123);
    }
    loop { }
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