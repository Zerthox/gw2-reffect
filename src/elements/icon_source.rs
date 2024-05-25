use crate::{
    render_util::{enum_combo, impl_static_variants},
    texture_manager::TextureManager,
};
use nexus::imgui::{ComboBoxFlags, TextureId, Ui};
use serde::{Deserialize, Serialize};
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
    Url(String),
    File(String), // TODO: relative to addon dir?
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
            Self::File(path) => format!("REFFECT_ICON_FILE_\"{}\"", path),
            Self::Url(url) => format!("REFFECT_ICON_URL_\"{url}\""),
        }
    }

    pub fn pretty_print(&self) -> String {
        match self {
            Self::Unknown => "unknown".into(),
            Self::File(path) => format!("file \"{}\"", path),
            Self::Url(url) => format!("url \"{url}\""),
        }
    }

    pub fn render_select(mut self: &mut Self, ui: &Ui) {
        if let Some(prev) = enum_combo(ui, "Icon", self, ComboBoxFlags::empty()) {
            match (prev, &mut self) {
                (Self::Url(url), Self::File(new)) => {
                    *new = url;
                }
                (Self::File(path), Self::Url(new)) => {
                    *new = path;
                }
                _ => {}
            }
        }

        match self {
            Self::Unknown => return,
            Self::File(path) => {
                // TODO: file dialog for select
                ui.input_text("##path", path).build();
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
