mod links;
mod map;
mod player;
mod render;
mod ui;

pub use self::{links::*, map::*, player::*, render::*, ui::*};

use crate::{
    get_buffs::{get_buffs, GetBuffsError, StackedBuff},
    interval::Interval,
};

#[derive(Debug)]
pub struct Context {
    pub edit: bool,
    pub ui: UiContext,
    pub map: MapContext,
    pub player: PlayerContext,
    pub buffs: Result<Vec<StackedBuff>, GetBuffsError>,
    links: Links,
    buffs_update: Interval,
    slow_update: Interval,
}

impl Context {
    pub fn new() -> Self {
        Self {
            edit: false,
            ui: UiContext::empty(),
            player: PlayerContext::empty(),
            map: MapContext::empty(),
            buffs: Err(GetBuffsError::Null),
            links: Links::load(),
            buffs_update: Interval::new(0.040),
            slow_update: Interval::new(1.000),
        }
    }

    pub fn as_render(&self) -> Option<RenderContext> {
        self.buffs
            .as_ref()
            .map(|buffs| RenderContext {
                edit: self.edit,
                ui: &self.ui,
                player: &self.player,
                map: &self.map,
                buffs: buffs.as_slice(),
            })
            .ok()
    }

    pub fn update(&mut self, time: f64) {
        self.ui.update(&self.links);
        if self.buffs_update.triggered(time) {
            self.buffs = unsafe { get_buffs() }.map(|buffs| buffs.into());
        }
        if self.slow_update.triggered(time) {
            if let Some(mumble) = self.links.mumble() {
                self.player.update(mumble);
                self.map.update(&mumble.context);
            }
        }
    }
}
