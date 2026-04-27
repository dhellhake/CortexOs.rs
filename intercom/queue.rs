use core::mem::MaybeUninit;

pub struct Queue<T: Copy, const CAP: usize> {
    buf: [MaybeUninit<T>; CAP],
    read: usize,
    write: usize,
    lost: usize,
}

impl<T: Copy, const CAP: usize> Queue<T, CAP> {
    pub const fn new() -> Self {
        assert!(CAP != 0);
        assert!((CAP & (CAP - 1)) == 0);

        Self {
            buf: [MaybeUninit::<T>::uninit(); CAP],
            read: 0,
            write: 0,
            lost: 0,
        }
    }

    #[inline(always)]
    pub fn push_overwrite_oldest(&mut self, msg: T) -> bool {
        let full = self.write.wrapping_sub(self.read) == CAP;

        if full {
            self.read = self.read.wrapping_add(1);
            self.lost = self.lost.wrapping_add(1);
        }

        let index = self.write & (CAP - 1);

        self.buf[index].write(msg);
        self.write = self.write.wrapping_add(1);

        full
    }

    #[inline(always)]
    pub fn pop(&mut self) -> Option<T> {
        if self.read == self.write {
            return None;
        }

        let index = self.read & (CAP - 1);
        let msg = unsafe { self.buf[index].assume_init_read() };

        self.read = self.read.wrapping_add(1);

        Some(msg)
    }

    #[inline(always)]
    pub fn take_lost(&mut self) -> usize {
        let lost = self.lost;
        self.lost = 0;
        lost
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.write.wrapping_sub(self.read)
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.read == self.write
    }

    #[inline(always)]
    pub fn is_full(&self) -> bool {
        self.len() == CAP
    }
}