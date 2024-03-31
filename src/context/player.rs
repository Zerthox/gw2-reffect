use super::MapInfo;
use nexus::data_link::{
    mumble::{Context, MumbleLink, Profession, UiState},
    read_nexus_link,
};

// TODO: race, mount?

#[derive(Debug, Clone)]
pub struct PlayerContext {
    pub prof: Profession,
    pub spec: u32,
    pub combat: bool,
    pub gameplay: bool,
    pub map_open: bool,
    pub map: MapInfo,
}

impl PlayerContext {
    pub const fn empty() -> Self {
        Self {
            prof: Profession::Unknown,
            spec: 0,
            combat: false,
            gameplay: false,
            map_open: false,
            map: MapInfo::empty(),
        }
    }

    pub fn update(&mut self, mumble: &MumbleLink) {
        match read_nexus_link() {
            Some(link) => self.gameplay = link.is_gameplay,
            None => log::warn!("Failed to read nexus link"),
        }
        match mumble.parse_identity() {
            Ok(identity) => {
                self.prof = identity.profession;
                self.spec = identity.spec;
            }
            Err(err) => log::warn!("Failed to parse mumble identity: {err}"),
        }
        let Context { ui_state, .. } = mumble.context;
        self.combat = ui_state.contains(UiState::IS_IN_COMBAT);
        self.map_open = ui_state.contains(UiState::IS_MAP_OPEN);

        self.map.update(&mumble.context);
    }

    pub fn is_on_map(&self, id: u32) -> bool {
        self.map.id == id
    }

    pub fn should_render(&self) -> bool {
        self.gameplay && !self.map_open
    }
}
