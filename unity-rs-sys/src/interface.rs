use std::collections::hash_map::HashMap;
use std::rc::Rc;

use super::ffi;

pub unsafe trait NativeInterface {
    fn uuid() -> u128
    where
        Self: Sized,
    {
        return -1;
    }
}

unsafe impl NativeInterface for ffi::IUnityInterface {}

unsafe impl<T: NativeInterface> NativeInterface for *const T {}
unsafe impl<T: NativeInterface> NativeInterface for *mut T {}

pub trait Interface<I: NativeInterface + ?Sized> {
    fn new(instance: *mut I) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;

    fn name() -> &'static str
    where
        Self: Sized;
    fn uuid() -> u128
    where
        Self: Sized,
    {
        return I::uuid();
    }
}

#[allow(dead_code)]
pub struct InterfaceManager {
    interfaces: HashMap<u128, Rc<dyn Interface<dyn NativeInterface>>>,
    inner: *mut ffi::IUnityInterfaces,
}

impl InterfaceManager {
    pub(crate) fn new(i: *mut ffi::IUnityInterfaces) -> InterfaceManager {
        InterfaceManager {
            interfaces: HashMap::new(),
            inner: i,
        }
    }
    pub(crate) fn register<T, I: Interface<dyn NativeInterface> + Sized + 'static>(
        &mut self,
        instance: *mut T,
    ) -> Result<(), InterfaceError> {
        if self.interfaces.contains_key(&I::uuid()) {
            Err(InterfaceError::DuplicateInterface(I::name()))
        } else {
            let (high, low) = split(I::uuid());
            unsafe {
                ffi::IUnityInterfaces_RegisterInterface(self.inner, high, low, instance as *mut _)
            };
            self.load::<T, I>()
        }
    }

    pub(crate) fn get<T, I: Interface<dyn NativeInterface> + Sized + 'static>(
        &mut self,
    ) -> Result<&Rc<dyn Interface<dyn NativeInterface>>, InterfaceError> {
        if self.interfaces.contains_key(&I::uuid()) {
            Ok(self.interfaces.get(&I::uuid()).unwrap())
        } else {
            return match self.load::<T, I>() {
                Ok(_) => Ok(self.interfaces.get(&I::uuid()).unwrap()),
                Err(e) => Err(e),
            };
        }
    }

    pub(crate) fn get_mut<'a, T, I: Interface<dyn NativeInterface> + Sized + 'static>(
        &'a mut self,
    ) -> Result<&'a mut Rc<dyn Interface<dyn NativeInterface>>, InterfaceError> {
        if self.interfaces.contains_key(&I::uuid()) {
            Ok(self.interfaces.get_mut(&I::uuid()).unwrap())
        } else {
            return match self.load::<T, I>() {
                Ok(_) => Ok(self.interfaces.get_mut(&I::uuid()).unwrap()),
                Err(e) => Err(e),
            };
        }
    }
}

impl InterfaceManager {
    pub(super) fn load<T, I: Interface<dyn NativeInterface> + Sized + 'static>(
        &mut self,
    ) -> Result<(), InterfaceError> {
        let (high, low) = split(I::uuid());
        let ptr = unsafe { ffi::IUnityInterfaces_GetInterface(self.inner, high, low) };
        if !ptr.is_null() {
            match I::new(ptr.into_raw() as *mut _) {
                Ok(i) => {
                    // We can be assured that if this method is called, there will be no collision on interface IDs
                    let interface = Rc::new(i);
                    self.interfaces.insert(I::uuid(), interface);
                    Ok(())
                }
                Err(e) => Err(InterfaceError::LoadError(I::name(), e)),
            }
        } else {
            Err(InterfaceError::CannotLoad(I::name()))
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum InterfaceError {
    #[error("Interface already in use")]
    AlreadyInUse,
    #[error("Interface {0} not found")]
    NotFound(&'static str),
    #[error("Interface {0} either already registered or has duplicate UUID")]
    DuplicateInterface(&'static str),
    #[error("Interface {0} could not be loaded")]
    CannotLoad(&'static str),
    #[error("Error encountered while loading {0}: {1}")]
    LoadError(&'static str, Box<dyn std::error::Error>),
}

fn split(v: u128) -> (u64, u64) {
    (
        ((v & 0xFFFFFFFF00000000) >> 64) as u64,
        (v & 0x00000000FFFFFFFF) as u64,
    )
}
