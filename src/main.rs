#![no_main]
#![no_std]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(non_snake_case)]

pub mod os;
pub mod cortex;
pub mod peripherals;

use core::{arch::asm, mem, ops::DerefMut, panic::PanicInfo};

use cortex::systick::{SysTick, SystemTimer};
use os::{OperatingSystem, Os, Task};
use peripherals::port::{IOPinController, PORT};

#[no_mangle]
pub unsafe extern "C" fn SysTick_Isr() {       
    cortex::CriticalSection(|st| {
        if let Some(ref mut port) = PORT.borrow(st).borrow_mut().deref_mut() {
            if port.Get_PinOutState(1, 9)
            {
                port.Set_PinOutState(1, 9, false);
            } else {                
                port.Set_PinOutState(1, 9, true);
            }
        }
    });
}

#[no_mangle]
pub unsafe extern "C" fn PendSV() {

}

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

fn cyclic(tskref: usize) {
    unsafe  {
        let task: Task = *(tskref as *const Task);
        
        (task.cyclic)(0);
        
        loop { }
    }
}

fn main() -> ! {
    cortex::CriticalSection(|st| {
        SysTick.borrow(st).replace(Some(SystemTimer::new().unwrap()));
        PORT.borrow(st).replace(Some(IOPinController::new().unwrap()));
    });

    cortex::CriticalSection(|st| {
        if let Some(ref mut port) = PORT.borrow(st).borrow_mut().deref_mut() {
            port.Set_PinDirection(1, 9, true);
        }
    });

    let mut reload: u32 = 0;
    cortex::CriticalSection(|st| {
        if let Some(ref mut syst) = SysTick.borrow(st).borrow_mut().deref_mut() {
            syst.Set_ControlValue(0);
            syst.Set_ReloadValue(12345);
            syst.Set_CounterValue(0);
            syst.Set_ControlValue(7);
            reload = syst.Get_ReloadValue();
        }
    });
    
    os::OsSection(|ot| {
        Os.borrow(ot).replace(Some(OperatingSystem::new().unwrap()));
    });

    let mut tskRef: usize = 0;
    let mut stack: u32 = 0;
    os::OsSection(|st| {
        if let Some(ref mut os) = Os.borrow(st).borrow_mut().deref_mut() {
            os.tasks[0].cyclic = taskone;
            os.tasks[0].sp = (os.tasks[0].stack.as_ptr() as usize) + 64 - (16 / 4);
            os.tasks[0].stack[64 - 1] = 0x01000000;
            os.tasks[0].stack[64 - 2] = (cyclic as *const ()) as u32;
            os.tasks[0].stack[64 - 8] = ((os.tasks.as_ptr() as usize) + (mem::size_of::<Task>() * 0)) as u32;

            os.tasks[1].cyclic = tasktwo;
            os.tasks[1].sp = (os.tasks[1].stack.as_ptr() as usize) + 64 - (16 / 4);
            os.tasks[1].stack[64 - 1] = 0x01000000;
            os.tasks[1].stack[64 - 2] = (cyclic as *const ()) as u32;
            os.tasks[1].stack[64 - 8] = ((os.tasks.as_ptr() as usize) + (mem::size_of::<Task>() * 1)) as u32;

            tskRef = ((&os.tasks[0]) as *const Task) as usize;
            stack = ((os.tasks[0].stack.as_ptr() as usize) + 64) as u32;
        }
    });
    
    unsafe { asm!("msr	psp, r0", in("r0")stack) };
    unsafe { asm!("msr	control, r0", in("r0")0x3) };
    unsafe { asm!("isb 0xF") };

    cyclic(tskRef);


    loop {}
}

#[panic_handler]
fn panic(_i: &PanicInfo) -> ! {
    loop {}
}

