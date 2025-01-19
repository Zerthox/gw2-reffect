mod clipboard;
mod edit_state;
mod links;
mod map;
mod player;
mod ui;

pub use self::{clipboard::*, edit_state::*, links::*, map::*, player::*, ui::*};

use crate::{
    internal::{BuffMap, Interface, Internal, Resources, State},
    interval::Interval,
    settings::GeneralSettings,
};
use enumflags2::{bitflags, BitFlags};
use reffect_internal::Skillbar;
use windows::Win32::Media::timeGetTime;

#[derive(Debug)]
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

    /// Internal state information.
    pub state: State,

    pub links: Links,

    pub state_interval: Interval, // TODO: upper cap

    pub player_interval: Interval,

    pub settings: GeneralSettings,
}

impl Context {
    pub const DEFAULT_STATE_INTERVAL: u32 = 100;

    pub const DEFAULT_PLAYER_INTERVAL: u32 = 1_000;

    /// Updates the context.
    pub fn update(&mut self) {
        self.updates = BitFlags::empty();

        self.now = unsafe { timeGetTime() };

        self.ui.update(&self.links);

        if self.state_interval.triggered(self.now) {
            self.update_state();
        }

        if let Some(mumble) = self.links.mumble() {
            self.player.update_fast(mumble);

            if self.player_interval.triggered(self.now) {
                self.updates.insert(ContextUpdate::Player);

                self.player.update_slow(mumble);
                let map_changed = self.map.update(mumble);
                if map_changed {
                    self.updates.insert(ContextUpdate::Map);
                    log::debug!("Updating slow triggers for map id {}", self.map.id);
                }
            }
        }
    }

    fn update_state(&mut self) {
        self.updates.insert(ContextUpdate::State);
        Internal::update_state(&mut self.state);
    }

    /// Checks whether any updates have happened.
    pub fn has_any_update(&self) -> bool {
        !self.updates.is_empty()
    }

    /// Checks whether any updates have happened or edit mode is active.
    pub fn has_any_update_or_edit(&self) -> bool {
        self.edit.is_editing() || self.has_any_update()
    }

    /// Checks whether the given updates have happened.
    pub fn has_update(&self, update: impl Into<BitFlags<ContextUpdate>>) -> bool {
        self.updates.contains(update)
    }

    /// Checks whether the given updates have happened or edit mode is active.
    pub fn has_update_or_edit(&self, update: impl Into<BitFlags<ContextUpdate>>) -> bool {
        self.edit.is_editing() || self.has_update(update)
    }

    /// Resets the intervals for all updates.
    pub fn reset_intervals(&mut self) {
        self.state_interval.frequency = Self::DEFAULT_STATE_INTERVAL;
        self.player_interval.frequency = Self::DEFAULT_PLAYER_INTERVAL;
    }

    /// Returns the [`Resources`] for the own character, if present.
    pub fn own_resources(&self) -> Option<&Resources> {
        self.state.own_resources.as_ref().ok()
    }

    /// Returns the [`BuffMap`] for the own character, if present.
    pub fn own_buffs(&self) -> Option<&BuffMap> {
        self.state.own_buffs.as_ref().ok()
    }

    /// Returns the [`Skillbar`] for the own character, if present.
    pub fn own_skillbar(&self) -> Option<&Skillbar> {
        self.state.own_skillbar.as_ref().ok()
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
            state: State::disabled(),
            links: Links::load(),
            state_interval: Interval::new(Self::DEFAULT_STATE_INTERVAL),
            player_interval: Interval::new(Self::DEFAULT_PLAYER_INTERVAL),
            settings: GeneralSettings::default(),
        }
    }
}

// TODO: update flag for traits?
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[bitflags]
#[repr(u8)]
pub enum ContextUpdate {
    State = 1 << 0,
    Player = 1 << 1,
    Map = 1 << 2,
}
