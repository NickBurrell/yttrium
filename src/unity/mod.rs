#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("Unity/IUnityInterface.h");
        include!("Unity/IUnityGraphics.h");

        include!("unity_cxx_wrapper.h");
    }
}