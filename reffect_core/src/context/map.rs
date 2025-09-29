use crate::{
    colors::{Color, Colored},
    named::Named,
};
use enumflags2::{BitFlags, bitflags};
use nexus::data_link::mumble::{MumblePtr, map_type};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIter, IntoStaticStr, VariantArray};

#[derive(Debug, Clone)]
pub struct MapInfo {
    pub id: u32,
    pub category: MapCategory,
}

impl MapInfo {
    #[inline]
    pub const fn empty() -> Self {
        Self {
            id: 0,
            category: MapCategory::Other,
        }
    }

    pub fn update(&mut self, mumble: MumblePtr) -> bool {
        let id = mumble.read_map_id();
        let map_type = mumble.read_map_type();
        let new = self.id != id;

        self.id = id;
        self.category = MapCategory::new(id, map_type);

        new
    }

    #[inline]
    pub fn is_on_map(&self, id: u32) -> bool {
        self.id == id
    }
}

// TODO: custom category for raids, fractals, strikes based on map id?
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
#[repr(u8)]
#[bitflags]
pub enum MapCategory {
    #[default]
    #[serde(alias = "Pve")]
    PvE = 1 << 0,

    #[serde(alias = "Pvp")]
    PvP = 1 << 1,

    #[serde(alias = "Wvw")]
    WvW = 1 << 2,

    Instance = 1 << 3,

    #[serde(alias = "Unknown")]
    Other = 1 << 4,
}

impl MapCategory {
    pub fn new(_id: u32, category: u32) -> Self {
        match category {
            map_type::PVE | map_type::PVE_MINI => Self::PvE,

            map_type::PVP | map_type::USER_TOURNAMENT | map_type::BIG_BATTLE => Self::PvP,

            map_type::INSTANCE => Self::Instance,

            map_type::GVG
            | map_type::WVW_ETERNAL_BATTLEGROUNDS
            | map_type::WVW_BLUE_BORDERLANDS
            | map_type::WVW_GREEN_BORDERLANDS
            | map_type::WVW_RED_BORDERLANDS
            | map_type::WVW_REWARD
            | map_type::WVW_OBSIDIAN_SANCTUM
            | map_type::WVW_EDGE_OF_THE_MISTS
            | map_type::WVW_LOUNGE => Self::WvW,

            _ => Self::Other,
        }
    }

    pub fn non_competitive() -> BitFlags<Self> {
        Self::PvE | Self::Instance | Self::Other
    }
}

impl Named for MapCategory {
    fn name(&self) -> &'static str {
        self.into()
    }

    fn short_name(&self) -> &'static str {
        match self {
            MapCategory::PvE | MapCategory::PvP | MapCategory::WvW => self.name(),
            MapCategory::Instance => "Inst",
            MapCategory::Other => "Other",
        }
    }
}

impl Colored for MapCategory {
    fn colored(&self) -> Option<Color> {
        None
    }
}
