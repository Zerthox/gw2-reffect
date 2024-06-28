mod edit_state;
mod links;
mod map;
mod player;
mod settings;
mod ui;

use std::collections::BTreeMap;

pub use self::{edit_state::*, links::*, map::*, player::*, settings::*, ui::*};

use crate::{
    internal::{get_buffs, Buff},
    interval::Interval,
};
use enumflags2::{bitflags, BitFlags};

const BUFFS_INTERVAL: f64 = 0.040;

const PLAYER_INTERVAL: f64 = 1.000;

#[derive(Debug, Clone)]
pub struct Context {
    /// Flags for pending updates.
    updates: BitFlags<ContextUpdate>,

    /// Edit mode state.
    pub edit: EditState,

    /// Information about game UI.
    pub ui: UiContext,

    /// Information about current map.
    pub map: MapContext,

    /// Information about player character.
    pub player: PlayerContext,

    /// Current buffs by id.
    pub buffs: BTreeMap<u32, Buff>,

    /// Current buffs state.
    pub buffs_state: bool, // TODO: error enum in internals

    links: Links,

    buffs_interval: Interval,

    player_interval: Interval,
}

impl Context {
    /// Updates the context.
    pub fn update(&mut self, time: f64) {
        self.updates = BitFlags::empty();

        self.ui.update(&self.links);

        if self.buffs_interval.triggered(time) {
            self.buffs.clear();
            if let Some(buffs) = unsafe { get_buffs() } {
                self.buffs
                    .extend(buffs.iter().map(|buff| (buff.id, buff.clone())));
                self.buffs_state = true;
            } else {
                self.buffs_state = false;
            }
            self.updates.insert(ContextUpdate::Buffs);
        }

        if let Some(mumble) = self.links.mumble() {
            self.player.update_fast(mumble);

            if self.player_interval.triggered(time) {
                self.player.update_slow(mumble);
                let map_changed = self.map.update(mumble);
                if map_changed {
                    self.updates.insert(ContextUpdate::Map);
                    log::debug!("Updating slow triggers for map id {}", self.map.id);
                }
            }
        }
    }

    /// Whether the given update has happened.
    pub fn has_update(&self, update: ContextUpdate) -> bool {
        self.updates.contains(update)
    }

    /// Whether the given update has happened or edit mode is active.
    pub fn has_update_or_edit(&self, update: ContextUpdate) -> bool {
        self.edit.is_editing() || self.has_update(update)
    }

    /// Returns the interval for buff updates.
    pub fn get_buffs_interval(&self) -> f64 {
        self.buffs_interval.frequency
    }

    /// Returns the interval for player updates.
    pub fn get_player_interval(&self) -> f64 {
        self.player_interval.frequency
    }

    /// Changes the interval for buff updates.
    pub fn replace_buffs_interval(&mut self, interval: f64) {
        self.buffs_interval = Interval::new(interval);
    }

    /// Changes the interval for player updates.
    pub fn replace_player_interval(&mut self, interval: f64) {
        self.player_interval = Interval::new(interval);
    }

    /// Resets the intervals for all updates.
    pub fn reset_intervals(&mut self) {
        self.replace_buffs_interval(BUFFS_INTERVAL);
        self.replace_player_interval(PLAYER_INTERVAL);
    }

    /// Checks whether a given buff id is present.
    pub fn has_buff(&self, id: u32) -> bool {
        self.buffs.contains_key(&id)
    }

    /// Returns the [`StackedBuff`] for a given buff id, if present.
    pub fn buff(&self, id: u32) -> Option<&Buff> {
        self.buffs.get(&id)
    }

    /// Returns the number of stacks for a given buff id, if present.
    pub fn stacks_of(&self, id: u32) -> Option<u32> {
        self.buff(id).map(|entry| entry.stacks)
    }
}

impl Default for Context {
    fn default() -> Self {
        Self {
            updates: BitFlags::empty(),
            edit: EditState::default(),
            ui: UiContext::empty(),
            player: PlayerContext::empty(),
            map: MapContext::empty(),
            buffs: BTreeMap::new(),
            buffs_state: false,
            links: Links::load(),
            buffs_interval: Interval::new(BUFFS_INTERVAL),
            player_interval: Interval::new(PLAYER_INTERVAL),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[bitflags]
#[repr(u8)]
pub enum ContextUpdate {
    Buffs = 1 << 0,
    Map = 1 << 1,
}
