use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::PathBuf,
};

use super::Addon;
use crate::context::ContextSettings;
use serde::{Deserialize, Serialize};

// TODO: setting for icon stacks text

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub context: ContextSettings,
}

impl Settings {
    pub fn file() -> PathBuf {
        Addon::addon_dir().join("settings.json")
    }

    pub fn try_load() -> Option<Self> {
        let file = File::open(Self::file())
            .inspect_err(|err| log::warn!("Failed to read settings file: {err}"))
            .ok()?;
        let reader = BufReader::new(file);
        serde_json::from_reader(reader)
            .inspect_err(|err| log::warn!("Failed to parse settings file: {err}"))
            .ok()
    }

    pub fn save(self) {
        match File::create(Self::file()) {
            Ok(file) => {
                let writer = BufWriter::new(file);
                serde_json::to_writer_pretty(writer, &self).expect("failed to serialize settings");
            }
            Err(err) => log::error!("Failed to save settings: {err}"),
        }
    }
}
