use generational_arena::Index;

pub struct Panel {
    pub(crate) id: Option<Index>,
}

impl Panel {
    pub(super) fn set_id(&mut self, idx: Index) {
        self.id = Some(idx);
    }
}
