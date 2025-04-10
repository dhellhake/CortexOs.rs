#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(static_mut_refs)]

use core::ptr;

use crate::main;

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
        unsafe static mut _szero: u32;
        unsafe static mut _ezero: u32;
    }

    unsafe {
        let start_addr: *const u32 = &_szero as *const u32;
        let end_addr: *const u32 = &_ezero as *const u32;
        let relocate_size: usize = end_addr as usize - start_addr as usize;

        ptr::write_bytes(start_addr as *mut u32, 0, relocate_size);
    }

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