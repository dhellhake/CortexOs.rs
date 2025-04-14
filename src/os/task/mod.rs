use super::STACK_SIZE;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Task 
{
    pub sp: u32,
	pub status: TaskStatus,
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