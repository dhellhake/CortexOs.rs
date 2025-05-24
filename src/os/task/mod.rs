use super::STACK_SIZE;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Task 
{
    pub sp: u32,
	pub status: TaskStatus,
	pub cycletime: TaskCycleTime,
    pub cyclic: fn(u32),
    pub stack: [u32; STACK_SIZE],
}

#[derive(Copy, Clone, Debug)]
pub enum TaskStatus
{
	PreInit		= 0,
	Suspended	= 1,
	Ready		= 2,
	Active		= 3,
	Finished	= 4,
	Unknown		= 255,
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

impl Task {
	pub fn SetTimeStamp(&mut self, _tstmp: u32)
	{		
        self.stack[STACK_SIZE - 7] = _tstmp;
	}	
}