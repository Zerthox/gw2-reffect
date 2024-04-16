mod event;
mod settings;
mod ui;

pub use self::settings::*;

use crate::{context::Context, elements::Pack};
use nexus::paths::get_addon_dir;
use std::{
    path::PathBuf,
    sync::{Mutex, MutexGuard, OnceLock},
};

static ADDON: OnceLock<Mutex<Addon>> = OnceLock::new();

#[derive(Debug)]
pub struct Addon {
    debug: bool,
    packs: Vec<Pack>,
    context: Context,
}

impl Addon {
    pub fn new() -> Self {
        Self {
            debug: false,
            packs: Vec::new(),
            context: Context::default(),
        }
    }

    pub fn lock() -> MutexGuard<'static, Addon> {
        ADDON
            .get_or_init(|| Mutex::new(Addon::new()))
            .lock()
            .unwrap()
    }

    pub fn addon_dir() -> PathBuf {
        get_addon_dir("reffect").expect("invalid addon directory")
    }

    pub fn packs_dir() -> PathBuf {
        Self::addon_dir().join("packs")
    }
}
