use core::{arch::asm, ops::DerefMut};

use crate::{cortex, SCB};

use super::{task::Task, Os, OsSection};

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
    OsSection(|st| {
        if let Some(ref mut os) = Os.borrow(st).borrow_mut().deref_mut() {
            let [ref mut a, ref mut b, ..] = os.tasks;
            if os.taskIdx == 0 {
                ContextSwitch(a, b);
                os.ResetTask(0);

                os.taskIdx = 1;
            } else {
                ContextSwitch(b, a);
                os.ResetTask(1);

                os.taskIdx = 0;
            }
        }
    });
}

fn ContextSwitch(curTask: &mut Task, setTask: &mut Task)
{
    let mut t0sp: u32 = ((&curTask.sp) as *const u32) as u32;
    let mut t1sp: u32 = ((&setTask.sp) as *const u32) as u32;
    unsafe {
        asm!(
            "cpsid i",
            "mrs	r0, psp",
            "subs	r0, #16",
            "stmia	r0!,{{r4-r7}}",
            "mov	r4, r8",
            "mov	r5, r9",
            "mov	r6, r10",
            "mov	r7, r11",
            "subs	r0, #32",
            "stmia	r0!,{{r4-r7}}",
            "subs	r0, #16",
            "str	r0, [r1]",
            inout("r1") t0sp,
        );

        asm!(
            "ldr	r0, [r1]",
            "ldmia	r0!,{{r4-r7}}",
            "mov	r8, r4",
            "mov	r9, r5",
            "mov	r10, r6",
            "mov	r11, r7",
            "ldmia	r0!,{{r4-r7}}",
            "msr	psp, r0",
            "ldr r0, =0xFFFFFFFD",
            "cpsie	i",
            inout("r1") t1sp,
        );
    }
}