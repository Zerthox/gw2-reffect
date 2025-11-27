mod build;
mod gear;
mod mount;
mod profession;
mod race;
mod resources;
mod specialization;

use crate::{
    context::skill::{BuffMap, Skillbar},
    error::Error,
};
use nexus::data_link::mumble::{Identity, MumblePtr};

pub use self::{
    build::*, gear::*, mount::*, profession::*, race::*, resources::*, specialization::*,
};

#[derive(Debug, Clone)]
pub struct PlayerInfo {
    /// Player race.
    pub race: Result<Race, u8>,

    /// Player profession.
    pub prof: Result<Profession, u8>,

    /// Player elite specialization.
    pub spec: Result<Specialization, u32>,

    /// Player mount.
    pub mount: Result<Mount, u8>,

    /// Player equipped gear.
    pub gear: Result<Gear, Error>,

    /// Player selected build.
    pub build: Result<Build, Error>,

    /// Player resources.
    pub resources: Result<PlayerResources, Error>,

    /// Player buffs.
    pub buff_info: Result<PlayerBuffInfo, Error>,

    /// Player skillbar.
    pub skillbar: Result<Skillbar, Error>,
}

impl PlayerInfo {
    #[inline]
    pub const fn empty() -> Self {
        Self {
            prof: Err(0),
            spec: Err(0),
            race: Err(0),
            mount: Err(0),
            gear: Err(Error::Disabled),
            build: Err(Error::Disabled),
            resources: Err(Error::Disabled),
            buff_info: Err(Error::Disabled),
            skillbar: Err(Error::Disabled),
        }
    }

    #[inline]
    pub fn update_fast(&mut self, mumble: MumblePtr) {
        self.mount = (mumble.read_mount_index() as u8).try_into();
    }

    #[inline]
    pub fn update_identity(&mut self, identity: Identity) {
        self.race = (identity.race as u8).try_into();
        self.prof = Profession::try_from(identity.profession as u8);
        self.spec = Specialization::try_from(self.prof.ok(), identity.spec).ok_or(identity.spec);
    }

    #[inline]
    pub fn set_error(&mut self, error: Error) {
        self.gear = Err(error.clone());
        self.build = Err(error.clone());
        self.resources = Err(error.clone());
        self.buff_info = Err(error.clone());
        self.skillbar = Err(error);
    }
}

#[derive(Debug, Clone, Default)]
pub struct PlayerBuffInfo {
    /// Current buffs.
    pub buffs: BuffMap,

    /// Last applied screen border.
    pub last_screen_border: u32,

    /// Last applied squad highlight.
    pub last_squad_highlight: u32,
}
