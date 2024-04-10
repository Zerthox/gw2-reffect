mod mount;
mod profession;
mod race;
mod specialization;

pub use self::{mount::*, profession::*, race::*, specialization::*};

use nexus::data_link::mumble::MumbleLink;

#[derive(Debug, Clone)]
pub struct PlayerContext {
    pub prof: Profession,
    pub spec: Specialization,
    pub race: Race,
    pub mount: Mount,
}

impl PlayerContext {
    pub const fn empty() -> Self {
        Self {
            prof: Profession::Unknown,
            spec: Specialization::Unknown,
            race: Race::Unknown,
            mount: Mount::None,
        }
    }

    pub fn update_fast(&mut self, mumble: &MumbleLink) {
        self.mount = (mumble.context.mount_index as u8).into();
    }

    pub fn update_slow(&mut self, mumble: &MumbleLink) {
        // only attempt parse after first tick
        if mumble.ui_tick > 0 {
            match mumble.parse_identity() {
                Ok(identity) => {
                    self.prof = (identity.profession as u8).into();
                    self.spec = identity.spec.into();
                    self.race = (identity.race as u8).into();
                }
                Err(err) => log::error!("Failed to parse mumble identity: {err}"),
            }
        }
    }
}
