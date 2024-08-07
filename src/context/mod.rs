mod buffs;
mod edit_state;
mod links;
mod map;
mod player;
mod settings;
mod ui;

pub use self::{edit_state::*, links::*, map::*, player::*, settings::*, ui::*};

use crate::{
    internal::{Error, Internal, Resources},
    interval::Interval,
    render_util::Font,
    settings::icon::IconSettings,
};
use buffs::Buffs;
use enumflags2::{bitflags, BitFlags};
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
    pub own_buffs: Result<Buffs, Error>,

    /// Current own resources.
    pub resources: Result<Resources, Error>,

    pub links: Links,

    pub own_interval: Interval,

    pub player_interval: Interval,

    pub save_on_unload: bool,

    pub font: Option<Font>,

    pub icon_settings: IconSettings,
}

impl Context {
    /// Updates the context.
    pub fn update(&mut self, internal: &Internal) {
        self.updates = BitFlags::empty();

        self.now = unsafe { timeGetTime() };

        self.ui.update(&self.links);

        if self.own_interval.triggered(self.now) {
            self.update_own_character(internal);
        }

        if let Some(mumble) = self.links.mumble() {
            self.player.update_fast(mumble);

            if self.player_interval.triggered(self.now) {
                self.updates.insert(ContextUpdate::Player);

                self.player.update_slow(mumble, internal);
                let map_changed = self.map.update(mumble);
                if map_changed {
                    self.updates.insert(ContextUpdate::Map);
                    log::debug!("Updating slow triggers for map id {}", self.map.id);
                }
            }
        }
    }

    fn update_own_character(&mut self, internal: &Internal) {
        self.updates.insert(ContextUpdate::OwnCharacter);
        let (buffs, resources) = internal.update_self();
        self.own_buffs = buffs.map(|buffs| buffs.iter().cloned().collect());
        self.resources = resources;
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
        self.own_interval.frequency = OWN_INTERVAL;
        self.player_interval.frequency = PLAYER_INTERVAL;
    }

    pub fn own_buffs(&self) -> Option<&Buffs> {
        self.own_buffs.as_ref().ok()
    }

    /// Returns the [`Resources`] for the own character, if present.
    pub fn resources(&self) -> Option<&Resources> {
        self.resources.as_ref().ok()
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
            own_buffs: Err(Error::default()),
            resources: Err(Error::default()),
            links: Links::load(),
            own_interval: Interval::new(OWN_INTERVAL),
            player_interval: Interval::new(PLAYER_INTERVAL),
            save_on_unload: true,
            font: None,
            icon_settings: IconSettings::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[bitflags]
#[repr(u8)]
pub enum ContextUpdate {
    OwnCharacter = 1 << 0,
    Player = 1 << 1,
    Map = 1 << 2,
}
