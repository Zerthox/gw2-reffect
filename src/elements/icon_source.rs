use crate::{
    render_util::{enum_combo, impl_static_variants},
    texture_manager::TextureManager,
};
use nexus::imgui::{ComboBoxFlags, TextureId, Ui};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use strum::{AsRefStr, EnumIter, IntoStaticStr};

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    Hash,
    IntoStaticStr,
    AsRefStr,
    EnumIter,
    Serialize,
    Deserialize,
)]
pub enum IconSource {
    #[default]
    Unknown,
    File(PathBuf),
    Url(String),
}

impl_static_variants!(IconSource);

impl IconSource {
    pub const UNKNOWN_ID: &'static str = "REFFECT_ICON_UNKNOWN";

    pub fn needs_load(&self) -> bool {
        !matches!(self, Self::Unknown)
    }

    pub fn load(&self) {
        TextureManager::add_source(self)
    }

    pub fn get_texture(&self) -> Option<TextureId> {
        TextureManager::get_texture(self)
    }

    pub fn generate_id(&self) -> String {
        match self {
            Self::Unknown => Self::UNKNOWN_ID.into(),
            Self::File(path) => format!("REFFECT_ICON_FILE_\"{}\"", path.display()),
            Self::Url(url) => format!("REFFECT_ICON_URL_\"{url}\""),
        }
    }

    pub fn pretty_print(&self) -> String {
        match self {
            Self::Unknown => "unknown".into(),
            Self::File(path) => format!("file \"{}\"", path.display()),
            Self::Url(url) => format!("url \"{url}\""),
        }
    }

    pub fn render_select(&mut self, ui: &Ui) {
        enum_combo(ui, "Icon", self, ComboBoxFlags::empty());

        match self {
            Self::Unknown => return,
            Self::File(path) => {
                let mut string = path.to_str().expect("invalid path string").into();
                if ui.input_text("##path", &mut string).build() {
                    *path = string.into();
                }
            }
            Self::Url(url) => {
                ui.input_text("##url", url).build();
            }
        }

        ui.same_line();
        if ui.button("Load") {
            self.load();
        }
    }
}
