mod map;
mod player;

pub use self::{map::*, player::*};

use crate::get_buffs::StackedBuff;

#[derive(Debug)]
pub struct Context<'a> {
    pub edit: bool,
    pub player: &'a PlayerContext,
    pub buffs: &'a [StackedBuff],
}

impl<'a> Context<'a> {
    pub const fn new(edit: bool, player: &'a PlayerContext, buffs: &'a [StackedBuff]) -> Self {
        Self {
            edit,
            player,
            buffs,
        }
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
