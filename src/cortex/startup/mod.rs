#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(static_mut_refs)]

use core::ptr;

use crate::{cortex::{self}, main, peripherals::{gclk::{GenericClockGenerator, GCLK, GCLK_GENCTRL__DIVSEL, GCLK_GENCTRL__SRC, GCLK_PCHCTRL__GEN, GENCTRL}, mclk::{MainClock, MCLK}, nvmctrl::{NVMController, RWSSelect}, oscctrl::{OscillatorsControl, DPLLCTRLA, DPLLCTRLB, DPLLRATIO, OSCCTRL, OSCCTRL_DPLLCTRLB__FILTER, OSCCTRL_DPLLCTRLB__LTIME, OSCCTRL_DPLLCTRLB__REFCLK, OSCCTRL_DPLLPRESC__PRESC, OSCCTRL_OSC48MDIV__DIV}, port::IOPinController, scb::{SystemControlBlock, SCB}, systick::SystemTimer}, SysTick, NVMCTRL, PORT};

extern "C" {

    fn NonMaskableInt();

    fn HardFault();

    fn SVCall();

    fn PendSV();

    fn SysTick_Isr();
}

#[repr(C)]
pub union Vector {
    handler: unsafe extern "C" fn(),
    reserved: usize,
}

#[doc(hidden)]
#[unsafe(link_section = ".vectors.exception_table")]
#[no_mangle]
pub static __exception_table: [Vector; 15] = [
    // Exception 1: Reset Handler.
    Vector { handler: Reset },
    // Exception 2: Non Maskable Interrupt.
    Vector { handler: NonMaskableInt },
    // Exception 3: Hard Fault Interrupt.
    Vector { handler: HardFault },
    // 4-10: Reserved
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    // Exception 11: SV Call Interrupt.
    Vector { handler: SVCall },
    // 12-13: Reserved
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    // Exception 14: Pend SV Interrupt
    Vector { handler: PendSV },
    // Exception 15: System Tick Interrupt.
    Vector { handler: SysTick_Isr },
];

#[no_mangle]
pub unsafe extern "C" fn Reset() {
    
    unsafe extern "C" {
        unsafe static mut _etext: u32;
        unsafe static mut _szero: u32;
        unsafe static mut _ezero: u32;
        unsafe static mut _srelocate: u32;
        unsafe static mut _erelocate: u32;
    }

    unsafe {
        let start_addr: *const u32 = &_szero as *const u32;
        let end_addr: *const u32 = &_ezero as *const u32;
        let relocate_size: usize = end_addr as usize - start_addr as usize;

        ptr::write_bytes(start_addr as *mut u32, 0, relocate_size);
    }
    

    unsafe {
        let etext: *const u32 = &_etext as *const u32;
        let start_relocate: *const u32 = &_srelocate as *const u32;
        let end_relocate: *const u32 = &_erelocate as *const u32;
        let relocate_size: usize = end_relocate as usize - start_relocate as usize;

        ptr::copy_nonoverlapping(etext, start_relocate as *mut u32, relocate_size / 4);
    }

    cortex::CriticalSection(|| {
        unsafe {
            SysTick.borrow().replace(Some(SystemTimer::new().unwrap()));
            PORT.borrow().replace(Some(IOPinController::new().unwrap()));
            NVMCTRL.borrow().replace(Some(NVMController::new().unwrap()));
            SCB.borrow().replace(Some(SystemControlBlock::new().unwrap()));
            GCLK.borrow().replace(Some(GenericClockGenerator::new().unwrap()));
            OSCCTRL.borrow().replace(Some(OscillatorsControl::new().unwrap()));
            MCLK.borrow().replace(Some(MainClock::new().unwrap()));
        }
    });   

    cortex::CriticalSection(|| {
        unsafe {
            let gclk = GCLK.borrow().as_mut_unchecked().as_mut().unwrap();
            let syst = SysTick.borrow().as_mut_unchecked().as_mut().unwrap();
            let port = PORT.borrow().as_mut_unchecked().as_mut().unwrap();
            let nvmctrl = NVMCTRL.borrow().as_mut_unchecked().as_mut().unwrap();
            let oscctrl = OSCCTRL.borrow().as_mut_unchecked().as_mut().unwrap();

            syst.WriteRaw_ControlAndStatusRegister(0);
            syst.WriteRaw_ReloadValueRegister(64618);
            syst.WriteRaw_CurrentValueRegister(0);

            port.Set_PinDirection(1, 9, true);
            
            nvmctrl.Set_ReadWaitStates(RWSSelect::DUAL);
            
            oscctrl.Set_OSC48MDIV_DIV(OSCCTRL_OSC48MDIV__DIV::DIV1);
            while oscctrl.ReadRaw_Osc48mSynchronizationBusy() != 0 {}

            // Set ClockGenerator 1 to 1Mhz (48Mhz / 48)
            gclk.Write_GenericClockGeneratorControl(1, GENCTRL {
                    SRC: GCLK_GENCTRL__SRC::OSC48M,
                    GENEN: 1,
                    IDC: 1,
                    OOV: 0,
                    OE: 0,
                    DIVSEL: GCLK_GENCTRL__DIVSEL::DIV1,
                    RUNSTDBY: 0,
                    DIV: 48,
                });
            
            // Set ClockGenerator 2 to 32kHz (32kHz / 1)
            gclk.Write_GenericClockGeneratorControl(2, GENCTRL {
                    SRC: GCLK_GENCTRL__SRC::OSCULP32K,
                    GENEN: 1,
                    IDC: 1,
                    OOV: 0,
                    OE: 0,
                    DIVSEL: GCLK_GENCTRL__DIVSEL::DIV1,
                    RUNSTDBY: 0,
                    DIV: 1,
                });

            // Enable Clock Generation for FDPLL (GCLK_ID 0)
            gclk.Set_PCHCTRL_CHEN(0, 0);                            // Disable Channel for GCLK_DPLL 
            gclk.Set_PCHCTRL_GEN(0, GCLK_PCHCTRL__GEN::GCLK1);      // Set Channel Clock Generator to GCLK1
            gclk.Set_PCHCTRL_CHEN(0, 1);                            // Disable Channel for GCLK_DPLL 

            // Enable Clock Generation for FDPLL32K (GCLK_ID 1)
            gclk.Set_PCHCTRL_CHEN(1, 0);                            // Disable Channel for GCLK_DPLL 
            gclk.Set_PCHCTRL_GEN(1, GCLK_PCHCTRL__GEN::GCLK2);      // Set Channel Clock Generator to GCLK2
            gclk.Set_PCHCTRL_CHEN(1, 1);                            // Disable Channel for GCLK_DPLL 
            
            // Set DPLL Reference Clock to GCLK (1Mhz)
            oscctrl.Write_DigitalCoreConfiguration(DPLLCTRLB {
                DIV: 0,
                LBYPASS: 0,
                LTIME: OSCCTRL_DPLLCTRLB__LTIME::DEFAULT,
                REFCLK: OSCCTRL_DPLLCTRLB__REFCLK::GCLK,
                WUF: 0,
                LPEN: 0,
                FILTER: OSCCTRL_DPLLCTRLB__FILTER::DEFAULT,
            });

            // Set DPLL Clock Multiplier to 128 (128 Mhz)
            oscctrl.Write_DpllRatioControl(DPLLRATIO {
                LDR: 128,
                LDRFRAC: 0,
            });
            while (oscctrl.ReadRaw_DpllSynchronizationBusy() & 0b0100 ) != 0 {}
            
            // Set DPLL Clock Prescaler to 1 (128 Mhz)
            oscctrl.Set_DPLLPRESC_PRESC(OSCCTRL_DPLLPRESC__PRESC::DIV2);
            while (oscctrl.ReadRaw_DpllSynchronizationBusy() & 0b1000 ) != 0 {}

            // Enable DPLL
            oscctrl.Write_DpllControl(DPLLCTRLA {
                ONDEMAND: 0,
                RUNSTDBY: 0,
                ENABLE: 1,
            });
            while (oscctrl.ReadRaw_DpllSynchronizationBusy() & 0b0010 ) != 0 {}

            // Wait for DPLL Frequency to stabilize
            while (oscctrl.ReadRaw_DpllStatus() & 0b1 ) != 1 {}
            
            // Set ClockGenerator 0 to 64Mhz
            gclk.Write_GenericClockGeneratorControl(0, GENCTRL {
                    SRC: GCLK_GENCTRL__SRC::DPLL96M,
                    GENEN: 1,
                    IDC: 1,
                    OOV: 0,
                    OE: 0,
                    DIVSEL: GCLK_GENCTRL__DIVSEL::DIV1,
                    RUNSTDBY: 0,
                    DIV: 1,
                });

        }
    });

    main();
}

#[no_mangle]
pub unsafe extern "C" fn DefaultHandler_() -> ! {
    #[allow(clippy::empty_loop)]
    loop {}
}

#[unsafe(link_section = ".vectors.interrupt_table")]
#[no_mangle]
pub static __interrupt_table: [unsafe extern "C" fn(); 32] = [{
    extern "C" {
        fn DefaultHandler();
    }

    DefaultHandler
}; 32];