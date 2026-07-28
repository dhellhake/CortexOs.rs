#[repr(align(8))]
#[derive(Copy, Clone, Debug)]
pub struct Stack<const STACK_SIZE: usize>(pub [u32; STACK_SIZE]);

impl<const STACK_SIZE: usize> Stack<STACK_SIZE> {
    #[inline]
    pub const fn new() -> Self {
        Self([0; STACK_SIZE])
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Task<const STACK_SIZE: usize>
{
    pub sp: u32,
    pub status: TaskStatus,
    pub cycletime: TaskCycleTime,
    pub id: u32,
    pub cyclic: fn(u64),
    pub role: TaskRole,
    pub timestamp_us: u64,
	pub next_release_us: u64,
	pub missed_releases: u32,
    pub stack: Stack<STACK_SIZE>,
}

#[derive(Copy, Clone, Debug)]
pub enum TaskStatus
{
	PreInit		= 0,
	Suspended	= 1,
	Pending		= 2,
	Ready		= 3,
	Active		= 4,
	Finished	= 5,
	Unknown		= 255,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TaskRole
{
	Supervised		= 0,
	Unsupervised	= 1,
	Background		= 2,
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
pub enum TaskCycleTime
{
	NonCyclic	= 0,
	_5MS		= 5,
	_10MS		= 10,
	_20MS		= 20,
	_50MS		= 50,
	_100MS		= 100,
	Unknown		= 255,
}

impl TaskCycleTime {
    pub const fn period_us(self) -> Option<u64> {
        match self {
            TaskCycleTime::NonCyclic | TaskCycleTime::Unknown => None,
            _ => Some(self as u64 * 1000),
        }
    }
}

pub fn empty(_tstmp: u64) {
    loop {}
}
