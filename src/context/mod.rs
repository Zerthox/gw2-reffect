mod edit_state;
mod links;
mod map;
mod player;
mod settings;
mod ui;

pub use self::{edit_state::*, links::*, map::*, player::*, settings::*, ui::*};

use crate::{
    internal::{Buff, Error, Internal, Resources},
    interval::Interval,
};
use enumflags2::{bitflags, BitFlags};
use std::collections::BTreeMap;
use windows::Win32::Media::timeGetTime;

const OWN_INTERVAL: u32 = 100;

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

    /// Current own buffs by id.
    pub own_buffs: BTreeMap<u32, Buff>,

    /// Current own buffs error.
    pub own_buffs_error: Option<Error>,

    /// Current own resources.
    pub resources: Resources,

    /// Current own resources error.
    pub resources_error: Option<Error>,

    pub links: Links,

    pub own_interval: Interval,

    pub player_interval: Interval,
}

impl Context {
    /// Updates the context.
    pub fn update(&mut self, internal: &mut Internal) {
        self.updates = BitFlags::empty();

        self.now = unsafe { timeGetTime() };

        self.ui.update(&self.links);

        if self.own_interval.triggered(self.now) {
            self.update_own_character(internal);
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

    fn update_own_character(&mut self, internal: &mut Internal) {
        self.own_buffs.clear();

        let (buffs, resources) = internal.update_self();
        self.own_buffs_error = buffs
            .map(|buffs| {
                self.own_buffs
                    .extend(buffs.iter().map(|buff| (buff.id, buff.clone())));
            })
            .err();

        (self.resources, self.resources_error) = match resources {
            Ok(resources) => (resources, None),
            Err(err) => (Resources::empty(), Some(err)),
        };

        self.updates.insert(ContextUpdate::OwnCharacter);
    }

    /// Whether the given update has happened.
    pub fn has_update(&self, update: ContextUpdate) -> bool {
        self.updates.contains(update)
    }

    /// Whether the given update has happened or edit mode is active.
    pub fn has_update_or_edit(&self, update: ContextUpdate) -> bool {
        self.edit.is_editing() || self.has_update(update)
    }

    /// Resets the intervals for all updates.
    pub fn reset_intervals(&mut self) {
        self.own_interval.frequency = OWN_INTERVAL;
        self.player_interval.frequency = PLAYER_INTERVAL;
    }

    /// Returns whether own character state is available.
    pub fn is_self_ok(&self) -> bool {
        self.own_buffs_error.is_none()
    }

    /// Returns the [`Buff`] for a given buff id, if present.
    pub fn buff(&self, id: u32) -> Option<&Buff> {
        self.own_buffs
            .get(&id)
            .filter(|buff| buff.runout_time > self.now)
    }

    /// Checks whether a given buff id is present.
    pub fn has_buff(&self, id: u32) -> bool {
        self.buff(id).is_some()
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
            own_buffs_error: None,
            own_buffs: BTreeMap::new(),
            resources_error: None,
            resources: Resources::empty(),
            links: Links::load(),
            own_interval: Interval::new(OWN_INTERVAL),
            player_interval: Interval::new(PLAYER_INTERVAL),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[bitflags]
#[repr(u8)]
pub enum ContextUpdate {
    OwnCharacter = 1 << 0,
    Map = 1 << 1,
}
