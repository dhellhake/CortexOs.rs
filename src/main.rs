#![no_main]
#![no_std]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_assignments)]

pub mod os;
pub mod cortex;
pub mod peripherals;

use core::{arch::asm, ops::DerefMut, panic::PanicInfo};

use cortex::{scb::{SystemControlBlock, SCB}, systick::{SysTick, SystemTimer}};
use os::{OperatingSystem, Os, Task};
use peripherals::port::{IOPinController, PORT};


fn taskone(_tstmp: u32) {
    cortex::CriticalSection(|st| {
        if let Some(ref mut port) = PORT.borrow(st).borrow_mut().deref_mut() {
            port.Set_PinOutState(1, 9, false);
        }
    });
}
fn tasktwo(_tstmp: u32) {
    cortex::CriticalSection(|st| {
        if let Some(ref mut port) = PORT.borrow(st).borrow_mut().deref_mut() {
            port.Set_PinOutState(1, 9, true);
        }
    });
}

fn main() -> ! {
    cortex::CriticalSection(|st| {
        SysTick.borrow(st).replace(Some(SystemTimer::new().unwrap()));
        PORT.borrow(st).replace(Some(IOPinController::new().unwrap()));
        SCB.borrow(st).replace(Some(SystemControlBlock::new().unwrap()));
    });

    cortex::CriticalSection(|st| {
        if let Some(ref mut port) = PORT.borrow(st).borrow_mut().deref_mut() {
            port.Set_PinDirection(1, 9, true);
        }
    });
    
    os::OsSection(|ot| {
        Os.borrow(ot).replace(Some(OperatingSystem::new().unwrap()));
    });

    let mut stack: u32 = 0;
    os::OsSection(|st| {
        if let Some(ref mut os) = Os.borrow(st).borrow_mut().deref_mut() {
            os.SetTask(0, taskone);
            
            os.SetTask(1, tasktwo);

            stack = (&(os.tasks[0].stack[256 - 16]) as *const u32) as u32;
        }
    });

    
    cortex::CriticalSection(|st| {
        if let Some(ref mut syst) = SysTick.borrow(st).borrow_mut().deref_mut() {
            syst.Set_ControlValue(0);
            syst.Set_ReloadValue(12345);
            syst.Set_CounterValue(0);
            syst.Set_ControlValue(7);
        }
    });


    let ctrl: u32 = 0x3;
    let mut startTask: *const Task = (0 as *const u32) as *const Task;

    unsafe {
        asm!("msr psp, {0}", in(reg) stack);
        asm!("msr control, {0}", in(reg) ctrl);
        asm!("isb");
    }

    os::OsSection(|st| {
        if let Some(ref mut os) = Os.borrow(st).borrow_mut().deref_mut() {
            startTask = &(os.tasks[0]);
        }
    });


    os::cyclic(startTask);

    loop {}
}

#[panic_handler]
fn panic(_i: &PanicInfo) -> ! {
    loop {}
}

