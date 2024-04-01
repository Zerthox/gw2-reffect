use crate::get_buffs::StackedBuff;
use std::borrow::Borrow;

use super::{MapContext, PlayerContext, UiContext};

#[derive(Debug)]
pub struct RenderContext<'a> {
    pub edit: bool,
    pub ui: &'a UiContext,
    pub player: &'a PlayerContext,
    pub map: &'a MapContext,
    pub buffs: &'a [StackedBuff],
}

impl<'a> RenderContext<'a> {
    pub fn with_edit(&self, edit: bool) -> Self {
        Self {
            edit: self.edit || edit,
            ui: self.ui,
            player: self.player,
            map: self.map,
            buffs: self.buffs,
        }
    }

    pub fn buff(&self, id: u32) -> Option<&StackedBuff> {
        self.buffs.iter().find(|entry| entry.id == id)
    }

    pub fn has_buff(&self, id: u32) -> bool {
        self.buffs.iter().any(|entry| entry.id == id)
    }

    pub fn has_buffs_any(&self, ids: impl IntoIterator<Item = impl Borrow<u32>>) -> bool {
        ids.into_iter().any(|id| self.has_buff(*id.borrow()))
    }

    pub fn has_buffs_all(&self, ids: impl IntoIterator<Item = impl Borrow<u32>>) -> bool {
        ids.into_iter().all(|id| self.has_buff(*id.borrow()))
    }

    pub fn stacks_of(&self, id: u32) -> Option<i32> {
        self.buff(id).map(|entry| entry.count)
    }

    pub fn stacks_of_summed(&self, ids: impl IntoIterator<Item = impl Borrow<u32>>) -> i32 {
        ids.into_iter()
            .filter_map(|id| self.stacks_of(*id.borrow()))
            .sum()
    }
}
