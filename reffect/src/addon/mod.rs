mod dir;
mod event;
mod font;
mod ui;

use crate::{elements::Pack, links::Links, settings::GeneralSettings, worker::StoppableWorker};
use std::sync::{Mutex, MutexGuard, OnceLock};

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
    pub const VERSION: &str = env!("CARGO_PKG_VERSION");

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

    pub fn release(&mut self) {
        *self = Self::new();
    }
}
