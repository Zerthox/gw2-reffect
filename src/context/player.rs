use super::MapInfo;
use nexus::data_link::mumble::{Context, MumbleLink, Profession, UiState};

// TODO: race, mount?

#[derive(Debug, Clone)]
pub struct PlayerContext {
    pub prof: Profession,
    pub spec: u32,
    pub in_combat: bool,
    pub map_open: bool,
    pub map: MapInfo,
}

impl PlayerContext {
    pub const fn empty() -> Self {
        Self {
            prof: Profession::Unknown,
            spec: 0,
            map: MapInfo::empty(),
            in_combat: false,
            map_open: false,
        }
    }

    pub fn update(&mut self, mumble: &MumbleLink) {
        match mumble.parse_identity() {
            Ok(identity) => {
                self.prof = identity.profession;
                self.spec = identity.spec;
            }
            Err(err) => log::warn!("Failed to parse mumble identity: {err}"),
        }
        let Context { ui_state, .. } = mumble.context;
        self.in_combat = ui_state.contains(UiState::IS_IN_COMBAT);
        self.map_open = ui_state.contains(UiState::IS_MAP_OPEN);

        self.map.update(&mumble.context);
    }

    pub fn is_on_map(&self, id: u32) -> bool {
        self.map.id == id
    }
}
