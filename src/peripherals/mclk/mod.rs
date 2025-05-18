#![allow(non_camel_case_types)]

use core::{cell::UnsafeCell, ptr};

use crate::cortex::{CriticalSection, Mutex};

pub(crate) static MCLK: Mutex<UnsafeCell<Option<MainClock>>> = Mutex::new(UnsafeCell::new(None));

#[repr(C)]
pub struct RegisterGroup {
    pub res0: u8,
    // Interrupt Enable Clear
    pub INTENCLR: u8,
    // Interrupt Enable Set
    pub INTENSET: u8,
    // Interrupt Flag Status and Clear
    pub INTFLAG: u8,
    // CPU Clock Division
    pub CPUDIV: u8,
    pub res1: [u8; 11],
    // AHB Mask
    pub AHBMASK: u32,
    // APBA Mask
    pub APBAMASK: u32,
    // APBB Mask
    pub APBBMASK: u32,
    // APBC Mask
    pub APBCMASK: u32,
}

pub struct MainClock {
    _reg: &'static mut RegisterGroup,
}

impl MainClock {
    #[inline]
    pub fn new() -> Option<Self> {
        let mut result: bool = true;
        unsafe {
            result = CriticalSection(|| MCLK.borrow().as_ref_unchecked().is_none());
        }

        if result {
            Some(MainClock {
                _reg: unsafe { &mut *(0x40000800 as *mut RegisterGroup) }
            })
        } else {
            None
        }
    }

    // --------------------------------------------------
    // Raw Register Access
    // --------------------------------------------------
    #[inline]
    pub unsafe fn WriteRaw_InterruptEnableClear(&mut self, regVal: u8) {
        ptr::write_volatile(&mut self._reg.INTENCLR, regVal)
    }
    #[inline]
    pub unsafe fn ReadRaw_InterruptEnableClear(&mut self) -> u8 {
        ptr::read_volatile(&mut self._reg.INTENCLR)
    }

    #[inline]
    pub unsafe fn WriteRaw_InterruptEnableSet(&mut self, regVal: u8) {
        ptr::write_volatile(&mut self._reg.INTENSET, regVal)
    }
    #[inline]
    pub unsafe fn ReadRaw_InterruptEnableSet(&mut self) -> u8 {
        ptr::read_volatile(&mut self._reg.INTENSET)
    }

    #[inline]
    pub unsafe fn WriteRaw_InterruptFlagStatusAndClear(&mut self, regVal: u8) {
        ptr::write_volatile(&mut self._reg.INTFLAG, regVal)
    }
    #[inline]
    pub unsafe fn ReadRaw_InterruptFlagStatusAndClear(&mut self) -> u8 {
        ptr::read_volatile(&mut self._reg.INTFLAG)
    }

    #[inline]
    pub unsafe fn WriteRaw_CpuClockDivision(&mut self, regVal: u8) {
        ptr::write_volatile(&mut self._reg.CPUDIV, regVal)
    }
    #[inline]
    pub unsafe fn ReadRaw_CpuClockDivision(&mut self) -> u8 {
        ptr::read_volatile(&mut self._reg.CPUDIV)
    }

    #[inline]
    pub unsafe fn WriteRaw_AhbMask(&mut self, regVal: u32) {
        ptr::write_volatile(&mut self._reg.AHBMASK, regVal)
    }
    #[inline]
    pub unsafe fn ReadRaw_AhbMask(&mut self) -> u32 {
        ptr::read_volatile(&mut self._reg.AHBMASK)
    }

    #[inline]
    pub unsafe fn WriteRaw_ApbaMask(&mut self, regVal: u32) {
        ptr::write_volatile(&mut self._reg.APBAMASK, regVal)
    }
    #[inline]
    pub unsafe fn ReadRaw_ApbaMask(&mut self) -> u32 {
        ptr::read_volatile(&mut self._reg.APBAMASK)
    }

    #[inline]
    pub unsafe fn WriteRaw_ApbbMask(&mut self, regVal: u32) {
        ptr::write_volatile(&mut self._reg.APBBMASK, regVal)
    }
    #[inline]
    pub unsafe fn ReadRaw_ApbbMask(&mut self) -> u32 {
        ptr::read_volatile(&mut self._reg.APBBMASK)
    }

    #[inline]
    pub unsafe fn WriteRaw_ApbcMask(&mut self, regVal: u32) {
        ptr::write_volatile(&mut self._reg.APBCMASK, regVal)
    }
    #[inline]
    pub unsafe fn ReadRaw_ApbcMask(&mut self) -> u32 {
        ptr::read_volatile(&mut self._reg.APBCMASK)
    }

    // --------------------------------------------------


    // --------------------------------------------------
    // Typed Register Access
    // --------------------------------------------------
    #[inline]
    pub unsafe fn Write_AhbMask(&mut self, regVal: AHBMASK) {
        let mut rawVal: u32 = 0;
        rawVal |= ((regVal.HPB0_ as u32 & 0x1) << 0) as u32; 
        rawVal |= ((regVal.HPB1_ as u32 & 0x2) << 1) as u32; 
        rawVal |= ((regVal.HPB2_ as u32 & 0x4) << 2) as u32; 
        rawVal |= ((regVal.DSU_ as u32 & 0x8) << 3) as u32; 
        rawVal |= ((regVal.HMATRIXHS_ as u32 & 0x10) << 4) as u32; 
        rawVal |= ((regVal.NVMCTRL_ as u32 & 0x20) << 5) as u32; 
        rawVal |= ((regVal.HSRAM_ as u32 & 0x40) << 6) as u32; 
        rawVal |= ((regVal.DMAC_ as u32 & 0x80) << 7) as u32; 
        rawVal |= ((regVal.CAN0_ as u32 & 0x100) << 8) as u32; 
        rawVal |= ((regVal.CAN1_ as u32 & 0x200) << 9) as u32; 
        rawVal |= ((regVal.PAC_ as u32 & 0x400) << 10) as u32; 
        rawVal |= ((regVal.NVMCTRL_PICACHU_ as u32 & 0x800) << 11) as u32; 
        rawVal |= ((regVal.DIVAS_ as u32 & 0x1000) << 12) as u32; 
        ptr::write_volatile(&mut self._reg.AHBMASK, rawVal)
    }

    #[inline]
    pub unsafe fn Write_ApbaMask(&mut self, regVal: APBAMASK) {
        let mut rawVal: u32 = 0;
        rawVal |= ((regVal.PAC_ as u32 & 0x1) << 0) as u32; 
        rawVal |= ((regVal.PM_ as u32 & 0x2) << 1) as u32; 
        rawVal |= ((regVal.MCLK_ as u32 & 0x4) << 2) as u32; 
        rawVal |= ((regVal.RSTC_ as u32 & 0x8) << 3) as u32; 
        rawVal |= ((regVal.OSCCTRL_ as u32 & 0x10) << 4) as u32; 
        rawVal |= ((regVal.OSC32KCTRL_ as u32 & 0x20) << 5) as u32; 
        rawVal |= ((regVal.SUPC_ as u32 & 0x40) << 6) as u32; 
        rawVal |= ((regVal.GCLK_ as u32 & 0x80) << 7) as u32; 
        rawVal |= ((regVal.WDT_ as u32 & 0x100) << 8) as u32; 
        rawVal |= ((regVal.RTC_ as u32 & 0x200) << 9) as u32; 
        rawVal |= ((regVal.EIC_ as u32 & 0x400) << 10) as u32; 
        rawVal |= ((regVal.FREQM_ as u32 & 0x800) << 11) as u32; 
        rawVal |= ((regVal.TSENS_ as u32 & 0x1000) << 12) as u32; 
        ptr::write_volatile(&mut self._reg.APBAMASK, rawVal)
    }

    #[inline]
    pub unsafe fn Write_ApbbMask(&mut self, regVal: APBBMASK) {
        let mut rawVal: u32 = 0;
        rawVal |= ((regVal.PORT_ as u32 & 0x1) << 0) as u32; 
        rawVal |= ((regVal.DSU_ as u32 & 0x2) << 1) as u32; 
        rawVal |= ((regVal.NVMCTRL_ as u32 & 0x4) << 2) as u32; 
        rawVal |= ((regVal.HMATRIXHS_ as u32 & 0x20) << 5) as u32; 
        ptr::write_volatile(&mut self._reg.APBBMASK, rawVal)
    }

    #[inline]
    pub unsafe fn Write_ApbcMask(&mut self, regVal: APBCMASK) {
        let mut rawVal: u32 = 0;
        rawVal |= ((regVal.EVSYS_ as u32 & 0x1) << 0) as u32; 
        rawVal |= ((regVal.SERCOM0_ as u32 & 0x2) << 1) as u32; 
        rawVal |= ((regVal.SERCOM1_ as u32 & 0x4) << 2) as u32; 
        rawVal |= ((regVal.SERCOM2_ as u32 & 0x8) << 3) as u32; 
        rawVal |= ((regVal.SERCOM3_ as u32 & 0x10) << 4) as u32; 
        rawVal |= ((regVal.SERCOM4_ as u32 & 0x20) << 5) as u32; 
        rawVal |= ((regVal.SERCOM5_ as u32 & 0x40) << 6) as u32; 
        rawVal |= ((regVal.TCC0_ as u32 & 0x200) << 9) as u32; 
        rawVal |= ((regVal.TCC1_ as u32 & 0x400) << 10) as u32; 
        rawVal |= ((regVal.TCC2_ as u32 & 0x800) << 11) as u32; 
        rawVal |= ((regVal.TC0_ as u32 & 0x1000) << 12) as u32; 
        rawVal |= ((regVal.TC1_ as u32 & 0x2000) << 13) as u32; 
        rawVal |= ((regVal.TC2_ as u32 & 0x4000) << 14) as u32; 
        rawVal |= ((regVal.TC3_ as u32 & 0x8000) << 15) as u32; 
        rawVal |= ((regVal.TC4_ as u32 & 0x10000) << 16) as u32; 
        rawVal |= ((regVal.ADC0_ as u32 & 0x20000) << 17) as u32; 
        rawVal |= ((regVal.ADC1_ as u32 & 0x40000) << 18) as u32; 
        rawVal |= ((regVal.SDADC_ as u32 & 0x80000) << 19) as u32; 
        rawVal |= ((regVal.AC_ as u32 & 0x100000) << 20) as u32; 
        rawVal |= ((regVal.DAC_ as u32 & 0x200000) << 21) as u32; 
        rawVal |= ((regVal.PTC_ as u32 & 0x400000) << 22) as u32; 
        rawVal |= ((regVal.CCL_ as u32 & 0x800000) << 23) as u32; 
        ptr::write_volatile(&mut self._reg.APBCMASK, rawVal)
    }

    // --------------------------------------------------


    // --------------------------------------------------
    // Register BitField Access
    // --------------------------------------------------
    #[inline]
    pub fn Set_INTENCLR_CKRDY(&mut self, val: u8) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.INTENCLR) & !(0x1);
            ptr::write_volatile(&mut self._reg.INTENCLR, ((val as u8) << 0) | regVal)
        }
    }

    #[inline]
    pub fn Set_INTENSET_CKRDY(&mut self, val: u8) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.INTENSET) & !(0x1);
            ptr::write_volatile(&mut self._reg.INTENSET, ((val as u8) << 0) | regVal)
        }
    }

    #[inline]
    pub fn Set_INTFLAG_CKRDY(&mut self, val: u8) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.INTFLAG) & !(0x1);
            ptr::write_volatile(&mut self._reg.INTFLAG, ((val as u8) << 0) | regVal)
        }
    }

    #[inline]
    pub fn Set_CPUDIV_CPUDIV(&mut self, val: MCLK_CPUDIV__CPUDIV) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.CPUDIV) & !(0xFF);
            ptr::write_volatile(&mut self._reg.CPUDIV, ((val as u8) << 0) | regVal)
        }
    }

    #[inline]
    pub fn Set_AHBMASK_HPB0_(&mut self, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.AHBMASK) & !(0x1);
            ptr::write_volatile(&mut self._reg.AHBMASK, ((val as u32) << 0) | regVal)
        }
    }
    #[inline]
    pub fn Set_AHBMASK_HPB1_(&mut self, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.AHBMASK) & !(0x2);
            ptr::write_volatile(&mut self._reg.AHBMASK, ((val as u32) << 1) | regVal)
        }
    }
    #[inline]
    pub fn Set_AHBMASK_HPB2_(&mut self, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.AHBMASK) & !(0x4);
            ptr::write_volatile(&mut self._reg.AHBMASK, ((val as u32) << 2) | regVal)
        }
    }
    #[inline]
    pub fn Set_AHBMASK_DSU_(&mut self, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.AHBMASK) & !(0x8);
            ptr::write_volatile(&mut self._reg.AHBMASK, ((val as u32) << 3) | regVal)
        }
    }
    #[inline]
    pub fn Set_AHBMASK_HMATRIXHS_(&mut self, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.AHBMASK) & !(0x10);
            ptr::write_volatile(&mut self._reg.AHBMASK, ((val as u32) << 4) | regVal)
        }
    }
    #[inline]
    pub fn Set_AHBMASK_NVMCTRL_(&mut self, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.AHBMASK) & !(0x20);
            ptr::write_volatile(&mut self._reg.AHBMASK, ((val as u32) << 5) | regVal)
        }
    }
    #[inline]
    pub fn Set_AHBMASK_HSRAM_(&mut self, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.AHBMASK) & !(0x40);
            ptr::write_volatile(&mut self._reg.AHBMASK, ((val as u32) << 6) | regVal)
        }
    }
    #[inline]
    pub fn Set_AHBMASK_DMAC_(&mut self, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.AHBMASK) & !(0x80);
            ptr::write_volatile(&mut self._reg.AHBMASK, ((val as u32) << 7) | regVal)
        }
    }
    #[inline]
    pub fn Set_AHBMASK_CAN0_(&mut self, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.AHBMASK) & !(0x100);
            ptr::write_volatile(&mut self._reg.AHBMASK, ((val as u32) << 8) | regVal)
        }
    }
    #[inline]
    pub fn Set_AHBMASK_CAN1_(&mut self, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.AHBMASK) & !(0x200);
            ptr::write_volatile(&mut self._reg.AHBMASK, ((val as u32) << 9) | regVal)
        }
    }
    #[inline]
    pub fn Set_AHBMASK_PAC_(&mut self, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.AHBMASK) & !(0x400);
            ptr::write_volatile(&mut self._reg.AHBMASK, ((val as u32) << 10) | regVal)
        }
    }
    #[inline]
    pub fn Set_AHBMASK_NVMCTRL_PICACHU_(&mut self, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.AHBMASK) & !(0x800);
            ptr::write_volatile(&mut self._reg.AHBMASK, ((val as u32) << 11) | regVal)
        }
    }
    #[inline]
    pub fn Set_AHBMASK_DIVAS_(&mut self, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.AHBMASK) & !(0x1000);
            ptr::write_volatile(&mut self._reg.AHBMASK, ((val as u32) << 12) | regVal)
        }
    }

    #[inline]
    pub fn Set_APBAMASK_PAC_(&mut self, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.APBAMASK) & !(0x1);
            ptr::write_volatile(&mut self._reg.APBAMASK, ((val as u32) << 0) | regVal)
        }
    }
    #[inline]
    pub fn Set_APBAMASK_PM_(&mut self, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.APBAMASK) & !(0x2);
            ptr::write_volatile(&mut self._reg.APBAMASK, ((val as u32) << 1) | regVal)
        }
    }
    #[inline]
    pub fn Set_APBAMASK_MCLK_(&mut self, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.APBAMASK) & !(0x4);
            ptr::write_volatile(&mut self._reg.APBAMASK, ((val as u32) << 2) | regVal)
        }
    }
    #[inline]
    pub fn Set_APBAMASK_RSTC_(&mut self, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.APBAMASK) & !(0x8);
            ptr::write_volatile(&mut self._reg.APBAMASK, ((val as u32) << 3) | regVal)
        }
    }
    #[inline]
    pub fn Set_APBAMASK_OSCCTRL_(&mut self, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.APBAMASK) & !(0x10);
            ptr::write_volatile(&mut self._reg.APBAMASK, ((val as u32) << 4) | regVal)
        }
    }
    #[inline]
    pub fn Set_APBAMASK_OSC32KCTRL_(&mut self, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.APBAMASK) & !(0x20);
            ptr::write_volatile(&mut self._reg.APBAMASK, ((val as u32) << 5) | regVal)
        }
    }
    #[inline]
    pub fn Set_APBAMASK_SUPC_(&mut self, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.APBAMASK) & !(0x40);
            ptr::write_volatile(&mut self._reg.APBAMASK, ((val as u32) << 6) | regVal)
        }
    }
    #[inline]
    pub fn Set_APBAMASK_GCLK_(&mut self, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.APBAMASK) & !(0x80);
            ptr::write_volatile(&mut self._reg.APBAMASK, ((val as u32) << 7) | regVal)
        }
    }
    #[inline]
    pub fn Set_APBAMASK_WDT_(&mut self, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.APBAMASK) & !(0x100);
            ptr::write_volatile(&mut self._reg.APBAMASK, ((val as u32) << 8) | regVal)
        }
    }
    #[inline]
    pub fn Set_APBAMASK_RTC_(&mut self, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.APBAMASK) & !(0x200);
            ptr::write_volatile(&mut self._reg.APBAMASK, ((val as u32) << 9) | regVal)
        }
    }
    #[inline]
    pub fn Set_APBAMASK_EIC_(&mut self, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.APBAMASK) & !(0x400);
            ptr::write_volatile(&mut self._reg.APBAMASK, ((val as u32) << 10) | regVal)
        }
    }
    #[inline]
    pub fn Set_APBAMASK_FREQM_(&mut self, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.APBAMASK) & !(0x800);
            ptr::write_volatile(&mut self._reg.APBAMASK, ((val as u32) << 11) | regVal)
        }
    }
    #[inline]
    pub fn Set_APBAMASK_TSENS_(&mut self, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.APBAMASK) & !(0x1000);
            ptr::write_volatile(&mut self._reg.APBAMASK, ((val as u32) << 12) | regVal)
        }
    }

    #[inline]
    pub fn Set_APBBMASK_PORT_(&mut self, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.APBBMASK) & !(0x1);
            ptr::write_volatile(&mut self._reg.APBBMASK, ((val as u32) << 0) | regVal)
        }
    }
    #[inline]
    pub fn Set_APBBMASK_DSU_(&mut self, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.APBBMASK) & !(0x2);
            ptr::write_volatile(&mut self._reg.APBBMASK, ((val as u32) << 1) | regVal)
        }
    }
    #[inline]
    pub fn Set_APBBMASK_NVMCTRL_(&mut self, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.APBBMASK) & !(0x4);
            ptr::write_volatile(&mut self._reg.APBBMASK, ((val as u32) << 2) | regVal)
        }
    }
    #[inline]
    pub fn Set_APBBMASK_HMATRIXHS_(&mut self, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.APBBMASK) & !(0x20);
            ptr::write_volatile(&mut self._reg.APBBMASK, ((val as u32) << 5) | regVal)
        }
    }

    #[inline]
    pub fn Set_APBCMASK_EVSYS_(&mut self, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.APBCMASK) & !(0x1);
            ptr::write_volatile(&mut self._reg.APBCMASK, ((val as u32) << 0) | regVal)
        }
    }
    #[inline]
    pub fn Set_APBCMASK_SERCOM0_(&mut self, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.APBCMASK) & !(0x2);
            ptr::write_volatile(&mut self._reg.APBCMASK, ((val as u32) << 1) | regVal)
        }
    }
    #[inline]
    pub fn Set_APBCMASK_SERCOM1_(&mut self, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.APBCMASK) & !(0x4);
            ptr::write_volatile(&mut self._reg.APBCMASK, ((val as u32) << 2) | regVal)
        }
    }
    #[inline]
    pub fn Set_APBCMASK_SERCOM2_(&mut self, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.APBCMASK) & !(0x8);
            ptr::write_volatile(&mut self._reg.APBCMASK, ((val as u32) << 3) | regVal)
        }
    }
    #[inline]
    pub fn Set_APBCMASK_SERCOM3_(&mut self, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.APBCMASK) & !(0x10);
            ptr::write_volatile(&mut self._reg.APBCMASK, ((val as u32) << 4) | regVal)
        }
    }
    #[inline]
    pub fn Set_APBCMASK_SERCOM4_(&mut self, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.APBCMASK) & !(0x20);
            ptr::write_volatile(&mut self._reg.APBCMASK, ((val as u32) << 5) | regVal)
        }
    }
    #[inline]
    pub fn Set_APBCMASK_SERCOM5_(&mut self, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.APBCMASK) & !(0x40);
            ptr::write_volatile(&mut self._reg.APBCMASK, ((val as u32) << 6) | regVal)
        }
    }
    #[inline]
    pub fn Set_APBCMASK_TCC0_(&mut self, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.APBCMASK) & !(0x200);
            ptr::write_volatile(&mut self._reg.APBCMASK, ((val as u32) << 9) | regVal)
        }
    }
    #[inline]
    pub fn Set_APBCMASK_TCC1_(&mut self, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.APBCMASK) & !(0x400);
            ptr::write_volatile(&mut self._reg.APBCMASK, ((val as u32) << 10) | regVal)
        }
    }
    #[inline]
    pub fn Set_APBCMASK_TCC2_(&mut self, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.APBCMASK) & !(0x800);
            ptr::write_volatile(&mut self._reg.APBCMASK, ((val as u32) << 11) | regVal)
        }
    }
    #[inline]
    pub fn Set_APBCMASK_TC0_(&mut self, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.APBCMASK) & !(0x1000);
            ptr::write_volatile(&mut self._reg.APBCMASK, ((val as u32) << 12) | regVal)
        }
    }
    #[inline]
    pub fn Set_APBCMASK_TC1_(&mut self, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.APBCMASK) & !(0x2000);
            ptr::write_volatile(&mut self._reg.APBCMASK, ((val as u32) << 13) | regVal)
        }
    }
    #[inline]
    pub fn Set_APBCMASK_TC2_(&mut self, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.APBCMASK) & !(0x4000);
            ptr::write_volatile(&mut self._reg.APBCMASK, ((val as u32) << 14) | regVal)
        }
    }
    #[inline]
    pub fn Set_APBCMASK_TC3_(&mut self, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.APBCMASK) & !(0x8000);
            ptr::write_volatile(&mut self._reg.APBCMASK, ((val as u32) << 15) | regVal)
        }
    }
    #[inline]
    pub fn Set_APBCMASK_TC4_(&mut self, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.APBCMASK) & !(0x10000);
            ptr::write_volatile(&mut self._reg.APBCMASK, ((val as u32) << 16) | regVal)
        }
    }
    #[inline]
    pub fn Set_APBCMASK_ADC0_(&mut self, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.APBCMASK) & !(0x20000);
            ptr::write_volatile(&mut self._reg.APBCMASK, ((val as u32) << 17) | regVal)
        }
    }
    #[inline]
    pub fn Set_APBCMASK_ADC1_(&mut self, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.APBCMASK) & !(0x40000);
            ptr::write_volatile(&mut self._reg.APBCMASK, ((val as u32) << 18) | regVal)
        }
    }
    #[inline]
    pub fn Set_APBCMASK_SDADC_(&mut self, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.APBCMASK) & !(0x80000);
            ptr::write_volatile(&mut self._reg.APBCMASK, ((val as u32) << 19) | regVal)
        }
    }
    #[inline]
    pub fn Set_APBCMASK_AC_(&mut self, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.APBCMASK) & !(0x100000);
            ptr::write_volatile(&mut self._reg.APBCMASK, ((val as u32) << 20) | regVal)
        }
    }
    #[inline]
    pub fn Set_APBCMASK_DAC_(&mut self, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.APBCMASK) & !(0x200000);
            ptr::write_volatile(&mut self._reg.APBCMASK, ((val as u32) << 21) | regVal)
        }
    }
    #[inline]
    pub fn Set_APBCMASK_PTC_(&mut self, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.APBCMASK) & !(0x400000);
            ptr::write_volatile(&mut self._reg.APBCMASK, ((val as u32) << 22) | regVal)
        }
    }
    #[inline]
    pub fn Set_APBCMASK_CCL_(&mut self, val: u32) {
        unsafe {
            let regVal = ptr::read_volatile(&self._reg.APBCMASK) & !(0x800000);
            ptr::write_volatile(&mut self._reg.APBCMASK, ((val as u32) << 23) | regVal)
        }
    }

    // --------------------------------------------------


}

// --------------------------------------------------
// Register Type Definition
// --------------------------------------------------
pub struct AHBMASK {
    pub HPB0_: u8,
    pub HPB1_: u8,
    pub HPB2_: u8,
    pub DSU_: u8,
    pub HMATRIXHS_: u8,
    pub NVMCTRL_: u8,
    pub HSRAM_: u8,
    pub DMAC_: u8,
    pub CAN0_: u8,
    pub CAN1_: u8,
    pub PAC_: u8,
    pub NVMCTRL_PICACHU_: u8,
    pub DIVAS_: u8,
}

pub struct APBAMASK {
    pub PAC_: u8,
    pub PM_: u8,
    pub MCLK_: u8,
    pub RSTC_: u8,
    pub OSCCTRL_: u8,
    pub OSC32KCTRL_: u8,
    pub SUPC_: u8,
    pub GCLK_: u8,
    pub WDT_: u8,
    pub RTC_: u8,
    pub EIC_: u8,
    pub FREQM_: u8,
    pub TSENS_: u8,
}

pub struct APBBMASK {
    pub PORT_: u8,
    pub DSU_: u8,
    pub NVMCTRL_: u8,
    pub HMATRIXHS_: u8,
}

pub struct APBCMASK {
    pub EVSYS_: u8,
    pub SERCOM0_: u8,
    pub SERCOM1_: u8,
    pub SERCOM2_: u8,
    pub SERCOM3_: u8,
    pub SERCOM4_: u8,
    pub SERCOM5_: u8,
    pub TCC0_: u8,
    pub TCC1_: u8,
    pub TCC2_: u8,
    pub TC0_: u8,
    pub TC1_: u8,
    pub TC2_: u8,
    pub TC3_: u8,
    pub TC4_: u8,
    pub ADC0_: u8,
    pub ADC1_: u8,
    pub SDADC_: u8,
    pub AC_: u8,
    pub DAC_: u8,
    pub PTC_: u8,
    pub CCL_: u8,
}

// --------------------------------------------------


pub enum MCLK_CPUDIV__CPUDIV {
    DIV1 = 1,                                       //Divide by 1
    DIV2 = 2,                                       //Divide by 2
    DIV4 = 4,                                       //Divide by 4
    DIV8 = 8,                                       //Divide by 8
    DIV16 = 16,                                     //Divide by 16
    DIV32 = 32,                                     //Divide by 32
    DIV64 = 64,                                     //Divide by 64
    DIV128 = 128,                                   //Divide by 128
}

