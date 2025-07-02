mod context;
mod general;

pub mod icon;

pub use self::{context::*, general::*};

use crate::{addon::Addon, context::Context};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::PathBuf,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AddonSettings {
    pub version: String,
    pub context: ContextSettings,
}

impl AddonSettings {
    pub fn new(settings: &GeneralSettings, ctx: &Context) -> Self {
        Self {
            version: VERSION.into(),
            context: ContextSettings::new(settings, ctx),
        }
    }

    pub fn file() -> PathBuf {
        Addon::addon_dir().join("settings.json")
    }

    pub fn try_load() -> Option<Self> {
        let path = Self::file();
        let file = File::open(&path)
            .inspect_err(|err| log::warn!("Failed to read settings file: {err}"))
            .ok()?;
        let reader = BufReader::new(file);
        let settings = serde_json::from_reader(reader)
            .inspect_err(|err| log::warn!("Failed to parse settings file: {err}"))
            .ok()?;
        log::info!("Loaded settings from \"{}\"", path.display());
        Some(settings)
    }

    pub fn apply(self, settings: &mut GeneralSettings, ctx: &mut Context) {
        let Self {
            version: _,
            context,
        } = self;
        context.apply(settings, ctx);
    }

    pub fn save(self) {
        let path = Self::file();
        match File::create(&path) {
            Ok(file) => {
                let writer = BufWriter::new(file);
                serde_json::to_writer_pretty(writer, &self).expect("failed to serialize settings");
                log::info!("Saved settings to \"{}\"", path.display())
            }
            Err(err) => log::error!("Failed to save settings: {err}"),
        }
    }
}
