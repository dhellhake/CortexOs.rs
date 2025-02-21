#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::{arch::asm, cell::UnsafeCell};

pub mod startup;
pub mod systick;
pub mod nvic;
pub mod scb;

pub struct CriticalSectionToken {
    _0: (),
}

impl CriticalSectionToken {
    pub fn new() -> Self {
        CriticalSectionToken { _0: () }
    }
}

/// Execute closure `f` in an interrupt-free context.
#[inline]
pub fn CriticalSection<F, R>(f: F) -> R
where
    F: FnOnce(&CriticalSectionToken) -> R,
{
    let primask: u32;
    unsafe { asm!("MRS {0}, primask", out(reg) primask) };
    
    unsafe { asm!("cpsid i") };

    let r = f(&CriticalSectionToken::new());

    if primask & (1 << 0) != (1 << 0) {
        unsafe { asm!("cpsie i") };
    }

    r
}

pub struct Mutex<T> {
    inner: UnsafeCell<T>,
}

impl<T> Mutex<T> {
    /// Creates a new mutex
    pub const fn new(value: T) -> Self {
        Mutex {
            inner: UnsafeCell::new(value),
        }
    }

    pub fn borrow<'st>(&'st self, _st: &'st CriticalSectionToken) -> &'st T {
        unsafe { &*self.inner.get() }
    }
}

unsafe impl<T> Sync for Mutex<T> where T: Send {}