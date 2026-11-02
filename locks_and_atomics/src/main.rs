use std::{
    cell::UnsafeCell,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    sync::atomic::AtomicBool,
};

struct SpinLock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

struct Guard<'a, T> {
    lock: &'a mut SpinLock<T>,
    _value: PhantomData<&'a mut T>,
}

impl<'a, T> Deref for Guard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.value.get() }
    }
}

impl<'a, T> DerefMut for Guard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.value.get() }
    }
}

impl<'a, T> Drop for Guard<'a, T> {
    fn drop(&mut self) {
        self.lock
            .locked
            .store(false, std::sync::atomic::Ordering::Release);
    }
}

impl<T> SpinLock<T>
where
    T: Send,
{
    fn lock(&mut self) -> Guard<T> {
        while self.locked.swap(true, std::sync::atomic::Ordering::Acquire) {
            std::hint::spin_loop();
        }
        Guard {
            lock: self,
            _value: PhantomData,
        }
    }
}

fn main() {
    println!("Hello, world!");
}
