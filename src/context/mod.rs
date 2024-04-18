mod edit_state;
mod links;
mod map;
mod player;
mod render;
mod settings;
mod ui;

pub use self::{edit_state::*, links::*, map::*, player::*, render::*, settings::*, ui::*};

use crate::{
    get_buffs::{get_buffs, GetBuffsError, StackedBuff},
    interval::Interval,
};

// TODO: optional no edit in combat

const BUFFS_INTERVAL: f64 = 0.040;

const PLAYER_INTERVAL: f64 = 1.000;

#[derive(Debug, Clone)]
pub struct Context {
    pub edit: EditState,
    pub ui: UiContext,
    pub map: MapContext,
    pub player: PlayerContext,
    pub buffs: Result<Vec<StackedBuff>, GetBuffsError>,
    links: Links,
    buffs_update: Interval,
    player_update: Interval,
}

impl Context {
    pub fn as_render(&self) -> RenderContext {
        RenderContext {
            edit: &self.edit,
            ui: &self.ui,
            player: &self.player,
            map: &self.map,
            buffs: self
                .buffs
                .as_ref()
                .map(|buffs| buffs.as_slice())
                .unwrap_or(&[]),
        }
    }

    /// Updates the context.
    /// Returns `true` if a slow update needs to be propagated.
    pub fn update(&mut self, time: f64) -> bool {
        let mut changed = false;

        self.ui.update(&self.links);

        if self.buffs_update.triggered(time) {
            self.buffs = unsafe { get_buffs() }.map(|buffs| buffs.into());
        }

        if let Some(mumble) = self.links.mumble() {
            self.player.update_fast(mumble);

            if self.player_update.triggered(time) {
                self.player.update_slow(mumble);
                changed |= self.map.update(mumble);
            }
        }

        changed
    }

    pub fn get_buffs_interval(&self) -> f64 {
        self.buffs_update.frequency
    }

    pub fn get_player_interval(&self) -> f64 {
        self.player_update.frequency
    }

    pub fn replace_buffs_interval(&mut self, interval: f64) {
        self.buffs_update = Interval::new(interval);
    }

    pub fn replace_player_intervals(&mut self, interval: f64) {
        self.player_update = Interval::new(interval);
    }
}

impl Default for Context {
    fn default() -> Self {
        Self {
            edit: EditState::default(),
            ui: UiContext::empty(),
            player: PlayerContext::empty(),
            map: MapContext::empty(),
            buffs: Err(GetBuffsError::Null),
            links: Links::load(),
            buffs_update: Interval::new(BUFFS_INTERVAL),
            player_update: Interval::new(PLAYER_INTERVAL),
        }
    }
}
