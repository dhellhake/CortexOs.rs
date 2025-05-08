#![no_main]
#![no_std]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![feature(unsafe_cell_access)]

pub mod os;
pub mod cortex;
pub mod peripherals;

use core::{ops::DerefMut, panic::PanicInfo};

use cortex::{scb::{SystemControlBlock, SCB}, systick::{SysTick, SystemTimer}};
use os::{task::TaskStatus, OperatingSystem, Os};
use peripherals::{nvmctrl::{NVMController, RWSSelect, NVMCTRL}, port::{IOPinController, PORT}};


fn taskone(_tstmp: u32) {
    cortex::CriticalSection(|| {
        unsafe {
            if let Some(ref mut port) = PORT.borrow().as_mut_unchecked() {
                port.Set_PinOutState(1, 9, false);
            }
        }
    });
}
fn tasktwo(_tstmp: u32) {
    cortex::CriticalSection(|| {
        unsafe {            
            if let Some(ref mut port) = PORT.borrow().as_mut_unchecked() {
                port.Set_PinOutState(1, 9, true);
            }
        }
    });
}

fn main() -> ! {    
    os::OsSection(|ot| {
        Os.borrow(ot).replace(Some(OperatingSystem::new().unwrap()));
    });
    os::OsSection(|st| {
        if let Some(ref mut os) = Os.borrow(st).borrow_mut().deref_mut() {
            os.SetTask(0, taskone);            
            os.SetTask(1, tasktwo);
            os.tasks[os.taskIdx as usize].status = TaskStatus::Active;
        }
    });
    
    cortex::CriticalSection(|| {
        unsafe {
            SysTick.borrow().replace(Some(SystemTimer::new().unwrap()));
            PORT.borrow().replace(Some(IOPinController::new().unwrap()));
            NVMCTRL.borrow().replace(Some(NVMController::new().unwrap()));
            SCB.borrow().replace(Some(SystemControlBlock::new().unwrap()));
        }
    });

    cortex::CriticalSection(|| {
        unsafe {
            if let Some(ref mut syst) = SysTick.borrow().as_mut_unchecked() {
                syst.Set_ControlValue(0);
                syst.Set_ReloadValue(12345);
                syst.Set_CounterValue(0);
                syst.Set_ControlValue(7);
            }
            if let Some(ref mut port) = PORT.borrow().as_mut_unchecked() {
                port.Set_PinDirection(1, 9, true);
            }
            if let Some(ref mut nvmctrl) = NVMCTRL.borrow().as_mut_unchecked() {
                nvmctrl.Set_ReadWaitStates(RWSSelect::DUAL);
            }
        }
    });

    OperatingSystem::OsStart();    
}

#[panic_handler]
fn panic(_i: &PanicInfo) -> ! {
    loop {}
}

