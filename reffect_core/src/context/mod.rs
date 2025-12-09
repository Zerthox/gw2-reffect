mod combatant;
mod edit;
mod group;
mod item;
mod map;
mod player;
mod resource;
mod skill;
mod target;
mod ui;

pub use self::{
    combatant::*, edit::*, group::*, item::*, map::*, player::*, resource::*, skill::*, target::*,
    ui::*,
};

use crate::{error::Error, links::Links, profiling::measure, worker::StoppableWorker};
use enumflags2::{BitFlags, bitflags};
use std::{
    sync::{Mutex, MutexGuard},
    thread,
    time::Duration,
};
use windows::Win32::Media::timeGetTime;

#[derive(Debug)]
pub struct Context {
    /// Current system time.
    pub now: u32,

    /// Flags for pending updates.
    pub updates: BitFlags<Update>,

    /// Edit state.
    pub edit: EditState,

    /// Information about game UI.
    pub ui: UiInfo,

    /// Information about current map.
    pub map: MapInfo,

    /// Information about player character.
    pub player: PlayerInfo,

    /// Information about current target.
    pub target: TargetInfo,

    /// Information about current group.
    pub group: Result<GroupInfo, Error>,
}

impl Context {
    #[inline]
    pub const fn new() -> Self {
        Self {
            now: 0,
            updates: BitFlags::EMPTY,
            edit: EditState::new(),
            ui: UiInfo::empty(),
            map: MapInfo::empty(),
            player: PlayerInfo::empty(),
            target: TargetInfo::empty(),
            group: Err(Error::Disabled),
        }
    }

    /// Returns the context.
    #[inline]
    pub fn lock() -> MutexGuard<'static, Self> {
        static CTX: Mutex<Context> = Mutex::new(Context::new());

        CTX.lock().unwrap()
    }

    #[inline]
    pub fn unload() {
        *Self::lock() = Self::new();
    }

    /// Creates the context worker.
    pub fn create_worker(links: Links) -> Option<StoppableWorker> {
        const SLEEP: Duration = Duration::from_millis(500);

        StoppableWorker::spawn("reffect-context-worker", move |token| {
            while !token.stop_requested() {
                Self::lock().update_slow(&links);
                thread::sleep(SLEEP);
            }
        })
    }

    /// Performs a quick update before rendering.
    #[inline]
    pub fn prepare_render(&mut self, links: &Links) {
        self.now = unsafe { timeGetTime() };
        self.ui.update(links);
        if let Some(mumble) = links.mumble() {
            self.player.update_fast(mumble);
        }
    }

    /// Performs a slow update.
    pub fn update_slow(&mut self, links: &Links) {
        measure(
            || {
                // only attempt to update mumble data after first tick
                if let Some(mumble) = links.mumble()
                    && mumble.read_ui_tick() > 0
                {
                    match mumble.parse_identity() {
                        Ok(identity) => {
                            self.updates.insert(Update::Identity);
                            self.player.update_identity(identity);
                        }
                        Err(err) => log::error!("Failed to parse mumble identity: {err}"),
                    }

                    let map_changed = self.map.update(mumble);
                    if map_changed {
                        self.updates.insert(Update::Map);
                    }
                }
            },
            |elapsed| log::debug!("Slow context update took {elapsed:?}"),
        )
    }

    /// Resets the context.
    #[inline]
    pub fn reset(&mut self) {
        self.updates = BitFlags::empty();
        self.edit.reset_allowed();
    }

    /// Checks whether any updates have happened.
    #[inline]
    pub fn has_any_update(&self) -> bool {
        !self.updates.is_empty()
    }

    /// Checks whether any updates have happened or edit mode is active.
    #[inline]
    pub fn has_any_update_or_edit(&self) -> bool {
        self.edit.is_editing() || self.has_any_update()
    }

    /// Checks whether the given updates have happened.
    #[inline]
    pub fn has_update(&self, update: impl Into<BitFlags<Update>>) -> bool {
        self.updates.contains(update)
    }

    /// Checks whether the given updates have happened or edit mode is active.
    #[inline]
    pub fn has_update_or_edit(&self, update: impl Into<BitFlags<Update>>) -> bool {
        self.edit.is_editing() || self.has_update(update)
    }

    #[inline]
    pub fn set_error(&mut self, error: Error) {
        self.player.set_error(error.clone());
        self.target.set_error(error.clone());
        self.group = Err(error);
    }
}

impl Default for Context {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[bitflags]
#[repr(u8)]
pub enum Update {
    /// Game update.
    Game = 1 << 0,

    /// Player identity update.
    Identity = 1 << 1,

    /// Map update.
    Map = 1 << 2,

    /// Traits update.
    Traits = 1 << 3,

    /// Gear update.
    Gear = 1 << 4,
}
