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

use os::{task::{TaskCycleTime, TaskStatus}, OperatingSystem, Os};
use peripherals::{nvmctrl::NVMCTRL, port::PORT, systick::SysTick};

#[unsafe(link_section = ".ramfunc")]
fn taskone(_tstmp: u32) {
    cortex::CriticalSection(#[inline(always)] || {
        unsafe {
            if let Some(ref mut port) = PORT.borrow().as_mut_unchecked() {
                port.Set_PinOutState(1, 9, true);
            }
        }
    });
}


#[unsafe(link_section = ".ramfunc")]
fn tasktwo(_tstmp: u32) {
    cortex::CriticalSection(#[inline(always)] || {
        unsafe {            
            if let Some(ref mut port) = PORT.borrow().as_mut_unchecked() {
                port.Set_PinOutState(1, 9, false);
            }
        }
    });
}

fn background(_tstmp: u32) {
    loop {}
}

fn main() -> ! {    
    os::OsSection(|| {
        Os.borrow().replace(Some(OperatingSystem::new().unwrap()));
    });
    os::OsSection(|| {
        if let Some(ref mut os) = Os.borrow().borrow_mut().deref_mut() {
            os.SetTask(0, taskone, TaskCycleTime::_5MS);
            os.SetTask(1, tasktwo, TaskCycleTime::_5MS);
            os.SetTask(2, background, TaskCycleTime::NonCyclic);
            os.tasks[os.taskIdx as usize].status = TaskStatus::Active;
        }
    });

    cortex::CriticalSection(|| {
        unsafe {
            if let Some(ref mut syst) = SysTick.borrow().as_mut_unchecked() {
                syst.WriteRaw_ControlAndStatusRegister(7);
            }
        }
    });

    OperatingSystem::OsStart();
}

#[panic_handler]
fn panic(_i: &PanicInfo) -> ! {
    loop {}
}

