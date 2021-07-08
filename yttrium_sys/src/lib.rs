#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("Unity/IUnityInterface.h");
        include!("Unity/IUnityGraphics.h");
    }
}