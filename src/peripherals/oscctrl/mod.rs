#![allow(non_camel_case_types)]

use core::{cell::UnsafeCell, ptr};

use crate::cortex::{CriticalSection, Mutex};

pub(crate) static OSCCTRL: Mutex<UnsafeCell<Option<OscillatorsControl>>> = Mutex::new(UnsafeCell::new(None));

#[repr(C)]
pub struct RegisterGroup {
    // Interrupt Enable Clear
    pub INTENCLR: u32,
    // Interrupt Enable Set
    pub INTENSET: u32,
    // Interrupt Flag Status and Clear
    pub INTFLAG: u32,
    // Power and Clocks Status
    pub STATUS: u32,
    // External Multipurpose Crystal Oscillator (XOSC) Control
    pub XOSCCTRL: u16,
    // Clock Failure Detector Prescaler
    pub CFDPRESC: u8,
    // Event Control
    pub EVCTRL: u8,
    // 48MHz Internal Oscillator (OSC48M) Control
    pub OSC48MCTRL: u8,
    // OSC48M Divider
    pub OSC48MDIV: u8,
    // OSC48M Startup Time
    pub OSC48MSTUP: u8,
    pub res0: u8,
    // OSC48M Synchronization Busy
    pub OSC48MSYNCBUSY: u32,
    // DPLL Control
    pub DPLLCTRLA: u8,
    pub res1: [u8; 3],
    // DPLL Ratio Control
    pub DPLLRATIO: u32,
    // Digital Core Configuration
    pub DPLLCTRLB: u32,
    // DPLL Prescaler
    pub DPLLPRESC: u8,
    pub res2: [u8; 3],
    // DPLL Synchronization Busy
    pub DPLLSYNCBUSY: u8,
    pub res3: [u8; 3],
    // DPLL Status
    pub DPLLSTATUS: u8,
    pub res4: [u8; 7],
    // 48MHz Oscillator Calibration
    pub CAL48M: u32,
}

pub struct OscillatorsControl {
    _reg: &'static mut RegisterGroup,
}

impl OscillatorsControl {
    #[inline]
    pub fn new() -> Option<Self> {
        let mut result: bool = true;
        unsafe {
            result = CriticalSection(|| OSCCTRL.borrow().as_ref_unchecked().is_none());
        }

        if result {
            Some(OscillatorsControl {
                _reg: unsafe { &mut *(0x40001000 as *mut RegisterGroup) }
            })
        } else {
            None
        }
    }

    // --------------------------------------------------
    // Raw Register Access
    // --------------------------------------------------
    #[inline]
    pub unsafe fn WriteRaw_InterruptEnableClear(&mut self, regVal: u32) {
        ptr::write_volatile(&mut self._reg.INTENCLR, regVal)
    }
    #[inline]
    pub unsafe fn ReadRaw_InterruptEnableClear(&mut self) -> u32 {
        ptr::read_volatile(&mut self._reg.INTENCLR)
    }

    #[inline]
    pub unsafe fn WriteRaw_InterruptEnableSet(&mut self, regVal: u32) {
        ptr::write_volatile(&mut self._reg.INTENSET, regVal)
    }
    #[inline]
    pub unsafe fn ReadRaw_InterruptEnableSet(&mut self) -> u32 {
        ptr::read_volatile(&mut self._reg.INTENSET)
    }

    #[inline]
    pub unsafe fn WriteRaw_InterruptFlagStatusAndClear(&mut self, regVal: u32) {
        ptr::write_volatile(&mut self._reg.INTFLAG, regVal)
    }
    #[inline]
    pub unsafe fn ReadRaw_InterruptFlagStatusAndClear(&mut self) -> u32 {
        ptr::read_volatile(&mut self._reg.INTFLAG)
    }

    #[inline]
    pub unsafe fn ReadRaw_PowerAndClocksStatus(&mut self) -> u32 {
        ptr::read_volatile(&mut self._reg.STATUS)
    }

    #[inline]
    pub unsafe fn WriteRaw_ExternalMultipurposeCrystalOscillatorControl(&mut self, regVal: u16) {
        ptr::write_volatile(&mut self._reg.XOSCCTRL, regVal)
    }
    #[inline]
    pub unsafe fn ReadRaw_ExternalMultipurposeCrystalOscillatorControl(&mut self) -> u16 {
        ptr::read_volatile(&mut self._reg.XOSCCTRL)
    }

    #[inline]
    pub unsafe fn WriteRaw_ClockFailureDetectorPrescaler(&mut self, regVal: u8) {
        ptr::write_volatile(&mut self._reg.CFDPRESC, regVal)
    }
    #[inline]
    pub unsafe fn ReadRaw_ClockFailureDetectorPrescaler(&mut self) -> u8 {
        ptr::read_volatile(&mut self._reg.CFDPRESC)
    }

    #[inline]
    pub unsafe fn WriteRaw_EventControl(&mut self, regVal: u8) {
        ptr::write_volatile(&mut self._reg.EVCTRL, regVal)
    }
    #[inline]
    pub unsafe fn ReadRaw_EventControl(&mut self) -> u8 {
        ptr::read_volatile(&mut self._reg.EVCTRL)
    }

    #[inline]
    pub unsafe fn WriteRaw_48mhzInternalOscillatorControl(&mut self, regVal: u8) {
        ptr::write_volatile(&mut self._reg.OSC48MCTRL, regVal)
    }
    #[inline]
    pub unsafe fn ReadRaw_48mhzInternalOscillatorControl(&mut self) -> u8 {
        ptr::read_volatile(&mut self._reg.OSC48MCTRL)
    }

    #[inline]
    pub unsafe fn WriteRaw_Osc48mDivider(&mut self, regVal: u8) {
        ptr::write_volatile(&mut self._reg.OSC48MDIV, regVal)
    }
    #[inline]
    pub unsafe fn ReadRaw_Osc48mDivider(&mut self) -> u8 {
        ptr::read_volatile(&mut self._reg.OSC48MDIV)
    }

    #[inline]
    pub unsafe fn WriteRaw_Osc48mStartupTime(&mut self, regVal: u8) {
        ptr::write_volatile(&mut self._reg.OSC48MSTUP, regVal)
    }
    #[inline]
    pub unsafe fn ReadRaw_Osc48mStartupTime(&mut self) -> u8 {
        ptr::read_volatile(&mut self._reg.OSC48MSTUP)
    }

    #[inline]
    pub unsafe fn ReadRaw_Osc48mSynchronizationBusy(&mut self) -> u32 {
        ptr::read_volatile(&mut self._reg.OSC48MSYNCBUSY)
    }

    #[inline]
    pub unsafe fn WriteRaw_DpllControl(&mut self, regVal: u8) {
        ptr::write_volatile(&mut self._reg.DPLLCTRLA, regVal)
    }
    #[inline]
    pub unsafe fn ReadRaw_DpllControl(&mut self) -> u8 {
        ptr::read_volatile(&mut self._reg.DPLLCTRLA)
    }

    #[inline]
    pub unsafe fn WriteRaw_DpllRatioControl(&mut self, regVal: u32) {
        ptr::write_volatile(&mut self._reg.DPLLRATIO, regVal)
    }
    #[inline]
    pub unsafe fn ReadRaw_DpllRatioControl(&mut self) -> u32 {
        ptr::read_volatile(&mut self._reg.DPLLRATIO)
    }

    #[inline]
    pub unsafe fn WriteRaw_DigitalCoreConfiguration(&mut self, regVal: u32) {
        ptr::write_volatile(&mut self._reg.DPLLCTRLB, regVal)
    }
    #[inline]
    pub unsafe fn ReadRaw_DigitalCoreConfiguration(&mut self) -> u32 {
        ptr::read_volatile(&mut self._reg.DPLLCTRLB)
    }

    #[inline]
    pub unsafe fn WriteRaw_DpllPrescaler(&mut self, regVal: u8) {
        ptr::write_volatile(&mut self._reg.DPLLPRESC, regVal)
    }
    #[inline]
    pub unsafe fn ReadRaw_DpllPrescaler(&mut self) -> u8 {
        ptr::read_volatile(&mut self._reg.DPLLPRESC)
    }

    #[inline]
    pub unsafe fn ReadRaw_DpllSynchronizationBusy(&mut self) -> u8 {
        ptr::read_volatile(&mut self._reg.DPLLSYNCBUSY)
    }

    #[inline]
    pub unsafe fn ReadRaw_DpllStatus(&mut self) -> u8 {
        ptr::read_volatile(&mut self._reg.DPLLSTATUS)
    }

    #[inline]
    pub unsafe fn WriteRaw_48mhzOscillatorCalibration(&mut self, regVal: u32) {
        ptr::write_volatile(&mut self._reg.CAL48M, regVal)
    }
    #[inline]
    pub unsafe fn ReadRaw_48mhzOscillatorCalibration(&mut self) -> u32 {
        ptr::read_volatile(&mut self._reg.CAL48M)
    }

    // --------------------------------------------------


    // --------------------------------------------------
    // Typed Register Access
    // --------------------------------------------------
    #[inline]
    pub unsafe fn Write_InterruptEnableClear(&mut self, regVal: INTENCLR) {
        let mut rawVal: u32 = 0;
        rawVal |= (((regVal.XOSCRDY as u32) << 0) & 0x1) as u32; 
        rawVal |= (((regVal.XOSCFAIL as u32) << 1) & 0x2) as u32; 
        rawVal |= (((regVal.OSC48MRDY as u32) << 4) & 0x10) as u32; 
        rawVal |= (((regVal.DPLLLCKR as u32) << 8) & 0x100) as u32; 
        rawVal |= (((regVal.DPLLLCKF as u32) << 9) & 0x200) as u32; 
        rawVal |= (((regVal.DPLLLTO as u32) << 10) & 0x400) as u32; 
        rawVal |= (((regVal.DPLLLDRTO as u32) << 11) & 0x800) as u32; 
        ptr::write_volatile(&mut self._reg.INTENCLR, rawVal)
    }

    #[inline]
    pub unsafe fn Write_InterruptEnableSet(&mut self, regVal: INTENSET) {
        let mut rawVal: u32 = 0;
        rawVal |= (((regVal.XOSCRDY as u32) << 0) & 0x1) as u32; 
        rawVal |= (((regVal.XOSCFAIL as u32) << 1) & 0x2) as u32; 
        rawVal |= (((regVal.OSC48MRDY as u32) << 4) & 0x10) as u32; 
        rawVal |= (((regVal.DPLLLCKR as u32) << 8) & 0x100) as u32; 
        rawVal |= (((regVal.DPLLLCKF as u32) << 9) & 0x200) as u32; 
        rawVal |= (((regVal.DPLLLTO as u32) << 10) & 0x400) as u32; 
        rawVal |= (((regVal.DPLLLDRTO as u32) << 11) & 0x800) as u32; 
        ptr::write_volatile(&mut self._reg.INTENSET, rawVal)
    }

    #[inline]
    pub unsafe fn Write_InterruptFlagStatusAndClear(&mut self, regVal: INTFLAG) {
        let mut rawVal: u32 = 0;
        rawVal |= (((regVal.XOSCRDY as u32) << 0) & 0x1) as u32; 
        rawVal |= (((regVal.XOSCFAIL as u32) << 1) & 0x2) as u32; 
        rawVal |= (((regVal.OSC48MRDY as u32) << 4) & 0x10) as u32; 
        rawVal |= (((regVal.DPLLLCKR as u32) << 8) & 0x100) as u32; 
        rawVal |= (((regVal.DPLLLCKF as u32) << 9) & 0x200) as u32; 
        rawVal |= (((regVal.DPLLLTO as u32) << 10) & 0x400) as u32; 
        rawVal |= (((regVal.DPLLLDRTO as u32) << 11) & 0x800) as u32; 
        ptr::write_volatile(&mut self._reg.INTFLAG, rawVal)
    }


    #[inline]
    pub unsafe fn Write_ExternalMultipurposeCrystalOscillatorControl(&mut self, regVal: XOSCCTRL) {
        let mut rawVal: u16 = 0;
        rawVal |= (((regVal.ENABLE as u32) << 1) & 0x2) as u16; 
        rawVal |= (((regVal.XTALEN as u32) << 2) & 0x4) as u16; 
        rawVal |= (((regVal.CFDEN as u32) << 3) & 0x8) as u16; 
        rawVal |= (((regVal.SWBEN as u32) << 4) & 0x10) as u16; 
        rawVal |= (((regVal.RUNSTDBY as u32) << 6) & 0x40) as u16; 
        rawVal |= (((regVal.ONDEMAND as u32) << 7) & 0x80) as u16; 
        rawVal |= (((regVal.GAIN as u32) << 8) & 0x700) as u16; 
        rawVal |= (((regVal.AMPGC as u32) << 11) & 0x800) as u16; 
        rawVal |= (((regVal.STARTUP as u32) << 12) & 0xF000) as u16; 
        ptr::write_volatile(&mut self._reg.XOSCCTRL, rawVal)
    }

    #[inline]
    pub unsafe fn Write_48mhzInternalOscillatorControl(&mut self, regVal: OSC48MCTRL) {
        let mut rawVal: u8 = 0;
        rawVal |= (((regVal.ENABLE as u32) << 1) & 0x2) as u8; 
        rawVal |= (((regVal.RUNSTDBY as u32) << 6) & 0x40) as u8; 
        rawVal |= (((regVal.ONDEMAND as u32) << 7) & 0x80) as u8; 
        ptr::write_volatile(&mut self._reg.OSC48MCTRL, rawVal)
    }

    #[inline]
    pub unsafe fn Write_DpllControl(&mut self, regVal: DPLLCTRLA) {
        let mut rawVal: u8 = 0;
        rawVal |= (((regVal.ENABLE as u32) << 1) & 0x2) as u8; 
        rawVal |= (((regVal.RUNSTDBY as u32) << 6) & 0x40) as u8; 
        rawVal |= (((regVal.ONDEMAND as u32) << 7) & 0x80) as u8; 
        ptr::write_volatile(&mut self._reg.DPLLCTRLA, rawVal)
    }

    #[inline]
    pub unsafe fn Write_DpllRatioControl(&mut self, regVal: DPLLRATIO) {
        let mut rawVal: u32 = 0;
        rawVal |= (((regVal.LDR as u32) << 0) & 0xFFF) as u32; 
        rawVal |= (((regVal.LDRFRAC as u32) << 16) & 0xF0000) as u32; 
        ptr::write_volatile(&mut self._reg.DPLLRATIO, rawVal)
    }

    #[inline]
    pub unsafe fn Write_DigitalCoreConfiguration(&mut self, regVal: DPLLCTRLB) {
        let mut rawVal: u32 = 0;
        rawVal |= (((regVal.FILTER as u32) << 0) & 0x3) as u32; 
        rawVal |= (((regVal.LPEN as u32) << 2) & 0x4) as u32; 
        rawVal |= (((regVal.WUF as u32) << 3) & 0x8) as u32; 
        rawVal |= (((regVal.REFCLK as u32) << 4) & 0x30) as u32; 
        rawVal |= (((regVal.LTIME as u32) << 8) & 0x700) as u32; 
        rawVal |= (((regVal.LBYPASS as u32) << 12) & 0x1000) as u32; 
        rawVal |= (((regVal.DIV as u32) << 16) & 0x7FF0000) as u32; 
        ptr::write_volatile(&mut self._reg.DPLLCTRLB, rawVal)
    }



    #[inline]
    pub unsafe fn Write_48mhzOscillatorCalibration(&mut self, regVal: CAL48M) {
        let mut rawVal: u32 = 0;
        rawVal |= (((regVal.FCAL as u32) << 0) & 0x3F) as u32; 
        rawVal |= (((regVal.FRANGE as u32) << 8) & 0x300) as u32; 
        rawVal |= (((regVal.TCAL as u32) << 16) & 0x3F0000) as u32; 
        ptr::write_volatile(&mut self._reg.CAL48M, rawVal)
    }

    // --------------------------------------------------


    // --------------------------------------------------
    // Register BitField Access
    // --------------------------------------------------
    #[inline]
    pub unsafe fn Set_INTENCLR_XOSCRDY(&mut self, val: u32) {
        let regVal = ptr::read_volatile(&self._reg.INTENCLR) & !(0x1);
        ptr::write_volatile(&mut self._reg.INTENCLR, ((val as u32) << 0) | regVal)
    }
    #[inline]
    pub unsafe fn Set_INTENCLR_XOSCFAIL(&mut self, val: u32) {
        let regVal = ptr::read_volatile(&self._reg.INTENCLR) & !(0x2);
        ptr::write_volatile(&mut self._reg.INTENCLR, ((val as u32) << 1) | regVal)
    }
    #[inline]
    pub unsafe fn Set_INTENCLR_OSC48MRDY(&mut self, val: u32) {
        let regVal = ptr::read_volatile(&self._reg.INTENCLR) & !(0x10);
        ptr::write_volatile(&mut self._reg.INTENCLR, ((val as u32) << 4) | regVal)
    }
    #[inline]
    pub unsafe fn Set_INTENCLR_DPLLLCKR(&mut self, val: u32) {
        let regVal = ptr::read_volatile(&self._reg.INTENCLR) & !(0x100);
        ptr::write_volatile(&mut self._reg.INTENCLR, ((val as u32) << 8) | regVal)
    }
    #[inline]
    pub unsafe fn Set_INTENCLR_DPLLLCKF(&mut self, val: u32) {
        let regVal = ptr::read_volatile(&self._reg.INTENCLR) & !(0x200);
        ptr::write_volatile(&mut self._reg.INTENCLR, ((val as u32) << 9) | regVal)
    }
    #[inline]
    pub unsafe fn Set_INTENCLR_DPLLLTO(&mut self, val: u32) {
        let regVal = ptr::read_volatile(&self._reg.INTENCLR) & !(0x400);
        ptr::write_volatile(&mut self._reg.INTENCLR, ((val as u32) << 10) | regVal)
    }
    #[inline]
    pub unsafe fn Set_INTENCLR_DPLLLDRTO(&mut self, val: u32) {
        let regVal = ptr::read_volatile(&self._reg.INTENCLR) & !(0x800);
        ptr::write_volatile(&mut self._reg.INTENCLR, ((val as u32) << 11) | regVal)
    }

    #[inline]
    pub unsafe fn Set_INTENSET_XOSCRDY(&mut self, val: u32) {
        let regVal = ptr::read_volatile(&self._reg.INTENSET) & !(0x1);
        ptr::write_volatile(&mut self._reg.INTENSET, ((val as u32) << 0) | regVal)
    }
    #[inline]
    pub unsafe fn Set_INTENSET_XOSCFAIL(&mut self, val: u32) {
        let regVal = ptr::read_volatile(&self._reg.INTENSET) & !(0x2);
        ptr::write_volatile(&mut self._reg.INTENSET, ((val as u32) << 1) | regVal)
    }
    #[inline]
    pub unsafe fn Set_INTENSET_OSC48MRDY(&mut self, val: u32) {
        let regVal = ptr::read_volatile(&self._reg.INTENSET) & !(0x10);
        ptr::write_volatile(&mut self._reg.INTENSET, ((val as u32) << 4) | regVal)
    }
    #[inline]
    pub unsafe fn Set_INTENSET_DPLLLCKR(&mut self, val: u32) {
        let regVal = ptr::read_volatile(&self._reg.INTENSET) & !(0x100);
        ptr::write_volatile(&mut self._reg.INTENSET, ((val as u32) << 8) | regVal)
    }
    #[inline]
    pub unsafe fn Set_INTENSET_DPLLLCKF(&mut self, val: u32) {
        let regVal = ptr::read_volatile(&self._reg.INTENSET) & !(0x200);
        ptr::write_volatile(&mut self._reg.INTENSET, ((val as u32) << 9) | regVal)
    }
    #[inline]
    pub unsafe fn Set_INTENSET_DPLLLTO(&mut self, val: u32) {
        let regVal = ptr::read_volatile(&self._reg.INTENSET) & !(0x400);
        ptr::write_volatile(&mut self._reg.INTENSET, ((val as u32) << 10) | regVal)
    }
    #[inline]
    pub unsafe fn Set_INTENSET_DPLLLDRTO(&mut self, val: u32) {
        let regVal = ptr::read_volatile(&self._reg.INTENSET) & !(0x800);
        ptr::write_volatile(&mut self._reg.INTENSET, ((val as u32) << 11) | regVal)
    }

    #[inline]
    pub unsafe fn Set_INTFLAG_XOSCRDY(&mut self, val: u32) {
        let regVal = ptr::read_volatile(&self._reg.INTFLAG) & !(0x1);
        ptr::write_volatile(&mut self._reg.INTFLAG, ((val as u32) << 0) | regVal)
    }
    #[inline]
    pub unsafe fn Set_INTFLAG_XOSCFAIL(&mut self, val: u32) {
        let regVal = ptr::read_volatile(&self._reg.INTFLAG) & !(0x2);
        ptr::write_volatile(&mut self._reg.INTFLAG, ((val as u32) << 1) | regVal)
    }
    #[inline]
    pub unsafe fn Set_INTFLAG_OSC48MRDY(&mut self, val: u32) {
        let regVal = ptr::read_volatile(&self._reg.INTFLAG) & !(0x10);
        ptr::write_volatile(&mut self._reg.INTFLAG, ((val as u32) << 4) | regVal)
    }
    #[inline]
    pub unsafe fn Set_INTFLAG_DPLLLCKR(&mut self, val: u32) {
        let regVal = ptr::read_volatile(&self._reg.INTFLAG) & !(0x100);
        ptr::write_volatile(&mut self._reg.INTFLAG, ((val as u32) << 8) | regVal)
    }
    #[inline]
    pub unsafe fn Set_INTFLAG_DPLLLCKF(&mut self, val: u32) {
        let regVal = ptr::read_volatile(&self._reg.INTFLAG) & !(0x200);
        ptr::write_volatile(&mut self._reg.INTFLAG, ((val as u32) << 9) | regVal)
    }
    #[inline]
    pub unsafe fn Set_INTFLAG_DPLLLTO(&mut self, val: u32) {
        let regVal = ptr::read_volatile(&self._reg.INTFLAG) & !(0x400);
        ptr::write_volatile(&mut self._reg.INTFLAG, ((val as u32) << 10) | regVal)
    }
    #[inline]
    pub unsafe fn Set_INTFLAG_DPLLLDRTO(&mut self, val: u32) {
        let regVal = ptr::read_volatile(&self._reg.INTFLAG) & !(0x800);
        ptr::write_volatile(&mut self._reg.INTFLAG, ((val as u32) << 11) | regVal)
    }


    #[inline]
    pub unsafe fn Set_XOSCCTRL_ENABLE(&mut self, val: u16) {
        let regVal = ptr::read_volatile(&self._reg.XOSCCTRL) & !(0x2);
        ptr::write_volatile(&mut self._reg.XOSCCTRL, ((val as u16) << 1) | regVal)
    }
    #[inline]
    pub unsafe fn Set_XOSCCTRL_XTALEN(&mut self, val: u16) {
        let regVal = ptr::read_volatile(&self._reg.XOSCCTRL) & !(0x4);
        ptr::write_volatile(&mut self._reg.XOSCCTRL, ((val as u16) << 2) | regVal)
    }
    #[inline]
    pub unsafe fn Set_XOSCCTRL_CFDEN(&mut self, val: u16) {
        let regVal = ptr::read_volatile(&self._reg.XOSCCTRL) & !(0x8);
        ptr::write_volatile(&mut self._reg.XOSCCTRL, ((val as u16) << 3) | regVal)
    }
    #[inline]
    pub unsafe fn Set_XOSCCTRL_SWBEN(&mut self, val: u16) {
        let regVal = ptr::read_volatile(&self._reg.XOSCCTRL) & !(0x10);
        ptr::write_volatile(&mut self._reg.XOSCCTRL, ((val as u16) << 4) | regVal)
    }
    #[inline]
    pub unsafe fn Set_XOSCCTRL_RUNSTDBY(&mut self, val: u16) {
        let regVal = ptr::read_volatile(&self._reg.XOSCCTRL) & !(0x40);
        ptr::write_volatile(&mut self._reg.XOSCCTRL, ((val as u16) << 6) | regVal)
    }
    #[inline]
    pub unsafe fn Set_XOSCCTRL_ONDEMAND(&mut self, val: u16) {
        let regVal = ptr::read_volatile(&self._reg.XOSCCTRL) & !(0x80);
        ptr::write_volatile(&mut self._reg.XOSCCTRL, ((val as u16) << 7) | regVal)
    }
    #[inline]
    pub unsafe fn Set_XOSCCTRL_GAIN(&mut self, val: OSCCTRL_XOSCCTRL__GAIN) {
        let regVal = ptr::read_volatile(&self._reg.XOSCCTRL) & !(0x700);
        ptr::write_volatile(&mut self._reg.XOSCCTRL, ((val as u16) << 8) | regVal)
    }
    #[inline]
    pub unsafe fn Set_XOSCCTRL_AMPGC(&mut self, val: u16) {
        let regVal = ptr::read_volatile(&self._reg.XOSCCTRL) & !(0x800);
        ptr::write_volatile(&mut self._reg.XOSCCTRL, ((val as u16) << 11) | regVal)
    }
    #[inline]
    pub unsafe fn Set_XOSCCTRL_STARTUP(&mut self, val: OSCCTRL_XOSCCTRL__STARTUP) {
        let regVal = ptr::read_volatile(&self._reg.XOSCCTRL) & !(0xF000);
        ptr::write_volatile(&mut self._reg.XOSCCTRL, ((val as u16) << 12) | regVal)
    }

    #[inline]
    pub unsafe fn Set_CFDPRESC_CFDPRESC(&mut self, val: OSCCTRL_CFDPRESC__CFDPRESC) {
        let regVal = ptr::read_volatile(&self._reg.CFDPRESC) & !(0x7);
        ptr::write_volatile(&mut self._reg.CFDPRESC, ((val as u8) << 0) | regVal)
    }

    #[inline]
    pub unsafe fn Set_EVCTRL_CFDEO(&mut self, val: u8) {
        let regVal = ptr::read_volatile(&self._reg.EVCTRL) & !(0x1);
        ptr::write_volatile(&mut self._reg.EVCTRL, ((val as u8) << 0) | regVal)
    }

    #[inline]
    pub unsafe fn Set_OSC48MCTRL_ENABLE(&mut self, val: u8) {
        let regVal = ptr::read_volatile(&self._reg.OSC48MCTRL) & !(0x2);
        ptr::write_volatile(&mut self._reg.OSC48MCTRL, ((val as u8) << 1) | regVal)
    }
    #[inline]
    pub unsafe fn Set_OSC48MCTRL_RUNSTDBY(&mut self, val: u8) {
        let regVal = ptr::read_volatile(&self._reg.OSC48MCTRL) & !(0x40);
        ptr::write_volatile(&mut self._reg.OSC48MCTRL, ((val as u8) << 6) | regVal)
    }
    #[inline]
    pub unsafe fn Set_OSC48MCTRL_ONDEMAND(&mut self, val: u8) {
        let regVal = ptr::read_volatile(&self._reg.OSC48MCTRL) & !(0x80);
        ptr::write_volatile(&mut self._reg.OSC48MCTRL, ((val as u8) << 7) | regVal)
    }

    #[inline]
    pub unsafe fn Set_OSC48MDIV_DIV(&mut self, val: OSCCTRL_OSC48MDIV__DIV) {
        let regVal = ptr::read_volatile(&self._reg.OSC48MDIV) & !(0xF);
        ptr::write_volatile(&mut self._reg.OSC48MDIV, ((val as u8) << 0) | regVal)
    }

    #[inline]
    pub unsafe fn Set_OSC48MSTUP_STARTUP(&mut self, val: OSCCTRL_OSC48MSTUP__STARTUP) {
        let regVal = ptr::read_volatile(&self._reg.OSC48MSTUP) & !(0x7);
        ptr::write_volatile(&mut self._reg.OSC48MSTUP, ((val as u8) << 0) | regVal)
    }


    #[inline]
    pub unsafe fn Set_DPLLCTRLA_ENABLE(&mut self, val: u8) {
        let regVal = ptr::read_volatile(&self._reg.DPLLCTRLA) & !(0x2);
        ptr::write_volatile(&mut self._reg.DPLLCTRLA, ((val as u8) << 1) | regVal)
    }
    #[inline]
    pub unsafe fn Set_DPLLCTRLA_RUNSTDBY(&mut self, val: u8) {
        let regVal = ptr::read_volatile(&self._reg.DPLLCTRLA) & !(0x40);
        ptr::write_volatile(&mut self._reg.DPLLCTRLA, ((val as u8) << 6) | regVal)
    }
    #[inline]
    pub unsafe fn Set_DPLLCTRLA_ONDEMAND(&mut self, val: u8) {
        let regVal = ptr::read_volatile(&self._reg.DPLLCTRLA) & !(0x80);
        ptr::write_volatile(&mut self._reg.DPLLCTRLA, ((val as u8) << 7) | regVal)
    }

    #[inline]
    pub unsafe fn Set_DPLLRATIO_LDR(&mut self, val: u32) {
        let regVal = ptr::read_volatile(&self._reg.DPLLRATIO) & !(0xFFF);
        ptr::write_volatile(&mut self._reg.DPLLRATIO, ((val as u32) << 0) | regVal)
    }
    #[inline]
    pub unsafe fn Set_DPLLRATIO_LDRFRAC(&mut self, val: u32) {
        let regVal = ptr::read_volatile(&self._reg.DPLLRATIO) & !(0xF0000);
        ptr::write_volatile(&mut self._reg.DPLLRATIO, ((val as u32) << 16) | regVal)
    }

    #[inline]
    pub unsafe fn Set_DPLLCTRLB_FILTER(&mut self, val: OSCCTRL_DPLLCTRLB__FILTER) {
        let regVal = ptr::read_volatile(&self._reg.DPLLCTRLB) & !(0x3);
        ptr::write_volatile(&mut self._reg.DPLLCTRLB, ((val as u32) << 0) | regVal)
    }
    #[inline]
    pub unsafe fn Set_DPLLCTRLB_LPEN(&mut self, val: u32) {
        let regVal = ptr::read_volatile(&self._reg.DPLLCTRLB) & !(0x4);
        ptr::write_volatile(&mut self._reg.DPLLCTRLB, ((val as u32) << 2) | regVal)
    }
    #[inline]
    pub unsafe fn Set_DPLLCTRLB_WUF(&mut self, val: u32) {
        let regVal = ptr::read_volatile(&self._reg.DPLLCTRLB) & !(0x8);
        ptr::write_volatile(&mut self._reg.DPLLCTRLB, ((val as u32) << 3) | regVal)
    }
    #[inline]
    pub unsafe fn Set_DPLLCTRLB_REFCLK(&mut self, val: OSCCTRL_DPLLCTRLB__REFCLK) {
        let regVal = ptr::read_volatile(&self._reg.DPLLCTRLB) & !(0x30);
        ptr::write_volatile(&mut self._reg.DPLLCTRLB, ((val as u32) << 4) | regVal)
    }
    #[inline]
    pub unsafe fn Set_DPLLCTRLB_LTIME(&mut self, val: OSCCTRL_DPLLCTRLB__LTIME) {
        let regVal = ptr::read_volatile(&self._reg.DPLLCTRLB) & !(0x700);
        ptr::write_volatile(&mut self._reg.DPLLCTRLB, ((val as u32) << 8) | regVal)
    }
    #[inline]
    pub unsafe fn Set_DPLLCTRLB_LBYPASS(&mut self, val: u32) {
        let regVal = ptr::read_volatile(&self._reg.DPLLCTRLB) & !(0x1000);
        ptr::write_volatile(&mut self._reg.DPLLCTRLB, ((val as u32) << 12) | regVal)
    }
    #[inline]
    pub unsafe fn Set_DPLLCTRLB_DIV(&mut self, val: u32) {
        let regVal = ptr::read_volatile(&self._reg.DPLLCTRLB) & !(0x7FF0000);
        ptr::write_volatile(&mut self._reg.DPLLCTRLB, ((val as u32) << 16) | regVal)
    }

    #[inline]
    pub unsafe fn Set_DPLLPRESC_PRESC(&mut self, val: OSCCTRL_DPLLPRESC__PRESC) {
        let regVal = ptr::read_volatile(&self._reg.DPLLPRESC) & !(0x3);
        ptr::write_volatile(&mut self._reg.DPLLPRESC, ((val as u8) << 0) | regVal)
    }



    #[inline]
    pub unsafe fn Set_CAL48M_FCAL(&mut self, val: u32) {
        let regVal = ptr::read_volatile(&self._reg.CAL48M) & !(0x3F);
        ptr::write_volatile(&mut self._reg.CAL48M, ((val as u32) << 0) | regVal)
    }
    #[inline]
    pub unsafe fn Set_CAL48M_FRANGE(&mut self, val: u32) {
        let regVal = ptr::read_volatile(&self._reg.CAL48M) & !(0x300);
        ptr::write_volatile(&mut self._reg.CAL48M, ((val as u32) << 8) | regVal)
    }
    #[inline]
    pub unsafe fn Set_CAL48M_TCAL(&mut self, val: u32) {
        let regVal = ptr::read_volatile(&self._reg.CAL48M) & !(0x3F0000);
        ptr::write_volatile(&mut self._reg.CAL48M, ((val as u32) << 16) | regVal)
    }

    // --------------------------------------------------


}

// --------------------------------------------------
// Register Type Definition
// --------------------------------------------------
pub struct INTENCLR {
    pub XOSCRDY: u8,
    pub XOSCFAIL: u8,
    pub OSC48MRDY: u8,
    pub DPLLLCKR: u8,
    pub DPLLLCKF: u8,
    pub DPLLLTO: u8,
    pub DPLLLDRTO: u8,
}

pub struct INTENSET {
    pub XOSCRDY: u8,
    pub XOSCFAIL: u8,
    pub OSC48MRDY: u8,
    pub DPLLLCKR: u8,
    pub DPLLLCKF: u8,
    pub DPLLLTO: u8,
    pub DPLLLDRTO: u8,
}

pub struct INTFLAG {
    pub XOSCRDY: u8,
    pub XOSCFAIL: u8,
    pub OSC48MRDY: u8,
    pub DPLLLCKR: u8,
    pub DPLLLCKF: u8,
    pub DPLLLTO: u8,
    pub DPLLLDRTO: u8,
}

pub struct STATUS {
    pub XOSCRDY: u8,
    pub XOSCFAIL: u8,
    pub XOSCCKSW: u8,
    pub OSC48MRDY: u8,
    pub DPLLLCKR: u8,
    pub DPLLLCKF: u8,
    pub DPLLTO: u8,
    pub DPLLLDRTO: u8,
}

pub struct XOSCCTRL {
    pub ENABLE: u8,
    pub XTALEN: u8,
    pub CFDEN: u8,
    pub SWBEN: u8,
    pub RUNSTDBY: u8,
    pub ONDEMAND: u8,
    pub GAIN: OSCCTRL_XOSCCTRL__GAIN,
    pub AMPGC: u8,
    pub STARTUP: OSCCTRL_XOSCCTRL__STARTUP,
}

pub struct OSC48MCTRL {
    pub ENABLE: u8,
    pub RUNSTDBY: u8,
    pub ONDEMAND: u8,
}

pub struct DPLLCTRLA {
    pub ENABLE: u8,
    pub RUNSTDBY: u8,
    pub ONDEMAND: u8,
}

pub struct DPLLRATIO {
    pub LDR: u16,
    pub LDRFRAC: u8,
}

pub struct DPLLCTRLB {
    pub FILTER: OSCCTRL_DPLLCTRLB__FILTER,
    pub LPEN: u8,
    pub WUF: u8,
    pub REFCLK: OSCCTRL_DPLLCTRLB__REFCLK,
    pub LTIME: OSCCTRL_DPLLCTRLB__LTIME,
    pub LBYPASS: u8,
    pub DIV: u16,
}

pub struct DPLLSYNCBUSY {
    pub ENABLE: u8,
    pub DPLLRATIO: u8,
    pub DPLLPRESC: u8,
}

pub struct DPLLSTATUS {
    pub LOCK: u8,
    pub CLKRDY: u8,
}

pub struct CAL48M {
    pub FCAL: u8,
    pub FRANGE: u8,
    pub TCAL: u8,
}

// --------------------------------------------------


pub enum OSCCTRL_XOSCCTRL__STARTUP {
    CYCLE1 = 0,                                     //31 us
    CYCLE2 = 1,                                     //61 us
    CYCLE4 = 2,                                     //122 us
    CYCLE8 = 3,                                     //244 us
    CYCLE16 = 4,                                    //488 us
    CYCLE32 = 5,                                    //977 us
    CYCLE64 = 6,                                    //1953 us
    CYCLE128 = 7,                                   //3906 us
    CYCLE256 = 8,                                   //7813 us
    CYCLE512 = 9,                                   //15625 us
    CYCLE1024 = 10,                                 //31250 us
    CYCLE2048 = 11,                                 //62500 us
    CYCLE4096 = 12,                                 //125000 us
    CYCLE8192 = 13,                                 //250000 us
    CYCLE16384 = 14,                                //500000 us
    CYCLE32768 = 15,                                //1000000 us
}

pub enum OSCCTRL_XOSCCTRL__GAIN {
    GAIN2 = 0,                                      //2 MHz
    GAIN4 = 1,                                      //4 MHz
    GAIN8 = 2,                                      //8 MHz
    GAIN16 = 3,                                     //16 MHz
    GAIN30 = 4,                                     //30 MHz
}

pub enum OSCCTRL_CFDPRESC__CFDPRESC {
    DIV1 = 0,                                       //48 MHz
    DIV2 = 1,                                       //24 MHz
    DIV4 = 2,                                       //12 MHz
    DIV8 = 3,                                       //6 MHz
    DIV16 = 4,                                      //3 MHz
    DIV32 = 5,                                      //1.5 MHz
    DIV64 = 6,                                      //0.75 MHz
    DIV128 = 7,                                     //0.3125 MHz
}

pub enum OSCCTRL_OSC48MDIV__DIV {
    DIV1 = 0,                                       //48 MHz
    DIV2 = 1,                                       //24 MHz
    DIV3 = 2,                                       //16 MHz
    DIV4 = 3,                                       //12 MHz
    DIV5 = 4,                                       //9.6 MHz
    DIV6 = 5,                                       //8 MHz
    DIV7 = 6,                                       //6.86 MHz
    DIV8 = 7,                                       //6 MHz
    DIV9 = 8,                                       //5.33 MHz
    DIV10 = 9,                                      //4.8 MHz
    DIV11 = 10,                                     //4.36 MHz
    DIV12 = 11,                                     //4 MHz
    DIV13 = 12,                                     //3.69 MHz
    DIV14 = 13,                                     //3.43 MHz
    DIV15 = 14,                                     //3.2 MHz
    DIV16 = 15,                                     //3 MHz
}

pub enum OSCCTRL_OSC48MSTUP__STARTUP {
    CYCLE8 = 0,                                     //166 ns
    CYCLE16 = 1,                                    //333 ns
    CYCLE32 = 2,                                    //667 ns
    CYCLE64 = 3,                                    //1.333 us
    CYCLE128 = 4,                                   //2.667 us
    CYCLE256 = 5,                                   //5.333 us
    CYCLE512 = 6,                                   //10.667 us
    CYCLE1024 = 7,                                  //21.333 us
}

pub enum OSCCTRL_DPLLCTRLB__LTIME {
    DEFAULT = 0,                                    //No time-out. Automatic lock.
    _8MS = 4,                                       //Time-out if no lock within 8ms
    _9MS = 5,                                       //Time-out if no lock within 9ms
    _10MS = 6,                                      //Time-out if no lock within 10ms
    _11MS = 7,                                      //Time-out if no lock within 11ms
}

pub enum OSCCTRL_DPLLCTRLB__FILTER {
    DEFAULT = 0,                                    //Default filter mode
    LBFILT = 1,                                     //Low bandwidth filter
    HBFILT = 2,                                     //High bandwidth filter
    HDFILT = 3,                                     //High damping filter
}

pub enum OSCCTRL_DPLLCTRLB__REFCLK {
    XOSC32K = 0,                                    //XOSC32K clock reference
    XOSC = 1,                                       //XOSC clock reference
    GCLK = 2,                                       //GCLK clock reference
}

pub enum OSCCTRL_DPLLPRESC__PRESC {
    DIV1 = 0,                                       //DPLL output is divided by 1
    DIV2 = 1,                                       //DPLL output is divided by 2
    DIV4 = 2,                                       //DPLL output is divided by 4
}

