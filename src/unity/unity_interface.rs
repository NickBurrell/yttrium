use std::collections::hash_map::{HashMap};

use super::ffi;

pub trait Interface {
    fn name() -> &'static str;
    fn guid() -> u128;
}

pub struct InterfaceManager {
    interfaces: HashMap<u128, dyn Interface>
}

impl InterfaceManager {
    pub(crate) fn get_interface<I: Interface>(&mut self) -> Result<&mut I, InterfaceError> {
        if let Some(interface) = self.interfaces.get_mut(&I::guid()) {
            Ok(interface)
        } else {
            Err(InterfaceError::InterfaceNotFound(I::name()))
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, thiserror::Error)]
pub enum InterfaceError {
    #[error("Interface already in use")]
    AlreadyInUse,
    #[error("Interface {0} not found")]
    NotFound(&'static str),
}