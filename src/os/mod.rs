use core::cell::{RefCell, UnsafeCell};
 
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