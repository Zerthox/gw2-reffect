use crate::{
    enums::check_variant_array,
    render::enum_combo,
    texture::{AsTextureSource, LoadedTexture, TextureManager, TextureSource},
};
use const_default::ConstDefault;
use nexus::imgui::{self, ComboBoxFlags, Ui};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use strum::{AsRefStr, EnumCount, EnumIter, IntoStaticStr, VariantArray};

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    IntoStaticStr,
    AsRefStr,
    EnumIter,
    EnumCount,
    Serialize,
    Deserialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum BarTexture {
    #[serde(alias = "Empty")]
    None,

    Url(String),

    File(PathBuf),
}

pub type LoadedBarTexture = LoadedTexture<BarTexture>;

impl ConstDefault for BarTexture {
    const DEFAULT: Self = Self::None;
}

impl Default for BarTexture {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl VariantArray for BarTexture {
    const VARIANTS: &'static [Self] = &[
        Self::None,
        Self::Url(String::new()),
        Self::File(PathBuf::new()),
    ];
}

const _: () = check_variant_array::<BarTexture>();

impl BarTexture {
    pub const fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    pub fn render_select(&mut self, ui: &Ui, label: impl AsRef<str>) -> bool {
        let mut reload = false;

        let label = label.as_ref();
        reload |= enum_combo(ui, label, self, ComboBoxFlags::empty()).is_some();

        let _id = ui.push_id(label);
        match self {
            Self::None => {}
            Self::File(path) => reload |= TextureSource::render_file_input(ui, path),
            Self::Url(url) => reload |= TextureSource::render_url_input(ui, url),
        }

        reload
    }
}

impl AsTextureSource for BarTexture {
    fn as_texture_source(&self) -> Option<TextureSource> {
        match self {
            Self::None => None,
            Self::Url(url) => Some(TextureSource::Url(url.clone())),
            Self::File(file) => Some(TextureSource::File(file.clone())),
        }
    }
}

impl LoadedBarTexture {
    pub fn get_texture(&self, _ui: &Ui) -> Option<imgui::TextureId> {
        match self.source() {
            BarTexture::None => None,
            BarTexture::File(_) | BarTexture::Url(_) => {
                self.key().and_then(TextureManager::get_texture)
            }
        }
    }

    pub fn render_select(&mut self, ui: &Ui, label: impl AsRef<str>) {
        let mut source = self.source_mut();
        let reload = source.render_select(ui, label);
        source.reload_if(reload);
    }
}
