use super::Addon;
use nexus::paths::get_addon_dir;
use std::{fs, path::PathBuf};

impl Addon {
    pub fn addon_dir() -> PathBuf {
        get_addon_dir("reffect").expect("invalid addon directory")
    }

    pub fn packs_dir() -> PathBuf {
        Self::addon_dir().join("packs")
    }

    pub fn icons_dir() -> PathBuf {
        Self::addon_dir().join("icons")
    }

    pub fn fonts_dir() -> PathBuf {
        Self::addon_dir().join("fonts")
    }

    pub fn create_dirs() {
        let _ = fs::create_dir_all(Self::packs_dir());
        let _ = fs::create_dir(Self::icons_dir());
        let _ = fs::create_dir(Self::fonts_dir());
    }

    pub fn open_addon_folder(&self) {
        if let Err(err) = open::that_detached(Self::addon_dir()) {
            log::error!("Failed to open addon folder: {err}");
        }
    }
}
