mod event;
mod ui;

use crate::{elements::Pack, settings::GeneralSettings};
use nexus::paths::get_addon_dir;
use reffect_core::{links::Links, worker::StoppableWorker};
use std::{
    path::PathBuf,
    sync::{Mutex, MutexGuard, OnceLock},
};

static ADDON: OnceLock<Mutex<Addon>> = OnceLock::new();

#[derive(Debug)]
pub struct Addon {
    debug: bool,
    create_error: bool,
    links: Links,
    packs: Vec<Pack>,
    settings: GeneralSettings,
    worker: Option<StoppableWorker>,
}

impl Addon {
    pub fn new() -> Self {
        Self {
            debug: false,
            create_error: false,
            links: Links::load(),
            packs: Vec::new(),
            settings: GeneralSettings::new(),
            worker: None,
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

    pub fn icons_dir() -> PathBuf {
        Self::addon_dir().join("icons")
    }
}
