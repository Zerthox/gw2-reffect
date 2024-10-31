mod mount;
mod profession;
mod race;
mod specialization;
mod weapon;

pub use self::{mount::*, profession::*, race::*, specialization::*};

use crate::api::{Error, Interface, Internal};
use nexus::data_link::mumble::MumblePtr;
use reffect_internal::{PlayerInfo, Traits, Weapons};

#[derive(Debug, Clone)]
pub struct PlayerContext {
    pub race: Result<Race, u8>,
    pub prof: Result<Profession, u8>,
    pub spec: Result<Specialization, u32>,
    pub info: Result<PlayerInfo, Error>,
    pub mount: Result<Mount, u8>,
}

impl PlayerContext {
    pub fn empty() -> Self {
        Self {
            prof: Err(0),
            spec: Err(0),
            race: Err(0),
            info: Err(Error::default()),
            mount: Err(0),
        }
    }

    pub fn weapons(&self) -> Option<Weapons> {
        self.info.as_ref().ok().map(|info| info.weapons)
    }

    pub fn traits(&self) -> Option<&Traits> {
        self.info.as_ref().ok().map(|info| &info.traits)
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
                    self.info = Internal::get_player_info();
                }
                Err(err) => log::error!("Failed to parse mumble identity: {err}"),
            }
        }
    }
}
