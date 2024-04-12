use uuid::Uuid;

// TODO: store parent chain and only display those during edit?

#[derive(Debug, Default, Clone)]
pub struct EditState {
    pub active: Uuid,
}

impl EditState {
    pub fn is_active(&self, id: Uuid) -> bool {
        self.active == id
    }

    pub fn select(&mut self, id: Uuid) {
        if self.active == id {
            self.active = Uuid::nil();
        } else {
            self.active = id;
        }
    }
}
