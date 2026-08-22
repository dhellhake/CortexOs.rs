pub mod queue;

#[derive(Copy, Clone)]
pub struct ReceiveResult<T: Copy> {
    pub message: Option<T>,
    pub lost: usize,
}

impl<T: Copy> ReceiveResult<T> {
    #[inline(always)]
    pub fn empty(lost: usize) -> Self {
        Self {
            message: None,
            lost,
        }
    }

    #[inline(always)]
    pub fn message(message: T, lost: usize) -> Self {
        Self {
            message: Some(message),
            lost,
        }
    }
}

#[derive(Copy, Clone, Default)]
pub struct FanoutReport {
    pub overwritten_mask: u32,
}
