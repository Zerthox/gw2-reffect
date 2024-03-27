mod event;
mod ui;

use crate::{
    context::PlayerContext,
    element::Pack,
    get_buffs::{GetBuffsError, StackedBuff},
    interval::Interval,
};
use std::sync::{Mutex, MutexGuard, OnceLock};

static ADDON: OnceLock<Mutex<Addon>> = OnceLock::new();

const BUFFS_INTERVAL: u32 = 10;

const PLAYER_INTERVAL: u32 = 100;

#[derive(Debug)]
pub struct Addon {
    packs: Vec<Pack>,
    editing: bool,
    buffs: Result<Vec<StackedBuff>, GetBuffsError>,
    player: PlayerContext,
    buffs_update: Interval,
    player_update: Interval,
}

impl Addon {
    pub fn new() -> Self {
        Self {
            packs: Vec::new(),
            editing: false,
            buffs: Ok(Vec::new()),
            player: PlayerContext::empty(),
            buffs_update: Interval::new(BUFFS_INTERVAL),
            player_update: Interval::new(PLAYER_INTERVAL),
        }
    }

    pub fn lock() -> MutexGuard<'static, Addon> {
        ADDON
            .get_or_init(|| Mutex::new(Addon::new()))
            .lock()
            .unwrap()
    }
}
