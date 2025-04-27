use std::{
    cell::UnsafeCell,
    sync::atomic::{AtomicBool, Ordering},
};

use super::guard::Guard;

pub struct SpinLock<T> {
    pub(super) locked: AtomicBool,
    pub(super) value: UnsafeCell<T>,
}

unsafe impl<T> Sync for SpinLock<T> where T: Send {}

impl<T> SpinLock<T> {
    pub const fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    pub fn lock(&self) -> Guard<T> {
        while self.locked.swap(true, Ordering::Acquire) {
            std::hint::spin_loop();
        }
        Guard { lock: self }
    }
}
