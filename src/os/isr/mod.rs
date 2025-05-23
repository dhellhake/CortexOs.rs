use core::ops::DerefMut;

use crate::{cortex, peripherals::scb::SCB};

use super::{task::TaskStatus, Os, OsSection, OsStatus};

#[no_mangle]
#[unsafe(link_section = ".ramfunc")]
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
#[unsafe(link_section = ".ramfunc")]
pub unsafe extern "C" fn PendSV() {
    OsSection(|| {
        if let Some(ref mut os) = Os.borrow().borrow_mut().deref_mut() {
            os.elapsedMillis += 1;

            if let OsStatus::Running = os.osStatus
            {
                for tIdx in 0..os.tasks.len() {
                    match os.tasks[tIdx].status {
                        TaskStatus::Active => {
                            break;
                        }
                        TaskStatus::Ready | TaskStatus::Suspended => {
                            if tIdx == os.taskIdx as usize {
                                os.tasks[tIdx].status = TaskStatus::Active;
                                break;
                            } else {
                                os.ContextSwitch(os.taskIdx as usize, tIdx);
                                
                                if let TaskStatus::Active = os.tasks[os.taskIdx as usize].status {
                                    os.tasks[os.taskIdx as usize].status = TaskStatus::Suspended;
                                }

                                os.tasks[tIdx].status = TaskStatus::Active;
                                os.taskIdx = tIdx as u32;
                                break;
                            }
                        }
                        _ => {}
                    }
                }

                for tIdx in 0..os.tasks.len() {
                    match os.tasks[tIdx].status {
                        TaskStatus::Finished => {            
                            if tIdx != os.taskIdx as usize {         
                                if os.elapsedMillis % (os.tasks[tIdx].cycletime as u64) == 0 {
                                    os.ResetTask(tIdx);
                                    os.tasks[tIdx].status = TaskStatus::Ready;
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    });
}