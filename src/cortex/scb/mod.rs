use core::cell::RefCell;
use super::{CriticalSection, Mutex};

pub(crate) static SCB: Mutex<RefCell<Option<SystemControlBlock>>> = Mutex::new(RefCell::new(None));

#[repr(C)]
pub struct RegisterBlock {
    /// CPUID Register
    pub CPUID: u32,
    /// Interrupt Control and State Register
    pub ICSR: u32,
    /// Application Interrupt and Reset Control Register
    pub AIRCR: u32,
    /// System Control Register
    pub SCR: u32,
    /// Configuration and Control Register
    pub CCR: u32,
    /// System Handler Priority Register 2
    pub SHPR2: u32,
    /// System Handler Priority Register 3
    pub SHPR3: u32,
}

pub struct SystemControlBlock {
    _reg: &'static mut RegisterBlock,
}

impl SystemControlBlock {

    #[inline]
    pub fn new() -> Option<Self> {
        let result: bool = CriticalSection(|st| SCB.borrow(st).borrow().is_none());

        if result {
            Some(SystemControlBlock {
                _reg: unsafe { &mut *(0xE000ED00 as *mut RegisterBlock) }
            })
        } else {
            None
        }
    }
}