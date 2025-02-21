use core::{cell::RefCell, ptr};

use crate::cortex::{CriticalSection, Mutex};


pub(crate) static PORT: Mutex<RefCell<Option<IOPinController>>> = Mutex::new(RefCell::new(None));


#[repr(C)]
pub struct RegisterBlock {
    /// Interrupt Set-enable Register
    pub Group: [PortGroup; 2],
}

#[repr(C)]
pub struct PortGroup {
    /// Interrupt Set-enable Register
    pub DIR: u32,           /**< \brief Offset: 0x00 (R/W 32) Data Direction */
    pub DIRCLR: u32,        /**< \brief Offset: 0x04 (R/W 32) Data Direction Clear */
    pub DIRSET: u32,        /**< \brief Offset: 0x08 (R/W 32) Data Direction Set */
    pub DIRTGL: u32,        /**< \brief Offset: 0x0C (R/W 32) Data Direction Toggle */
    pub OUT: u32,           /**< \brief Offset: 0x10 (R/W 32) Data Output Value */
    pub OUTCLR: u32,        /**< \brief Offset: 0x14 (R/W 32) Data Output Value Clear */
    pub OUTSET: u32,        /**< \brief Offset: 0x18 (R/W 32) Data Output Value Set */
    pub OUTTGL: u32,        /**< \brief Offset: 0x1C (R/W 32) Data Output Value Toggle */
    pub IN: u32,            /**< \brief Offset: 0x20 (R/  32) Data Input Value */
    pub CTRL: u32,          /**< \brief Offset: 0x24 (R/W 32) Control */
    pub WRCONFIG: u32,      /**< \brief Offset: 0x28 ( /W 32) Write Configuration */
    pub EVCTRL: u32,        /**< \brief Offset: 0x2C (R/W 32) Event Input Control */
    pub PMUX: [u8; 16],     /**< \brief Offset: 0x30 (R/W  8) Peripheral Multiplexing n */
    pub PINCFG: [u8; 32],   /**< \brief Offset: 0x40 (R/W  8) Pin Configuration n */
    pub Res: [u8; 0x20],
}


pub struct IOPinController {
    _reg: &'static mut RegisterBlock,
}

impl IOPinController {

    #[inline]
    pub fn new() -> Option<Self> {
        let result: bool = CriticalSection(|st| PORT.borrow(st).borrow().is_none());

        if result {
            Some(IOPinController {
                _reg: unsafe { &mut *(0x41000000 as *mut RegisterBlock) }
            })
        } else {
            None
        }
    }
    
    #[inline]
    pub fn Set_PinDirection(&mut self, pingroup: usize, pin: u8, isOut: bool) {
        unsafe {
            if isOut
            {
                ptr::write_volatile(&mut self._reg.Group[pingroup].DIRSET, 1 << pin)
            } else {
                ptr::write_volatile(&mut self._reg.Group[pingroup].DIRCLR, 1 << pin)
            }
        }
    }

    #[inline]
    pub fn Set_PinOutState(&mut self, pingroup: usize, pin: u8, state: bool) {
        unsafe {
            if state
            {
                ptr::write_volatile(&mut self._reg.Group[pingroup].OUTSET, 1 << pin)
            } else {
                ptr::write_volatile(&mut self._reg.Group[pingroup].OUTCLR, 1 << pin)
            }
        }
    }
    
    #[inline]
    pub fn Get_PinOutState(&mut self, pingroup: usize, pin: u8) -> bool {
        unsafe {
            ptr::read_volatile(&mut self._reg.Group[pingroup].OUT) & (1 << pin) > 0
        }
    }
}