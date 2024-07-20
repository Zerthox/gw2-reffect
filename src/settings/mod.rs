use super::Addon;
use crate::context::{Context, ContextSettings};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::PathBuf,
};

// TODO: setting for icon stacks text

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub version: String,
    pub context: ContextSettings,
}

impl Settings {
    pub fn new(context: &Context) -> Self {
        Self {
            version: VERSION.into(),
            context: context.settings(),
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
