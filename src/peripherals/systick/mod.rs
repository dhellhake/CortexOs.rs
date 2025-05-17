#![allow(non_camel_case_types)]

use core::{cell::UnsafeCell, ptr};

use crate::cortex::{CriticalSection, Mutex};

pub(crate) static SysTick: Mutex<UnsafeCell<Option<SystemTimer>>> = Mutex::new(UnsafeCell::new(None));

#[repr(C)]
pub struct RegisterGroup {
    // SysTick Control and Status Register
    pub CSR: u32,
    // SysTick Reload Value Register
    pub RVR: u32,
    // SysTick Current Value Register
    pub CVR: u32,
    // SysTick Calibration Value Register
    pub CALIB: u32,
}

pub struct SystemTimer {
    _reg: &'static mut RegisterGroup,
}

impl SystemTimer {
    #[inline]
    pub fn new() -> Option<Self> {
        let mut result: bool = true;
        unsafe {
            result = CriticalSection(|| SysTick.borrow().as_ref_unchecked().is_none());
        }

        if result {
            Some(SystemTimer {
                _reg: unsafe { &mut *(0xE000E010 as *mut RegisterGroup) }
            })
        } else {
            None
        }
    }

    #[inline]
    pub unsafe fn WriteRaw_ControlAndStatusRegister(&mut self, regVal: u32) {
        ptr::write_volatile(&mut self._reg.CSR, regVal)
    }
    #[inline]
    pub unsafe fn ReadRaw_ControlAndStatusRegister(&mut self) -> u32 {
        ptr::read_volatile(&mut self._reg.CSR)
    }

    #[inline]
    pub fn Set_ENABLE(&mut self, val: SysTick_CSR__ENABLE) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.CSR) & !(0x1);
            ptr::write_volatile(&mut self._reg.CSR, ((val as u32) << 0) | regVal)
        }
    }
    #[inline]
    pub fn Set_TICKINT(&mut self, val: SysTick_CSR__TICKINT) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.CSR) & !(0x2);
            ptr::write_volatile(&mut self._reg.CSR, ((val as u32) << 1) | regVal)
        }
    }
    #[inline]
    pub fn Set_CLKSOURCE(&mut self, val: SysTick_CSR__CLKSOURCE) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.CSR) & !(0x4);
            ptr::write_volatile(&mut self._reg.CSR, ((val as u32) << 2) | regVal)
        }
    }
    #[inline]
    pub fn Set_COUNTFLAG(&mut self, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.CSR) & !(0x10000);
            ptr::write_volatile(&mut self._reg.CSR, ((val as u32) << 16) | regVal)
        }
    }

    #[inline]
    pub unsafe fn WriteRaw_ReloadValueRegister(&mut self, regVal: u32) {
        ptr::write_volatile(&mut self._reg.RVR, regVal)
    }
    #[inline]
    pub unsafe fn ReadRaw_ReloadValueRegister(&mut self) -> u32 {
        ptr::read_volatile(&mut self._reg.RVR)
    }

    #[inline]
    pub fn Set_RELOAD(&mut self, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.RVR) & !(0xFFFFFF);
            ptr::write_volatile(&mut self._reg.RVR, ((val as u32) << 0) | regVal)
        }
    }

    #[inline]
    pub unsafe fn WriteRaw_CurrentValueRegister(&mut self, regVal: u32) {
        ptr::write_volatile(&mut self._reg.CVR, regVal)
    }
    #[inline]
    pub unsafe fn ReadRaw_CurrentValueRegister(&mut self) -> u32 {
        ptr::read_volatile(&mut self._reg.CVR)
    }

    #[inline]
    pub fn Set_CURRENT(&mut self, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.CVR) & !(0xFFFFFF);
            ptr::write_volatile(&mut self._reg.CVR, ((val as u32) << 0) | regVal)
        }
    }

    #[inline]
    pub unsafe fn ReadRaw_CalibrationValueRegister(&mut self) -> u32 {
        ptr::read_volatile(&mut self._reg.CALIB)
    }


}

pub enum SysTick_CSR__CLKSOURCE {
    VALUE_0 = 0,                                    //External clock
    VALUE_1 = 1,                                    //Processor clock
}

pub enum SysTick_CSR__ENABLE {
    VALUE_0 = 0,                                    //Counter disabled
    VALUE_1 = 1,                                    //Counter enabled
}

pub enum SysTick_CSR__TICKINT {
    VALUE_0 = 0,                                    //Counting down to 0 does not assert the SysTick exception request
    VALUE_1 = 1,                                    //Counting down to 0 asserts the SysTick exception request
}

pub enum SysTick_CALIB__NOREF {
    VALUE_0 = 0,                                    //The reference clock is provided
    VALUE_1 = 1,                                    //The reference clock is not provided
}

pub enum SysTick_CALIB__SKEW {
    VALUE_0 = 0,                                    //10ms calibration value is exact
    VALUE_1 = 1,                                    //10ms calibration value is inexact, because of the clock frequency
}

