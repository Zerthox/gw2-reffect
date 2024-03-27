use nexus::data_link::mumble::{map_type, Context};

#[derive(Debug, Clone)]
pub struct MapInfo {
    pub id: u32,
    pub kind: MapKind,
}

impl MapInfo {
    pub const fn empty() -> Self {
        Self {
            id: 0,
            kind: MapKind::Unknown,
        }
    }

    pub fn update(&mut self, context: &Context) {
        self.id = context.map_id;
        self.kind = context.map_type.into();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MapKind {
    Pve,
    Pvp,
    Wvw,
    Instance,
    Unknown,
}

impl From<u32> for MapKind {
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
