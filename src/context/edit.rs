use uuid::Uuid;

#[derive(Debug, Default, Clone)]
pub struct EditState {
    pub selected: Uuid,
    pub parent_pack: Uuid,
}

impl EditState {
    pub fn is_active(&self, id: Uuid) -> bool {
        self.selected == id
    }

    pub fn is_active_or_parent(&self, id: Uuid) -> bool {
        self.is_active(id) || self.parent_pack == id
    }
}
