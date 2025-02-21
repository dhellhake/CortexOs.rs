
use core::cell::RefCell;

use super::{CriticalSection, Mutex};

pub(crate) static NVIC: Mutex<RefCell<Option<NestedVectoredInterruptController>>> = Mutex::new(RefCell::new(None));

#[repr(C)]
pub struct RegisterBlock {
    /// Interrupt Set-enable Register
    pub SETENA: u32,
    /// Interrupt Clear-enable Register
    pub CLRENA: u32,
    /// Interrupt Set-pending Register
    pub SETPEND: u32,
    /// Interrupt Clear-pending Register
    pub CLRPEND: u32,
    /// Interrupt Priority Registers
    pub IP: [u32; 8],
}

pub struct NestedVectoredInterruptController {
    _reg: &'static mut RegisterBlock,
}

impl NestedVectoredInterruptController {

    #[inline]
    pub fn new() -> Option<Self> {
        let result: bool = CriticalSection(|st| NVIC.borrow(st).borrow().is_none());

        if result {
            Some(NestedVectoredInterruptController {
                _reg: unsafe { &mut *(0xE000E100 as *mut RegisterBlock) }
            })
        } else {
            None
        }
    }
}