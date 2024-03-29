use super::MapInfo;
use nexus::data_link::mumble::{MumbleLink, Profession};

#[derive(Debug, Clone)]
pub struct PlayerContext {
    pub prof: Profession,
    pub spec: u32,
    pub map: MapInfo,
}

impl PlayerContext {
    pub const fn empty() -> Self {
        Self {
            prof: Profession::Unknown,
            spec: 0,
            map: MapInfo::empty(),
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
        self.map.update(&mumble.context);
    }

    pub fn is_on_map(&self, id: u32) -> bool {
        self.map.id == id
    }
}
