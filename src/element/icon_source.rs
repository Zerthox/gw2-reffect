use nexus::{
    imgui::TextureId,
    texture::{get_texture_or_create_from_file, get_texture_or_create_from_url},
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use url::Url;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IconSource {
    Empty,
    Url(String),
    File(PathBuf),
}

impl IconSource {
    pub fn gen_id(&self) -> String {
        format!("REFFECT_ICON_{}", Uuid::new_v4().simple())
    }

    pub fn load_texture(&self) -> Option<TextureId> {
        let id = self.gen_id();
        let result = match self {
            Self::Empty => None,
            Self::Url(url) => {
                let url = Url::parse(url).ok()?;
                if !matches!(url.scheme(), "http" | "https") {
                    return None;
                }
                let host = url.host_str()?;
                let path = url.path();
                get_texture_or_create_from_url(id, format!("https://{host}"), path)
            }
            Self::File(path) => get_texture_or_create_from_file(id, path),
        };
        result.map(|texture| texture.id())
    }
}
