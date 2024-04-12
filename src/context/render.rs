use super::{EditState, MapContext, PlayerContext, UiContext};
use crate::get_buffs::StackedBuff;

#[derive(Debug)]
pub struct RenderContext<'a> {
    pub edit: &'a EditState,
    pub ui: &'a UiContext,
    pub player: &'a PlayerContext,
    pub map: &'a MapContext,
    pub buffs: &'a [StackedBuff],
}

impl<'a> RenderContext<'a> {
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
