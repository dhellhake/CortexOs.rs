use core::ops::DerefMut;

use crate::{cortex, SCB};

use super::{task::TaskStatus, Os, OsSection};

#[no_mangle]
pub unsafe extern "C" fn SysTick_Isr() {  
    cortex::CriticalSection(|| {
        unsafe {
            if let Some(ref mut scb) = SCB.borrow().as_mut_unchecked() {
                scb.Set_PendSV();
            }
        }
    });
}

#[no_mangle]
pub unsafe extern "C" fn PendSV() {
    OsSection(|| {
        if let Some(ref mut os) = Os.borrow().borrow_mut().deref_mut() {
            for tIdx in 0..os.tasks.len() {
                match os.tasks[tIdx].status {
                    TaskStatus::Finished | TaskStatus::Ready => {
                        if tIdx != os.taskIdx as usize {
                            os.ContextSwitch(os.taskIdx as usize, tIdx);

                            if let TaskStatus::Finished = os.tasks[os.taskIdx as usize].status {
                                os.ResetTask(os.taskIdx as usize);
                                os.tasks[os.taskIdx as usize].status = TaskStatus::Ready;
                            }

                            os.tasks[tIdx].status = TaskStatus::Active;
                            os.taskIdx = tIdx as u32;
                            break;
                        }
                    }
                    _ => {}
                }
            }
        }
    });
}