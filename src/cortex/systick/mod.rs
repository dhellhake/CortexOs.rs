use core::{cell::RefCell, ptr};
use super::{CriticalSection, Mutex};

pub(crate) static SysTick: Mutex<RefCell<Option<SystemTimer>>> = Mutex::new(RefCell::new(None));

#[repr(C)]
pub struct RegisterBlock {
    /// Control and Status
    pub CTRL: u32,
    /// Reload Value Register
    pub LOAD: u32,
    /// Counter Value
    pub VAL: u32,
    /// Calibration Value
    pub CALIB: u32,
}

pub struct SystemTimer {
    _reg: &'static mut RegisterBlock,
}

impl SystemTimer {

    #[inline]
    pub fn new() -> Option<Self> {
        let result: bool = CriticalSection(|st | SysTick.borrow(st).borrow().is_none());

        if result {
            Some(SystemTimer {
                _reg: unsafe { &mut *(0xE000E010 as *mut RegisterBlock) }
            })
        } else {
            None
        }
    }

    #[inline]
    pub fn Get_ReloadValue(&self) -> u32 {
        unsafe { 
            ptr::read_volatile(&self._reg.LOAD)
        }
    }    

    #[inline]
    pub fn Set_ReloadValue(&mut self, value: u32) {
        unsafe { 
            ptr::write_volatile(&mut self._reg.LOAD, value)
        }
    }
    
    #[inline]
    pub fn Set_ControlValue(&mut self, value: u32) {
        unsafe { 
            ptr::write_volatile(&mut self._reg.CTRL, value)
        }
    }
    
    #[inline]
    pub fn Set_CounterValue(&mut self, value: u32) {
        unsafe { 
            ptr::write_volatile(&mut self._reg.VAL, value)
        }
    }

}
