use core::{cell::UnsafeCell, ptr};

use crate::cortex::{CriticalSection, Mutex};

pub(crate) static SCB: Mutex<UnsafeCell<Option<SystemControlBlock>>> = Mutex::new(UnsafeCell::new(None));

#[repr(C)]
pub struct RegisterBlock {
    /// CPUID Register
    pub CPUID: u32,
    /// Interrupt Control and State Register
    pub ICSR: u32,
    /// Vector Table Offset Register
    pub VTOR: u32,
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
        let mut result: bool = true;        
        unsafe {
            result = CriticalSection(|| SCB.borrow().as_ref_unchecked().is_none());    
        }

        if result {
            Some(SystemControlBlock {
                _reg: unsafe { &mut *(0xE000ED00 as *mut RegisterBlock) }
            })
        } else {
            None
        }
    }
    
    #[inline]
    pub unsafe fn Set_PendSV(&mut self) {
        ptr::write_volatile(&mut self._reg.ICSR, 1 << 28)
    }
    
    #[inline]
    pub unsafe fn Set_VectorTableOffset(&mut self, tbloff: u32) {
        ptr::write_volatile(&mut self._reg.VTOR, tbloff & 0xFFFFFFC0)
    }
}