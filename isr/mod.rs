use core::ops::DerefMut;

use crate::drv::{
    cortex::CriticalSection,
    scb::{
        SCB,
        PENDSVSET
    }
};
use super::{
    Os,
    task::TaskStatus,
    OsStatus
};

#[no_mangle]
#[unsafe(link_section = ".ramfunc")]
pub unsafe extern "C" fn SysTick_Isr() {  
    CriticalSection(|| {
        let scb = SCB.borrow().as_mut_unchecked().as_mut().unwrap();
        scb.Set_ICSR_PENDSVSET(PENDSVSET::VALUE_1);
    });
}

#[no_mangle]
#[unsafe(link_section = ".ramfunc")]
pub unsafe extern "C" fn PendSV() {
    if let Some(ref mut os) = Os.borrow().borrow_mut().deref_mut() {
        os.elapsedMillis += 1;

        if let OsStatus::Running = os.osStatus
        {
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
                            os.tasks[tIdx].SetTimeStamp(os.elapsedMillis as u32);
                            
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
        }
    }
}