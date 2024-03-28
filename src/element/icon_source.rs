use crate::texture_manager::TextureManager;
use nexus::imgui::TextureId;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IconSource {
    Empty,
    File(PathBuf),
    Url(String),
}

impl IconSource {
    pub fn load(&self) {
        TextureManager::add_source(self)
    }

    pub fn get_texture(&self) -> TextureId {
        TextureManager::get_texture(self)
    }
}
