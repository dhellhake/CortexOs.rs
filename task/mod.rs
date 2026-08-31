use core::{cell::UnsafeCell, marker::PhantomPinned};

pub(super) const INITIAL_FRAME_WORDS: usize = 16;
pub(super) const STACK_GUARD_WORDS: usize = 4;

pub type TaskFunction = extern "C" fn(u64);

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub(super) struct TaskControl {
    pub(super) sp: u32,
    pub(super) status: TaskStatus,
    pub(super) cycletime: TaskCycleTime,
    pub(super) id: u32,
    pub(super) cyclic: Option<TaskFunction>,
    pub(super) role: TaskRole,
    pub(super) timestamp_us: u64,
    pub(super) next_release_us: u64,
    pub(super) missed_releases: u32,
}

impl TaskControl {
    const fn new() -> Self {
        Self {
            sp: 0,
            status: TaskStatus::PreInit,
            cycletime: TaskCycleTime::NonCyclic,
            id: 0,
            cyclic: None,
            role: TaskRole::Supervised,
            timestamp_us: 0,
            next_release_us: 0,
            missed_releases: 0,
        }
    }
}

/// A statically pinned scheduler task that owns its stack and control state.
///
/// Both mutable regions use `UnsafeCell`: the processor may actively use
/// `stack` through PSP while the scheduler updates the disjoint `control`
/// block through a scheduler-held handle. No API creates `&mut Task` or a
/// reference to the stack storage.
#[repr(C, align(8))]
pub struct Task<const STACK_SIZE: usize> {
    // Keep the stack first so the task address is also its aligned stack base.
    stack: UnsafeCell<[u32; STACK_SIZE]>,
    control: UnsafeCell<TaskControl>,
    _pin: PhantomPinned,
}

impl<const STACK_SIZE: usize> Task<STACK_SIZE> {
    #[inline]
    pub const fn new() -> Self {
        assert!(STACK_SIZE >= INITIAL_FRAME_WORDS + STACK_GUARD_WORDS);
        assert!((STACK_SIZE & 1) == 0);

        Self {
            stack: UnsafeCell::new([0; STACK_SIZE]),
            control: UnsafeCell::new(TaskControl::new()),
            _pin: PhantomPinned,
        }
    }

    /// Creates an inert handle to this statically allocated task.
    ///
    /// The `'static` receiver makes the task's address stable before its raw
    /// stack and control pointers can be registered with a `Scheduler`.
    #[inline]
    pub const fn handle(&'static self) -> TaskHandle {
        TaskHandle {
            stack: self.stack.get().cast::<u32>(),
            stack_size_words: STACK_SIZE,
            control: self.control.get(),
        }
    }
}

impl<const STACK_SIZE: usize> Default for Task<STACK_SIZE> {
    fn default() -> Self {
        Self::new()
    }
}

// All mutation is behind private UnsafeCell/raw-pointer boundaries. The owning
// Scheduler serializes control access with PRIMASK, and stack memory is
// written only while that task is inactive. NMI must not access scheduler data.
unsafe impl<const STACK_SIZE: usize> Sync for Task<STACK_SIZE> {}

/// Type-erased handle used by `Scheduler`, allowing each task to select its
/// own compile-time stack size.
#[derive(Copy, Clone)]
pub struct TaskHandle {
    stack: *mut u32,
    stack_size_words: usize,
    control: *mut TaskControl,
}

impl TaskHandle {
    #[inline]
    pub(super) const fn stack_ptr(&self) -> *mut u32 {
        self.stack
    }

    #[inline]
    pub(super) const fn stack_size_words(&self) -> usize {
        self.stack_size_words
    }

    #[inline]
    pub(super) const fn control_ptr(&self) -> *mut TaskControl {
        self.control
    }
}

// TaskHandle exposes no public pointer access. Dereferencing is confined to the
// uniquely owning Scheduler established by its unsafe constructor contract.
unsafe impl Send for TaskHandle {}
unsafe impl Sync for TaskHandle {}

#[repr(u8)]
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

#[repr(u8)]
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

/// Immutable task metadata exposed without granting access to the task's
/// control block or live stack.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct TaskConfiguration {
    pub id: u32,
    pub cycletime: TaskCycleTime,
    pub role: TaskRole,
}

impl TaskConfiguration {
    #[inline]
    pub const fn new(id: u32, cycletime: TaskCycleTime, role: TaskRole) -> Self {
        Self {
            id,
            cycletime,
            role,
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TaskCycleTime {
    NonCyclic = 0,
    _1MS = 1,
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
