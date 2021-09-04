use threadbound::ThreadBound;
use std::panic::{RefUnwindSafe, UnwindSafe};
use std::ops::{Deref, DerefMut};

#[repr(transparent)]
pub struct HandleOwned1<T>(*mut ThreadBound<T>) where T: ?Sized;

#[repr(transparent)]
pub struct HandleShared1<T: ?Sized>(*const T);

unsafe impl<T> Send for HandleOwned1<T> where T: ?Sized + RefUnwindSafe {}
unsafe impl<T> Send for HandleShared1<T> where T: ?Sized + Sync {}

impl<T> UnwindSafe for HandleShared1<T> where T: ?Sized + RefUnwindSafe {}

impl<T> HandleOwned1<T> where T: Send + 'static {
    fn alloc(value: T) -> Self {
        let b = box ThreadBound::new(value);
        HandleOwned1(Box::into_raw(b))
    }
}

impl<T> HandleShared1<T> where T: Send + Sync + 'static {
    fn alloc(value: T) -> Self {
        HandleShared1(Box::into_raw(box v))
    }
}

impl<T> HandleOwned1<T> where T: ?Sized + Send {
    unsafe fn dealloc<R>(handle: Self, f: impl FnOnce(&mut T) -> R) -> R {
        let mut v = Box::from_raw(handle.0);
        f(&mut **v)
    }
}

impl<T> HandleShared1<T> where T: ?Sized + Send + Sync {
    unsafe fn dealloc<R>(handle: Self, f: impl FnOnce(&mut T) -> R) -> R {
        let mut v = Box::from_raw(handle.0 as *mut T);
        f(&mut *v)
    }
}

impl<T> Deref for HandleOwned1<T> where T: ?Sized {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe {
            &**self.0
        }
    }
}

impl<T> DerefMut for HandleOwned1<T> where T: ?Sized {
    fn deref_mut(&mut self) -> &mut T {
        unsafe {
            &mut **self.0
        }
    }
}

