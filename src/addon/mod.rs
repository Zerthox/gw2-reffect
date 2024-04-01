mod event;
mod ui;

use crate::{context::Context, element::Pack};
use std::sync::{Mutex, MutexGuard, OnceLock};

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
            context: Context::new(),
        }
    }

    pub fn lock() -> MutexGuard<'static, Addon> {
        ADDON
            .get_or_init(|| Mutex::new(Addon::new()))
            .lock()
            .unwrap()
    }
}

unsafe impl Send for Addon {}
