#![no_main]
#![no_std]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_assignments)]

pub mod os;
pub mod cortex;
pub mod peripherals;

use core::{arch::asm, mem, ops::DerefMut, panic::PanicInfo};

use cortex::{scb::{SystemControlBlock, SCB}, systick::{SysTick, SystemTimer}};
use os::{OperatingSystem, Os, Task};
use peripherals::port::{IOPinController, PORT};

#[no_mangle]
pub unsafe extern "C" fn SysTick_Isr() {  
    cortex::CriticalSection(|st| {
        if let Some(ref mut scb) = SCB.borrow(st).borrow_mut().deref_mut() {
            scb.Set_PendSV();
        }
    });
}

#[no_mangle]
pub unsafe extern "C" fn PendSV() {
    os::OsSection(|st| {
        if let Some(ref mut os) = Os.borrow(st).borrow_mut().deref_mut() {
            let [ref mut a, ref mut b, ..] = os.tasks;
            if os.taskIdx == 0 {
                ContextSwitch(a, b);

                let stackIdx = 255 - ((((&a.stack[255]) as *const u32) as u32) - a.sp) / 4;

                a.stack[stackIdx as usize + 14] = (cyclic as *const ()) as u32;
                for stackOffset in 1..6 {
                    a.stack[stackIdx as usize + 14 - stackOffset as usize] = 0;
                }
                a.stack[stackIdx as usize + 14 - 6] = (a as *const Task) as u32;

                os.taskIdx = 1;
            } else {
                ContextSwitch(b, a);

                let stackIdx = 255 - ((((&b.stack[255]) as *const u32) as u32) - b.sp) / 4;

                b.stack[stackIdx as usize + 14] = (cyclic as *const ()) as u32;
                for stackOffset in 1..6 {
                    b.stack[stackIdx as usize + 14 - stackOffset as usize] = 0;
                }
                b.stack[stackIdx as usize + 14 - 6] = (b as *const Task) as u32;

                os.taskIdx = 0;
            }
        }
    });
}

pub fn ContextSwitch(task0: &mut Task, task1: &mut Task)
{
    let mut t0sp: u32 = ((&task0.sp) as *const u32) as u32;
    let mut t1sp: u32 = ((&task1.sp) as *const u32) as u32;
    unsafe {
        asm!("cpsid i");
        asm!("mrs	r0, psp");
        asm!("subs	r0, #16");
        asm!("stmia	r0!,{{r4-r7}}");
        asm!("mov	r4, r8");
        asm!("mov	r5, r9");
        asm!("mov	r6, r10");
        asm!("mov	r7, r11");
        asm!("subs	r0, #32");
        asm!("stmia	r0!,{{r4-r7}}");
        asm!("subs	r0, #16");

        asm!("str	r0, [r1]", inout("r1") t0sp);

        asm!("ldr	r0, [r1]", inout("r1") t1sp);
        
        asm!("ldmia	r0!,{{r4-r7}}");
        asm!("mov	r8, r4");
        asm!("mov	r9, r5");
        asm!("mov	r10, r6");
        asm!("mov	r11, r7");
        asm!("ldmia	r0!,{{r4-r7}}");
        asm!("msr	psp, r0");
        asm!("ldr r0, =0xFFFFFFFD");
        asm!("cpsie	i");
    }
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

//tskref: *const Task
fn cyclic(task: *const Task) {
    unsafe {
        ((*task).cyclic)(123);
    }
    loop { }
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
            os.tasks[0].cyclic = taskone;
            os.tasks[0].sp = ((&os.tasks[0].stack[256 - 16]) as *const u32) as u32;
            os.tasks[0].stack[256 - 1] = 0x01000000;
            os.tasks[0].stack[256 - 2] = (cyclic as *const ()) as u32;
            os.tasks[0].stack[256 - 8] = ((os.tasks.as_ptr() as usize) + (mem::size_of::<Task>() * 0)) as u32;

            os.tasks[1].cyclic = tasktwo;
            os.tasks[1].sp = ((&os.tasks[1].stack[256 - 16]) as *const u32) as u32;
            os.tasks[1].stack[256 - 1] = 0x01000000;
            os.tasks[1].stack[256 - 2] = (cyclic as *const ()) as u32;
            os.tasks[1].stack[256 - 8] = ((os.tasks.as_ptr() as usize) + (mem::size_of::<Task>() * 1)) as u32;

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


    cyclic(startTask);

    loop {}
}

#[panic_handler]
fn panic(_i: &PanicInfo) -> ! {
    loop {}
}

