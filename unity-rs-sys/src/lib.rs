#![feature(const_generics)]

use cxx::type_id;
use cxx::ExternType;
use std::marker::PhantomData;

pub mod gfx;
pub mod interface;
pub mod profiler;

extern "Rust" {
    fn __unity_rs_entry_point(lib: &mut Unity);
}

pub struct Unity {
    _not_send: PhantomData<*const ()>,
    instances: interface::InterfaceManager,
}

impl Unity {
    fn new(interfaces: *mut ffi::IUnityInterfaces) -> Unity {
        Unity {
            _not_send: PhantomData,
            instances: interface::InterfaceManager::new(interfaces),
        }
    }
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("Unity/IUnityInterface.h");
        pub(crate) type IUnityInterfaces;
        pub(crate) type IUnityInterface;

        include!("IUnityInterface_shim.h");

        // IUnityInterface.h ffi methods
        pub(crate) unsafe fn IUnityInterfaces_GetInterface(
            interfaces: *const IUnityInterfaces,
            high: u64,
            low: u64,
        ) -> UniquePtr<IUnityInterface>;
        pub(crate) unsafe fn IUnityInterfaces_RegisterInterface(
            interfaces: *const IUnityInterfaces,
            high: u64,
            low: u64,
            interface: *mut IUnityInterface,
        );
    }
}

#[cfg(target_os = "windows")]
#[allow(non_snake_case)]
pub extern "stdcall" fn UnityPluginLoad(interfaces: *mut ffi::IUnityInterfaces) {
    let mut lib = Unity::new(interfaces);
    unsafe {
        __unity_rs_entry_point(&mut lib);
    }
}

#[cfg(not(target_os = "windows"))]
#[allow(non_snake_case)]
pub extern "C" fn UnityPluginLoad(interfaces: *mut ffi::IUnityInterfaces) {
    let mut lib = Unity::new(interfaces);
    unsafe {
        __unity_rs_entry_point(&mut lib);
    }
}
