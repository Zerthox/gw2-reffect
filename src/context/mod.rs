mod edit_state;
mod links;
mod map;
mod player;
mod settings;
mod ui;

pub use self::{edit_state::*, links::*, map::*, player::*, settings::*, ui::*};

use crate::{
    get_buffs::{get_buffs, GetBuffsError, StackedBuff},
    interval::Interval,
};
use enumflags2::{bitflags, BitFlags};

const BUFFS_INTERVAL: f64 = 0.040;

const PLAYER_INTERVAL: f64 = 1.000;

#[derive(Debug, Clone)]
pub struct Context {
    /// Edit mode state.
    pub edit: EditState,

    /// Flags for pending updates.
    pub updates: BitFlags<Update>,

    /// Information about game UI.
    pub ui: UiContext,

    /// Information about current map.
    pub map: MapContext,

    /// Information about player character.
    pub player: PlayerContext,

    /// Current buffs sorted by id.
    pub buffs: Vec<StackedBuff>,

    /// Current buffs state.
    pub buffs_state: Result<(), GetBuffsError>,

    links: Links,

    buffs_interval: Interval,

    player_interval: Interval,
}

impl Context {
    /// Updates the context.
    pub fn update(&mut self, time: f64) {
        self.updates = BitFlags::empty();

        self.ui.update(&self.links);
        self.edit.update_allowed(&self.ui);

        if self.buffs_interval.triggered(time) {
            self.buffs_state = unsafe { get_buffs() }.map(|buffs| {
                // keep buffs sorted, unstable is fine
                self.buffs = buffs.to_vec();
                self.buffs.sort_unstable_by_key(|buff| buff.id);
                self.updates.insert(Update::Buffs);
            });
        }

        if let Some(mumble) = self.links.mumble() {
            self.player.update_fast(mumble);

            if self.player_interval.triggered(time) {
                self.player.update_slow(mumble);
                let map_changed = self.map.update(mumble);
                if map_changed {
                    self.updates.insert(Update::Map);
                }
            }
        }
    }

    pub fn get_buffs_interval(&self) -> f64 {
        self.buffs_interval.frequency
    }

    pub fn get_player_interval(&self) -> f64 {
        self.player_interval.frequency
    }

    pub fn replace_buffs_interval(&mut self, interval: f64) {
        self.buffs_interval = Interval::new(interval);
    }

    pub fn replace_player_intervals(&mut self, interval: f64) {
        self.player_interval = Interval::new(interval);
    }

    pub fn buff(&self, id: u32) -> Option<&StackedBuff> {
        self.buffs
            .binary_search_by_key(&id, |entry| entry.id)
            .ok()
            .map(|index| unsafe { self.buffs.get_unchecked(index) }) // index is from binary search, avoid bounds check here
    }

    pub fn has_buff(&self, id: u32) -> bool {
        self.buffs
            .binary_search_by_key(&id, |entry| entry.id)
            .is_ok()
    }

    pub fn stacks_of(&self, id: u32) -> Option<i32> {
        self.buff(id).map(|entry| entry.count)
    }
}

impl Default for Context {
    fn default() -> Self {
        Self {
            edit: EditState::default(),
            updates: BitFlags::empty(),
            ui: UiContext::empty(),
            player: PlayerContext::empty(),
            map: MapContext::empty(),
            buffs: Vec::new(),
            buffs_state: Err(GetBuffsError::Null),
            links: Links::load(),
            buffs_interval: Interval::new(BUFFS_INTERVAL),
            player_interval: Interval::new(PLAYER_INTERVAL),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[bitflags]
#[repr(u8)]
pub enum Update {
    Buffs = 1 << 0,
    Map = 1 << 1,
}
