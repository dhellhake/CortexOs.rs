#![allow(non_camel_case_types)]

use core::{cell::UnsafeCell, ptr};

use crate::cortex::{CriticalSection, Mutex};

pub(crate) static GCLK: Mutex<UnsafeCell<Option<GenericClockGenerator>>> = Mutex::new(UnsafeCell::new(None));

#[repr(C)]
pub struct RegisterGroup {
    // Control
    pub CTRLA: u8,
    pub res0: [u8; 4],
    // Synchronization Busy
    pub SYNCBUSY: u32,
    pub res1: [u8; 28],
    // Generic Clock Generator Control
    pub GENCTRL: [u32; 9],
    pub res2: [u8; 96],
    // Peripheral Clock Control
    pub PCHCTRL: [u32; 41],
}

pub struct GenericClockGenerator {
    _reg: &'static mut RegisterGroup,
}

impl GenericClockGenerator {
    #[inline]
    pub fn new() -> Option<Self> {
        let mut result: bool = true;
        unsafe {
            result = CriticalSection(|| GCLK.borrow().as_ref_unchecked().is_none());
        }

        if result {
            Some(GenericClockGenerator {
                _reg: unsafe { &mut *(0x40001C00 as *mut RegisterGroup) }
            })
        } else {
            None
        }
    }

    #[inline]
    pub unsafe fn WriteRaw_Control(&mut self, regVal: u8) {
        ptr::write_volatile(&mut self._reg.CTRLA, regVal)
    }
    #[inline]
    pub unsafe fn ReadRaw_Control(&mut self) -> u8 {
        ptr::read_volatile(&mut self._reg.CTRLA)
    }

    #[inline]
    pub fn Set_CTRLA_SWRST(&mut self, val: u8) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.CTRLA) & !(0x1);
            ptr::write_volatile(&mut self._reg.CTRLA, ((val as u8) << 0) | regVal)
        }
    }

    #[inline]
    pub unsafe fn ReadRaw_SynchronizationBusy(&mut self) -> u32 {
        ptr::read_volatile(&mut self._reg.SYNCBUSY)
    }


    #[inline]
    pub unsafe fn WriteRaw_GenericClockGeneratorControl(&mut self, regIdx: usize, regVal: u32) {
        ptr::write_volatile(&mut self._reg.GENCTRL[regIdx], regVal)
    }
    #[inline]
    pub unsafe fn ReadRaw_GenericClockGeneratorControl(&mut self, regIdx: usize) -> u32 {
        ptr::read_volatile(&mut self._reg.GENCTRL[regIdx])
    }

    #[inline]
    pub fn Set_GENCTRL_SRC(&mut self, regIdx: usize, val: GCLK_GENCTRL__SRC) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.GENCTRL[regIdx]) & !(0x7);
            ptr::write_volatile(&mut self._reg.GENCTRL[regIdx], ((val as u32) << 0) | regVal)
        }
    }
    #[inline]
    pub fn Set_GENCTRL_GENEN(&mut self, regIdx: usize, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.GENCTRL[regIdx]) & !(0x100);
            ptr::write_volatile(&mut self._reg.GENCTRL[regIdx], ((val as u32) << 8) | regVal)
        }
    }
    #[inline]
    pub fn Set_GENCTRL_IDC(&mut self, regIdx: usize, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.GENCTRL[regIdx]) & !(0x200);
            ptr::write_volatile(&mut self._reg.GENCTRL[regIdx], ((val as u32) << 9) | regVal)
        }
    }
    #[inline]
    pub fn Set_GENCTRL_OOV(&mut self, regIdx: usize, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.GENCTRL[regIdx]) & !(0x400);
            ptr::write_volatile(&mut self._reg.GENCTRL[regIdx], ((val as u32) << 10) | regVal)
        }
    }
    #[inline]
    pub fn Set_GENCTRL_OE(&mut self, regIdx: usize, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.GENCTRL[regIdx]) & !(0x800);
            ptr::write_volatile(&mut self._reg.GENCTRL[regIdx], ((val as u32) << 11) | regVal)
        }
    }
    #[inline]
    pub fn Set_GENCTRL_DIVSEL(&mut self, regIdx: usize, val: GCLK_GENCTRL__DIVSEL) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.GENCTRL[regIdx]) & !(0x1000);
            ptr::write_volatile(&mut self._reg.GENCTRL[regIdx], ((val as u32) << 12) | regVal)
        }
    }
    #[inline]
    pub fn Set_GENCTRL_RUNSTDBY(&mut self, regIdx: usize, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.GENCTRL[regIdx]) & !(0x2000);
            ptr::write_volatile(&mut self._reg.GENCTRL[regIdx], ((val as u32) << 13) | regVal)
        }
    }
    #[inline]
    pub fn Set_GENCTRL_DIV(&mut self, regIdx: usize, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.GENCTRL[regIdx]) & !(0xFFFF0000);
            ptr::write_volatile(&mut self._reg.GENCTRL[regIdx], ((val as u32) << 16) | regVal)
        }
    }

    #[inline]
    pub unsafe fn WriteRaw_PeripheralClockControl(&mut self, regIdx: usize, regVal: u32) {
        ptr::write_volatile(&mut self._reg.PCHCTRL[regIdx], regVal)
    }
    #[inline]
    pub unsafe fn ReadRaw_PeripheralClockControl(&mut self, regIdx: usize) -> u32 {
        ptr::read_volatile(&mut self._reg.PCHCTRL[regIdx])
    }

    #[inline]
    pub fn Set_PCHCTRL_GEN(&mut self, regIdx: usize, val: GCLK_PCHCTRL__GEN) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.PCHCTRL[regIdx]) & !(0xF);
            ptr::write_volatile(&mut self._reg.PCHCTRL[regIdx], ((val as u32) << 0) | regVal)
        }
    }
    #[inline]
    pub fn Set_PCHCTRL_CHEN(&mut self, regIdx: usize, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.PCHCTRL[regIdx]) & !(0x40);
            ptr::write_volatile(&mut self._reg.PCHCTRL[regIdx], ((val as u32) << 6) | regVal)
        }
    }
    #[inline]
    pub fn Set_PCHCTRL_WRTLOCK(&mut self, regIdx: usize, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.PCHCTRL[regIdx]) & !(0x80);
            ptr::write_volatile(&mut self._reg.PCHCTRL[regIdx], ((val as u32) << 7) | regVal)
        }
    }

}

pub enum GCLK_SYNCBUSY__GENCTRL {
    GCLK0 = 1,                                      //Generic clock generator 0
    GCLK1 = 2,                                      //Generic clock generator 1
    GCLK2 = 4,                                      //Generic clock generator 2
    GCLK3 = 8,                                      //Generic clock generator 3
    GCLK4 = 16,                                     //Generic clock generator 4
    GCLK5 = 32,                                     //Generic clock generator 5
    GCLK6 = 64,                                     //Generic clock generator 6
    GCLK7 = 128,                                    //Generic clock generator 7
    GCLK8 = 256,                                    //Generic clock generator 8
}

pub enum GCLK_GENCTRL__SRC {
    XOSC = 0,                                       //XOSC oscillator output
    GCLKIN = 1,                                     //Generator input pad
    GCLKGEN1 = 2,                                   //Generic clock generator 1 output
    OSCULP32K = 3,                                  //OSCULP32K oscillator output
    OSC32K = 4,                                     //OSC32K oscillator output
    XOSC32K = 5,                                    //XOSC32K oscillator output
    OSC48M = 6,                                     //OSC48M oscillator output
    DPLL96M = 7,                                    //DPLL96M output
}

pub enum GCLK_PCHCTRL__GEN {
    GCLK0 = 0,                                      //Generic clock generator 0
    GCLK1 = 1,                                      //Generic clock generator 1
    GCLK2 = 2,                                      //Generic clock generator 2
    GCLK3 = 3,                                      //Generic clock generator 3
    GCLK4 = 4,                                      //Generic clock generator 4
    GCLK5 = 5,                                      //Generic clock generator 5
    GCLK6 = 6,                                      //Generic clock generator 6
    GCLK7 = 7,                                      //Generic clock generator 7
    GCLK8 = 8,                                      //Generic clock generator 8
}

pub enum GCLK_GENCTRL__DIVSEL {
    DIV1 = 0,                                       //Divide input directly by divider factor
    DIV2 = 1,                                       //Divide input by 2^(divider factor+ 1)
}

