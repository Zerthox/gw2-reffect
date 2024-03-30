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
    pub fn needs_load(&self) -> bool {
        !matches!(self, Self::Empty)
    }

    pub fn load(&self) {
        TextureManager::add_source(self)
    }

    pub fn get_texture(&self) -> Option<TextureId> {
        TextureManager::get_texture(self)
    }

    pub const DEFAULT_ID: &'static str = "REFFECT_ICON_DEFAULT";

    pub fn generate_id(&self) -> String {
        match self {
            Self::Empty => Self::DEFAULT_ID.into(),
            Self::File(path) => format!("REFFECT_ICON_FILE_\"{}\"", path.display()),
            Self::Url(url) => format!("REFFECT_ICON_URL_\"{url}\""),
        }
    }

    pub fn pretty_print(&self) -> String {
        match self {
            Self::Empty => "empty".into(),
            Self::File(path) => format!("file \"{}\"", path.display()),
            Self::Url(url) => format!("url \"{}\"", url.replace('%', "%%")), // TODO: remove once fixed in nexus
        }
    }
}
