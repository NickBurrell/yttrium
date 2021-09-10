use std::cell::UnsafeCell;
use std::marker::PhantomData;
use std::ops::Deref;
use std::panic::{RefUnwindSafe, UnwindSafe};
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

pub struct NativeResource<'a, T> {
    _phantom_lifetime: PhantomData<&'a T>,
    inner: NativeResourceInner,
    data: UnsafeCell<T>,
}

unsafe impl<'a, T: Send> Send for NativeResource<'a, T> {}
unsafe impl<'a, T: Send + Sync> Sync for NativeResource<'a, T> {}

struct NativeResourceInner {
    mut_acq: AtomicBool,
    refs: AtomicU64,
}

impl NativeResourceInner {
    pub(crate) unsafe fn acq_shared(&self) -> Result<(), HandleError> {
        if self.try_shared() {
            Ok(())
        } else {
            Err(HandleError::AcquiredShared)
        }
    }

    pub(crate) unsafe fn acq_owned(&self) -> Result<(), HandleError> {
        if self.try_owned() {
            Ok(())
        } else {
            Err(HandleError::AcquiredOwned)
        }
    }

    pub(crate) unsafe fn try_shared(&self) -> bool {
        if self.mut_acq.load(Ordering::SeqCst) {
            false
        } else {
            self.refs.fetch_add(1, Ordering::SeqCst);
            true
        }
    }

    pub(crate) unsafe fn try_owned(&self) -> bool {
        if self.refs.load(Ordering::SeqCst) > 0 {
            false
        } else {
            self.mut_acq.store(true, Ordering::SeqCst);
            true
        }
    }

    pub(crate) unsafe fn free_shared(&self) -> Result<(), HandleError> {
        if self.mut_acq.load(Ordering::SeqCst) {
            Err(HandleError::FreeShared)
        } else {
            self.refs.fetch_sub(1, Ordering::SeqCst);
            Ok(())
        }
    }

    pub(crate) unsafe fn free_owned(&self) -> Result<(), HandleError> {
        if self.refs.load(Ordering::SeqCst) > 0 {
            Err(HandleError::FreeOwned)
        } else {
            self.mut_acq.store(false, Ordering::SeqCst);
            Ok(())
        }
    }
}

pub struct HandleShared<'a, 'ytt, T>(*const T, &'a NativeResource<'ytt, T>);

unsafe impl<'a, 'ytt, T> Send for HandleShared<'a, 'ytt, T> {}

impl<'a, 'ytt, T> UnwindSafe for HandleShared<'a, 'ytt, T> where T: RefUnwindSafe {}

impl<'a, 'ytt, T> HandleShared<'a, 'ytt, T> {
    unsafe fn new<'b>(
        resource: &'ytt NativeResource<T>,
    ) -> Result<HandleShared<'b, 'ytt, T>, HandleError> {
        match resource.inner.acq_shared() {
            Ok(_) => Ok(HandleShared(resource.data.get() as *const _, resource)),
            Err(e) => Err(e),
        }
    }
}

impl<'a, 'ytt, T> Deref for HandleShared<'a, 'ytt, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.0 }
    }
}

impl<'a, 'ytt, T> Drop for HandleShared<'a, 'ytt, T> {
    fn drop(&mut self) {
        unsafe {
            self.1.inner.free_shared().unwrap();
        }
    }
}

pub struct HandleOwned<'a, 'ytt, T>(Pin<Box<T>>, &'a NativeResource<'ytt, T>);

unsafe impl<'a, 'ytt, T> Send for HandleOwned<'a, 'ytt, T> {}

impl<'a, 'ytt, T> UnwindSafe for HandleOwned<'a, 'ytt, T> where T: RefUnwindSafe {}

impl<'a, 'ytt, T: Send> HandleOwned<'a, 'ytt, T> {
    unsafe fn new<'b>(
        resource: &'ytt NativeResource<T>,
    ) -> Result<HandleOwned<'b, 'ytt, T>, HandleError> {
        match resource.inner.acq_owned() {
            Ok(_) => Ok(HandleOwned(
                Pin::from(Box::from_raw(resource.data.get())),
                resource,
            )),
            Err(e) => Err(e),
        }
    }
}
/*
impl<'a, 'ytt, T> Deref for HandleOwned<'a, 'ytt, T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.0.
    }
}

impl<'a, 'ytt, T> DerefMut for HandleOwned<'a, 'ytt, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.0.as_mut()
    }
}

impl<'a, 'ytt, T> Drop for HandleOwned<'a, 'ytt, T> {
    fn drop(&mut self) {
        unsafe { self.1.inner.free_owned().unwrap(); }
    }
}
*/

#[derive(Debug, Copy, Clone, PartialEq, thiserror::Error)]
pub enum HandleError {
    #[error("Attempt to acquire shared handle when owned handle is acquired")]
    AcquiredShared,
    #[error("Attempt to acquire owned handle when shared handle is acquired")]
    AcquiredOwned,
    #[error("Attempt to free shared handle when owned handle is acquired")]
    FreeShared,
    #[error("Attempt to free owned handle when shared handle is acquired")]
    FreeOwned,
}
