use std::marker::PhantomData;
use std::sync::atomic::{AtomicBool, Ordering};

use generational_arena::Arena;

use winit::dpi::Size;

use super::panel::Panel;

static YTTRIUM_IN_USE: AtomicBool = AtomicBool::new(false);

pub struct Yttrium {
    _not_send: PhantomData<*const ()>,
    panels: Arena<Panel>,
}

impl Yttrium {
    pub fn new() -> Result<Yttrium, YttriumError> {
        if !YTTRIUM_IN_USE
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .unwrap()
        {
            Ok(Yttrium {
                _not_send: PhantomData,
                panels: Arena::new(),
            })
        } else {
            Err(YttriumError::AlreadyInUse)
        }
    }

    pub fn new_panel(&mut self) -> PanelBuilder<'_> {
        PanelBuilder {
            yttrium: self,
            title: None,
            size: None,
        }
    }
}

impl Yttrium {
    pub(super) fn create_panel(
        &mut self,
        panel: Panel,
    ) -> Result<generational_arena::Index, YttriumError> {
        let id = self.panels.insert(panel);
        self.panels.get_mut(id).unwrap().set_id(id);
        Ok(id)
    }
}

pub struct PanelBuilder<'ytt> {
    yttrium: &'ytt mut Yttrium,
    title: Option<String>,
    size: Option<Size>,
}

impl<'ytt> PanelBuilder<'ytt> {
    pub fn with_title<S: AsRef<str>>(&mut self, title: S) -> &mut Self {
        self.title = Some(title.as_ref().to_owned());
        self
    }

    pub fn with_size<S: Into<Size>>(&mut self, size: S) -> &mut Self {
        self.size = Some(size.into());
        self
    }

    pub fn build(self) -> Result<&'ytt mut Panel, YttriumError> {
        if let Ok(idx) = self.yttrium.create_panel(Panel { id: None }) {
            if let Some(panel) = self.yttrium.panels.get_mut(idx) {
                Ok(panel)
            } else {
                Err(YttriumError::PanelCreationError)
            }
        } else {
            Err(YttriumError::PanelCreationError)
        }
    }
}

impl Drop for Yttrium {
    fn drop(&mut self) {
        YTTRIUM_IN_USE.store(false, Ordering::SeqCst);
    }
}

#[derive(Debug, Copy, Clone, PartialEq, thiserror::Error)]
pub enum YttriumError {
    #[error("Yttrium is already in use")]
    AlreadyInUse,
    #[error("Failed to create panel")]
    PanelCreationError,
}
