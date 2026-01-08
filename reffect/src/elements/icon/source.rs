use crate::{
    action::DynAction,
    elements::RenderCtx,
    enums::check_variant_array,
    render::{Validation, enum_combo, item_context_menu},
    texture::TextureSource,
};
use const_default::ConstDefault;
use nexus::imgui::{ComboBoxFlags, MenuItem, Ui};
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
pub enum IconSource {
    Empty,

    Unknown,

    #[serde(alias = "Dynamic")]
    Automatic,

    Url(String),

    File(PathBuf),
}

impl ConstDefault for IconSource {
    const DEFAULT: Self = Self::Unknown;
}

impl Default for IconSource {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl VariantArray for IconSource {
    const VARIANTS: &'static [Self] = &[
        Self::Unknown,
        Self::Empty,
        Self::Automatic,
        Self::Url(String::new()),
        Self::File(PathBuf::new()),
    ];
}

const _: () = check_variant_array::<IconSource>();

impl IconSource {
    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }

    pub fn render_select(&mut self, ui: &Ui, ctx: &RenderCtx) -> IconEditResult {
        let mut reload = false;
        let mut action = DynAction::empty();

        let validation = match self {
            Self::Automatic if !ctx.settings.use_game_icons => {
                Validation::Error("Requires experimental reuse game icons setting")
            }
            _ => Validation::Ok,
        };
        validation.for_item(ui, || {
            reload |= enum_combo(ui, "Icon", self, ComboBoxFlags::empty()).is_some()
        });
        item_context_menu("iconsrc", || {
            if MenuItem::new("Copy to all siblings").build(ui) {
                let source = self.clone();
                action.set(move |target| *target = source.clone());
            }
        });

        match self {
            Self::Unknown | Self::Empty | Self::Automatic => {}
            Self::File(path) => reload |= TextureSource::render_file_input(ui, path),
            Self::Url(url) => reload |= TextureSource::render_url_input(ui, url),
        }

        IconEditResult { reload, action }
    }
}

#[derive(Debug)]
pub struct IconEditResult {
    pub reload: bool,
    pub action: DynAction<IconSource>,
}
