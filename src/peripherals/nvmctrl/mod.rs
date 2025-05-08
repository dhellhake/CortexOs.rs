use core::{cell::RefCell, ptr};

use crate::cortex::{CriticalSection, Mutex};


pub(crate) static NVMCTRL: Mutex<RefCell<Option<NVMController>>> = Mutex::new(RefCell::new(None));

#[repr(C)]
pub struct RegisterBlock {
    pub CTRLA: u16,
    pub Res: u16,
    pub CTRLB: u32,
    pub PARAM: u32,
    pub INTENCLR: u8,
}

pub struct NVMController {
    _reg: &'static mut RegisterBlock,
}

impl NVMController {

    #[inline]
    pub fn new() -> Option<Self> {
        let result: bool = CriticalSection(|st| NVMCTRL.borrow(st).borrow().is_none());

        if result {
            Some(NVMController {
                _reg: unsafe { &mut *(0x41004000 as *mut RegisterBlock) }
            })
        } else {
            None
        }
    }
    
    #[inline]
    pub fn Set_ReadWaitStates(&mut self, rws: RWSSelect) {
        unsafe { 
            let regVal = ptr::read_volatile(&self._reg.CTRLB) & !(0b1111 << 1);
            ptr::write_volatile(&mut self._reg.CTRLB, ((rws as u32) << 1) | regVal)
        }
    }
}

pub enum RWSSelect {
    SINGLE = 0,
    HALF = 1,
    DUAL = 2,
}