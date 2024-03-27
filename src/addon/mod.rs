mod event;
mod ui;

use crate::element::Element;
use std::sync::{Mutex, MutexGuard};

static ADDON: Mutex<Addon> = Mutex::new(Addon::new());

#[derive(Debug)]
pub struct Addon {
    elements: Vec<Element>,
}

impl Addon {
    pub const fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    pub fn lock<'a>() -> MutexGuard<'a, Addon> {
        ADDON.lock().unwrap()
    }
}
