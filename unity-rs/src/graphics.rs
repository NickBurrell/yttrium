use super::interface::Interface;
use crate::ffi;
use std::error::Error;

pub type GraphicsCallback = extern "stdcall" fn(UnityGraphicsEvent);

pub struct GraphicsInterface {
    callbacks: Vec<GraphicsCallback>,
    inner: *mut ffi::IUnityGraphics,
}

impl Interface for GraphicsInterface {
    fn new<T>(instance: *mut T) -> Result<Self, Box<dyn Error>>
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
