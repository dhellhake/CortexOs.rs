use core::cell::UnsafeCell;

/// Stack memory lives outside `Application`, so an interrupt-time mutable
/// borrow of scheduler metadata never encompasses a task's live PSP frames.
#[repr(align(8))]
pub struct TaskStackStorage<const TASK_COUNT: usize, const STACK_SIZE: usize> {
    words: UnsafeCell<[[u32; STACK_SIZE]; TASK_COUNT]>,
}

impl<const TASK_COUNT: usize, const STACK_SIZE: usize> TaskStackStorage<TASK_COUNT, STACK_SIZE> {
    #[inline]
    pub const fn new() -> Self {
        Self {
            words: UnsafeCell::new([[0; STACK_SIZE]; TASK_COUNT]),
        }
    }

    /// Returns one stack as a raw pointer without creating a Rust reference to
    /// memory that can later become the processor's active stack.
    ///
    /// # Safety
    ///
    /// `task_idx` must be in range, and the caller must ensure that the chosen
    /// stack is not executing while it is written.
    #[inline]
    pub(crate) unsafe fn stack_ptr(&self, task_idx: usize) -> *mut u32 {
        debug_assert!(task_idx < TASK_COUNT);
        unsafe { self.words.get().cast::<u32>().add(task_idx * STACK_SIZE) }
    }
}

impl<const TASK_COUNT: usize, const STACK_SIZE: usize> Default
    for TaskStackStorage<TASK_COUNT, STACK_SIZE>
{
    fn default() -> Self {
        Self::new()
    }
}

// Mutation is available only through the unsafe raw-stack boundary above.
// Live task stack memory is never exposed as a Rust reference.
unsafe impl<const TASK_COUNT: usize, const STACK_SIZE: usize> Sync
    for TaskStackStorage<TASK_COUNT, STACK_SIZE>
{
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Task {
    pub sp: u32,
    pub status: TaskStatus,
    pub cycletime: TaskCycleTime,
    pub id: u32,
    pub cyclic: extern "C" fn(u64),
    pub role: TaskRole,
    pub timestamp_us: u64,
    pub next_release_us: u64,
    pub missed_releases: u32,
}

#[derive(Copy, Clone, Debug)]
pub enum TaskStatus {
    PreInit = 0,
    Suspended = 1,
    Pending = 2,
    Ready = 3,
    Active = 4,
    Finished = 5,
    Unknown = 255,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TaskRole {
    Supervised = 0,
    Unsupervised = 1,
    Background = 2,
}

impl TaskRole {
    #[inline]
    pub const fn ReportsProgramFlowCheckpoints(self, cycletime: TaskCycleTime) -> bool {
        matches!(self, TaskRole::Supervised)
            && !matches!(cycletime, TaskCycleTime::NonCyclic | TaskCycleTime::Unknown)
    }

    #[inline]
    pub const fn IsUnsupervised(self) -> bool {
        matches!(self, TaskRole::Unsupervised)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum TaskCycleTime {
    NonCyclic = 0,
    _5MS = 5,
    _10MS = 10,
    _20MS = 20,
    _50MS = 50,
    _100MS = 100,
    Unknown = 255,
}

impl TaskCycleTime {
    pub const fn period_us(self) -> Option<u64> {
        match self {
            TaskCycleTime::NonCyclic | TaskCycleTime::Unknown => None,
            _ => Some(self as u64 * 1000),
        }
    }
}

pub extern "C" fn empty(_tstmp: u64) {
    loop {
        core::hint::spin_loop();
    }
}
