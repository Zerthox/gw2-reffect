use crate::enums::check_variant_array;
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
pub enum TextureSource {
    File(PathBuf),
    Url(String),
}

impl VariantArray for TextureSource {
    const VARIANTS: &'static [Self] = &[Self::File(PathBuf::new()), Self::Url(String::new())];
}

const _: () = check_variant_array::<TextureSource>();

impl TextureSource {
    pub fn generate_nexus_id(&self) -> String {
        match self {
            Self::File(path) => format!("REFFECT_ICON_FILE_\"{}\"", path.display()),
            Self::Url(url) => format!("REFFECT_ICON_URL_\"{url}\""),
        }
    }

    pub fn pretty_print(&self) -> String {
        match self {
            Self::File(path) => format!("file \"{}\"", path.display()),
            Self::Url(url) => format!("url \"{url}\""),
        }
    }
}
