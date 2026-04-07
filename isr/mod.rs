use core::ops::DerefMut;

use crate::{
    mcu::{
        Os,
        SYSTICK
    },
    os::task::TaskStatus
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
pub unsafe extern "C" fn PendSV() {
    if let Some(ref mut os) = Os.borrow().borrow_mut().deref_mut() {
        let tIdx = os.GetNextTask();
        os.ContextSwitch(os.taskIdx as usize, tIdx as usize);
        
        match os.tasks[os.taskIdx as usize].status {
            TaskStatus::Active => {
                os.tasks[os.taskIdx as usize].status = TaskStatus::Suspended;
            },
            TaskStatus::Finished => {
                os.ResetTask(os.taskIdx as usize);
                os.tasks[os.taskIdx as usize].status = TaskStatus::Ready;
            },
            _ => {}
        }

        os.tasks[tIdx as usize].status = TaskStatus::Active;
        os.taskIdx = tIdx as u32;
    }
}