use super::ffi;
use super::interface::{Interface, NativeInterface};
use std::error::Error;

unsafe impl NativeInterface for ffi::IUnityGraphics {}

pub struct GraphicsInterface {
    callbacks: Vec<ffi::UnityGraphicsDeviceEventCallback>,
    inner: *mut ffi::IUnityGraphics,
}

impl Interface<ffi::IUnityGraphics> for GraphicsInterface {
    fn new(instance: *mut ffi::IUnityGraphics) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized,
    {
        Ok(GraphicsInterface {
            callbacks: Vec::new(),
            inner: instance as *mut _,
        })
    }

    fn name() -> &'static str
    where
        Self: Sized,
    {
        "IUnityGraphics"
    }

    fn uuid() -> u128
    where
        Self: Sized,
    {
        0x7CBA0A9CA4DDB5448C5AD4926EB17B11
    }
}
