use core::{
    arch::global_asm,
    ops::DerefMut
};

use crate::{
    mcu::{
        Os,
        SYSTICK
    }
};

#[no_mangle]
pub unsafe extern "C" fn SysTick_Isr() {
    SYSTICK.with(|syst| {
        syst.RollOver();
        let elapsed_us: u64 = syst.GetElapsedMicroseconds();

        if let Some(ref mut os) = Os.borrow().borrow_mut().deref_mut() {
            os.InvokeSchedule(elapsed_us);
        }
    });
}

#[no_mangle]
pub unsafe extern "C" fn SVCall() {
SYSTICK.with(|syst| {
    let elapsed_us: u64 = syst.GetElapsedMicroseconds();

    if let Some(ref mut os) = Os.borrow().borrow_mut().deref_mut() {
        os.InvokeSchedule(elapsed_us);
    }
});
}

#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn PendSVSelectNext(current_sp: u32) -> u32 {
    if let Some(ref mut os) = Os.borrow().borrow_mut().deref_mut() {
        os.PendSVSelectNext(current_sp)
    } else {
        current_sp
    }
}

global_asm!(r#"
    .section .text.PendSV, "ax", %progbits
    .global PendSV
    .type PendSV, %function
    .thumb_func

PendSV:
    mrs     r2, primask
    cpsid   i

    @ Save outgoing task software frame.
    @ Stack layout after this:
    @   [sp + 0]  r8
    @   [sp + 4]  r9
    @   [sp + 8]  r10
    @   [sp + 12] r11
    @   [sp + 16] r4
    @   [sp + 20] r5
    @   [sp + 24] r6
    @   [sp + 28] r7
    @   [sp + 32] hardware exception frame
    mrs     r0, psp
    subs    r0, #16
    stmia   r0!, {{r4-r7}}

    mov     r4, r8
    mov     r5, r9
    mov     r6, r10
    mov     r7, r11

    subs    r0, #32
    stmia   r0!, {{r4-r7}}
    subs    r0, #16

    @ r0 now contains the saved PSP of the outgoing task.
    @ Call Rust scheduler on MSP.
    @ Push 16 bytes to preserve 8-byte stack alignment and preserve EXC_RETURN.
    push    {{r1, r2, r3, lr}}
    bl      PendSVSelectNext
    pop     {{r1, r2, r3, r4}}
    mov     lr, r4

    @ r0 now contains the saved PSP of the incoming task.
    @ Restore incoming task software frame.
    ldmia   r0!, {{r4-r7}}
    mov     r8, r4
    mov     r9, r5
    mov     r10, r6
    mov     r11, r7

    ldmia   r0!, {{r4-r7}}
    msr     psp, r0

    msr     primask, r2
    bx      lr
"#);