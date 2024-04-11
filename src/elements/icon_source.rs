use crate::{texture_manager::TextureManager, util::enum_combo};
use nexus::imgui::{TextureId, Ui};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use strum::{AsRefStr, EnumDiscriminants, EnumIter};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, EnumDiscriminants, Serialize, Deserialize)]
#[strum_discriminants(derive(AsRefStr, EnumIter))]
pub enum IconSource {
    #[default]
    Empty,
    File(PathBuf),
    Url(String),
}

impl IconSource {
    pub const DEFAULT_ID: &'static str = "REFFECT_ICON_DEFAULT";

    pub fn needs_load(&self) -> bool {
        !matches!(self, Self::Empty)
    }

    pub fn load(&self) {
        TextureManager::add_source(self)
    }

    pub fn get_texture(&self) -> Option<TextureId> {
        TextureManager::get_texture(self)
    }

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
            Self::Url(url) => format!("url \"{url}\""),
        }
    }

    fn to_discrim(&self) -> IconSourceDiscriminants {
        match self {
            Self::Empty => IconSourceDiscriminants::Empty,
            Self::File(_) => IconSourceDiscriminants::File,
            Self::Url(_) => IconSourceDiscriminants::Url,
        }
    }

    fn from_discrim(discrim: IconSourceDiscriminants) -> Self {
        match discrim {
            IconSourceDiscriminants::Empty => Self::Empty,
            IconSourceDiscriminants::File => Self::File(PathBuf::new()),
            IconSourceDiscriminants::Url => Self::Url(String::new()),
        }
    }

    pub fn render_select(&mut self, ui: &Ui) {
        let mut discrim = self.to_discrim();
        if enum_combo::<IconSourceDiscriminants>(ui, "Icon", &mut discrim) {
            *self = Self::from_discrim(discrim);
        }

        ui.same_line();
        match self {
            Self::Empty => {}
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
    }
}
