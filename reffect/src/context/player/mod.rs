mod mount;
mod profession;
mod race;
mod specialization;

pub use self::{mount::*, profession::*, race::*, specialization::*};

use crate::api::{Internal, Error, Interface, Traits};
use nexus::data_link::mumble::MumblePtr;

#[derive(Debug, Clone)]
pub struct PlayerContext {
    pub race: Result<Race, u8>,
    pub prof: Result<Profession, u8>,
    pub spec: Result<Specialization, u32>,
    pub traits: Result<Traits, Error>,
    pub mount: Result<Mount, u8>,
}

impl PlayerContext {
    pub fn empty() -> Self {
        Self {
            prof: Err(0),
            spec: Err(0),
            race: Err(0),
            traits: Err(Error::default()),
            mount: Err(0),
        }
    }

    pub fn update_fast(&mut self, mumble: MumblePtr) {
        self.mount = (mumble.read_mount_index() as u8).try_into();
    }

    pub fn update_slow(&mut self, mumble: MumblePtr) {
        // only attempt update after first tick
        if mumble.read_ui_tick() > 0 {
            match mumble.parse_identity() {
                Ok(identity) => {
                    self.race = (identity.race as u8).try_into();
                    self.prof = Profession::try_from(identity.profession as u8);
                    self.spec = Specialization::try_from(self.prof.ok(), identity.spec)
                        .ok_or(identity.spec);

                    let player_info = Internal::get_player_info();
                    self.traits = player_info.map(|info| info.traits);
                }
                Err(err) => log::error!("Failed to parse mumble identity: {err}"),
            }
        }
    }
}
