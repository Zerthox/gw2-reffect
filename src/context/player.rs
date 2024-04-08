use crate::player::{Profession, Specialization};
use nexus::data_link::mumble::MumbleLink;

// TODO: race, mount?

#[derive(Debug, Clone)]
pub struct PlayerContext {
    pub prof: Profession,
    pub spec: Specialization,
}

impl PlayerContext {
    pub const fn empty() -> Self {
        Self {
            prof: Profession::Unknown,
            spec: Specialization::Unknown,
        }
    }

    pub fn update(&mut self, mumble: &MumbleLink) {
        // only attempt parse after first tick
        if mumble.ui_tick > 0 {
            match mumble.parse_identity() {
                Ok(identity) => {
                    self.prof = (identity.profession as u8).into();
                    self.spec = identity.spec.into();
                }
                Err(err) => log::warn!("Failed to parse mumble identity: {err}"),
            }
        }
    }
}
