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
mod updates;

pub use self::{
    combatant::*, edit::*, group::*, item::*, map::*, player::*, resource::*, skill::*, target::*,
    ui::*, updates::*,
};

use crate::{error::Error, links::Links, profiling::measure, worker::StoppableWorker};
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

    /// Pending updates.
    pub updates: Updates,

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
    pub const fn disabled() -> Self {
        Self {
            now: 0,
            updates: Updates::EMPTY,
            edit: EditState::new(),
            ui: UiInfo::new(),
            map: MapInfo::empty(),
            player: PlayerInfo::disabled(),
            target: TargetInfo::disabled(),
            group: Err(Error::Disabled),
        }
    }

    #[inline]
    pub const fn empty() -> Self {
        Self {
            now: 0,
            updates: Updates::EMPTY,
            edit: EditState::new(),
            ui: UiInfo::new(),
            map: MapInfo::empty(),
            player: PlayerInfo::empty(),
            target: TargetInfo::empty(),
            group: Ok(GroupInfo::empty()),
        }
    }

    /// Returns the context.
    #[inline]
    pub fn lock() -> MutexGuard<'static, Self> {
        static CTX: Mutex<Context> = Mutex::new(Context::disabled());

        CTX.lock().unwrap()
    }

    #[inline]
    pub fn unload() {
        *Self::lock() = Self::disabled();
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
                            self.updates.insert(Update::PlayerIdentity);
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
            |elapsed| log::trace!("Slow context update took {elapsed:?}"),
        )
    }

    /// Prepares updates before rendering elements.
    #[inline]
    pub fn prepare_render(&mut self, links: &Links) {
        self.now = unsafe { timeGetTime() };
        self.ui.update(links);
        if let Some(mumble) = links.mumble() {
            self.player.update_fast(mumble);
        }
    }

    /// Resets after rendering.
    #[inline]
    pub fn reset_after_render(&mut self) {
        self.reset_updates();
        self.edit.reset_allowed();
    }

    /// Updates edit mode.
    #[inline]
    pub fn update_edit_mode(&mut self) {
        self.edit.update_allowed(&self.ui);
    }

    /// Checks whether any updates have happened.
    #[inline]
    pub fn has_any_update(&self) -> bool {
        !self.updates.is_empty()
    }

    /// Checks whether any updates have happened or edit mode is active.
    #[inline]
    pub fn has_any_update_or_edit(&self) -> bool {
        self.has_any_update() || self.edit.is_editing()
    }

    /// Checks whether any of the given updates has happened.
    #[inline]
    pub fn has_update(&self, update: impl Into<Updates>) -> bool {
        self.updates.intersects(update)
    }

    /// Checks whether any of the given updates has happened or edit mode is active.
    #[inline]
    pub fn has_update_or_edit(&self, update: impl Into<Updates>) -> bool {
        self.has_update(update) || self.edit.is_editing()
    }

    /// Resets updates.
    #[inline]
    pub fn reset_updates(&mut self) {
        self.updates = Updates::empty();
    }

    /// Forces all updates.
    #[inline]
    pub fn force_update(&mut self) {
        self.updates = Updates::all();
        log::trace!("Forcing all updates");
    }

    #[inline]
    pub fn set_error(&mut self, error: Error) {
        self.player.set_error(error.clone());
        self.target.set_error(error.clone());
        self.group = Err(error);
    }
}
