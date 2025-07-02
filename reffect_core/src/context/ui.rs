use crate::links::Links;
use nexus::data_link::mumble::UiState;

#[derive(Debug, Clone)]
pub struct UiInfo {
    pub gameplay: bool,
    pub map_open: bool,
    pub combat: bool,
}

impl UiInfo {
    pub const fn empty() -> Self {
        Self {
            gameplay: false,
            map_open: false,
            combat: false,
        }
    }

    pub fn update(&mut self, links: &Links) {
        if let Some(nexus) = unsafe { links.nexus() } {
            self.gameplay = nexus.is_gameplay;
        }
        if let Some(mumble) = links.mumble() {
            let ui_state = mumble.read_ui_state();
            self.combat = ui_state.contains(UiState::IS_IN_COMBAT);
            self.map_open = ui_state.contains(UiState::IS_MAP_OPEN);
        }
    }

    pub fn should_show(&self) -> bool {
        self.gameplay && !self.map_open
    }
}
