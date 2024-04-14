use nexus::data_link::mumble::{map_type, MumblePtr};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIter, IntoStaticStr, VariantArray};

#[derive(Debug, Clone)]
pub struct MapContext {
    pub id: u32,
    pub category: MapCategory,
}

impl MapContext {
    pub const fn empty() -> Self {
        Self {
            id: 0,
            category: MapCategory::Unknown,
        }
    }

    pub fn update(&mut self, mumble: MumblePtr) {
        self.id = mumble.read_map_id();
        self.category = mumble.read_map_type().into();
    }

    pub fn is_on_map(&self, id: u32) -> bool {
        self.id == id
    }
}

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    AsRefStr,
    IntoStaticStr,
    Display,
    EnumIter,
    VariantArray,
    Serialize,
    Deserialize,
)]
pub enum MapCategory {
    #[default]
    Pve,
    Pvp,
    Wvw,
    Instance,
    Unknown,
}

impl From<u32> for MapCategory {
    fn from(value: u32) -> Self {
        match value {
            map_type::PVE | map_type::PVE_MINI => Self::Pve,

            map_type::PVP | map_type::USER_TOURNAMENT | map_type::BIG_BATTLE => Self::Pvp,

            map_type::INSTANCE => Self::Instance,

            map_type::GVG
            | map_type::WVW_ETERNAL_BATTLEGROUNDS
            | map_type::WVW_BLUE_BORDERLANDS
            | map_type::WVW_GREEN_BORDERLANDS
            | map_type::WVW_RED_BORDERLANDS
            | map_type::WVW_REWARD
            | map_type::WVW_OBSIDIAN_SANCTUM
            | map_type::WVW_EDGE_OF_THE_MISTS
            | map_type::WVW_LOUNGE => Self::Wvw,

            _ => Self::Unknown,
        }
    }
}
