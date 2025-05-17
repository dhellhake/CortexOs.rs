#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(static_mut_refs)]

use core::ptr;

use crate::{cortex::{self}, main, peripherals::{gclk::{GenericClockGenerator, GCLK}, nvmctrl::{NVMController, RWSSelect}, port::IOPinController, scb::{SystemControlBlock, SCB}, systick::SystemTimer}, SysTick, NVMCTRL, PORT};

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
        }
    });
    

    cortex::CriticalSection(|| {
        unsafe {
            if let Some(ref mut syst) = SysTick.borrow().as_mut_unchecked() {
                syst.WriteRaw_ControlAndStatusRegister(0);
                syst.WriteRaw_ReloadValueRegister(12345);
                syst.WriteRaw_CurrentValueRegister(0);
            }
            if let Some(ref mut port) = PORT.borrow().as_mut_unchecked() {
                port.Set_PinDirection(1, 9, true);
            }
            if let Some(ref mut nvmctrl) = NVMCTRL.borrow().as_mut_unchecked() {
                nvmctrl.Set_ReadWaitStates(RWSSelect::DUAL);
            }
            if let Some(ref mut gclk) = GCLK.borrow().as_mut_unchecked() {
                gclk.WriteRaw_Control(1);
            }
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