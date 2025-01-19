#![no_main]
#![no_std]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

pub mod device;

use core::panic::PanicInfo;

fn main() -> ! {
    loop {}
}

#[panic_handler]
fn panic(_i: &PanicInfo) -> ! {
    loop {}
}

