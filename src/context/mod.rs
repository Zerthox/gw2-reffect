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
use windows::Win32::Media::timeGetTime;

const BUFFS_INTERVAL: u32 = 100;

const PLAYER_INTERVAL: u32 = 1_000;

#[derive(Debug, Clone)]
pub struct Context {
    /// Current system time.
    pub now: u32,

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
    pub buffs: BTreeMap<u32, Buff>, // TODO: switch to result/option here?

    /// Current buffs state.
    pub buffs_state: bool, // TODO: error enum in internals

    links: Links,

    buffs_interval: Interval,

    player_interval: Interval,
}

impl Context {
    /// Updates the context.
    pub fn update(&mut self) {
        self.updates = BitFlags::empty();

        self.now = unsafe { timeGetTime() };

        self.ui.update(&self.links);

        if self.buffs_interval.triggered(self.now) {
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

            if self.player_interval.triggered(self.now) {
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
    pub fn get_buffs_interval(&self) -> u32 {
        self.buffs_interval.frequency
    }

    /// Returns the interval for player updates.
    pub fn get_player_interval(&self) -> u32 {
        self.player_interval.frequency
    }

    /// Changes the interval for buff updates.
    pub fn replace_buffs_interval(&mut self, interval: u32) {
        self.buffs_interval = Interval::new(interval);
    }

    /// Changes the interval for player updates.
    pub fn replace_player_interval(&mut self, interval: u32) {
        self.player_interval = Interval::new(interval);
    }

    /// Resets the intervals for all updates.
    pub fn reset_intervals(&mut self) {
        self.replace_buffs_interval(BUFFS_INTERVAL);
        self.replace_player_interval(PLAYER_INTERVAL);
    }

    /// Checks whether a given buff id is present.
    pub fn has_buff(&self, id: u32) -> bool {
        self.buff(id).is_some()
    }

    /// Returns the [`Buff`] for a given buff id, if present.
    pub fn buff(&self, id: u32) -> Option<&Buff> {
        self.buffs
            .get(&id)
            .filter(|buff| buff.runout_time > self.now)
    }

    /// Returns the number of stacks for a given buff id, if present.
    pub fn stacks_of(&self, id: u32) -> Option<u32> {
        self.buff(id).map(|entry| entry.stacks)
    }

    /// Returns the apply and runout time for a given buff id, if present.
    pub fn time_range(&self, id: u32) -> Option<(u32, u32)> {
        self.buff(id)
            .map(|entry| (entry.apply_time, entry.runout_time))
    }

    /// Returns the duration passed since a given timestamp.
    pub fn time_since(&self, time: u32) -> Option<u32> {
        (time != u32::MAX).then(|| self.now.saturating_sub(time))
    }

    /// Returns the remaining duration until a given timestamp.
    pub fn time_until(&self, time: u32) -> Option<u32> {
        (time != u32::MAX).then(|| time.saturating_sub(self.now))
    }

    /// Returns the remaining progress between two timestamps.
    pub fn progress_remaining(&self, start: u32, end: u32) -> Option<f32> {
        self.time_until(end).map(|remain| {
            let full = end - start;
            remain as f32 / full as f32
        })
    }
}

impl Default for Context {
    fn default() -> Self {
        Self {
            now: 0,
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
