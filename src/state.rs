#![allow(dead_code)]

use crate::get_buffs::StackedBuff;
use nexus::data_link::mumble::{map_type, Context};

#[derive(Debug)]
pub struct State<'a> {
    buffs: &'a [StackedBuff],
    map: Option<MapInfo>,
}

impl<'a> State<'a> {
    pub const fn new(buffs: &'a [StackedBuff], map: Option<MapInfo>) -> Self {
        Self { buffs, map }
    }

    pub fn buff(&self, id: u32) -> Option<&StackedBuff> {
        self.buffs.iter().find(|entry| entry.id == id)
    }

    pub fn has_buff(&self, id: u32) -> bool {
        self.buffs.iter().any(|entry| entry.id == id)
    }

    pub fn stacks_of(&self, id: u32) -> Option<i32> {
        self.buff(id).map(|entry| entry.count)
    }
}

#[derive(Debug, Clone)]
pub struct MapInfo {
    id: u32,
    kind: MapKind,
}

impl MapInfo {
    pub fn from_mumble(context: &Context) -> Self {
        Self {
            id: context.map_id,
            kind: context.map_type.into(),
        }
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
