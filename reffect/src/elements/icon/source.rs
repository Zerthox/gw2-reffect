use crate::{
    action::DynAction,
    addon::Addon,
    elements::RenderCtx,
    enums::impl_static_variants,
    internal::{Interface, Internal},
    lockbox::Lockbox,
    render::{Validation, enum_combo, input_text_simple_menu},
    texture_manager::TextureManager,
    trigger::Skill,
};
use const_default::ConstDefault;
use nexus::imgui::{ComboBoxFlags, TextureId, Ui};
use rfd::FileDialog;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, thread};
use strum::{AsRefStr, EnumCount, EnumIter, IntoStaticStr};
use windows::core::Interface as _;

// TODO: id gen for loaded icons? handle duplicates on load?

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
pub enum IconSource {
    Unknown,

    Empty,

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

impl_static_variants!(IconSource); // TODO: move to const variant array when pathbuf new is const

impl IconSource {
    pub const UNKNOWN_ID: &'static str = "REFFECT_ICON_UNKNOWN";

    pub fn needs_load(&self) -> bool {
        !matches!(self, Self::Unknown | Self::Empty)
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }

    pub fn load(&self) {
        TextureManager::add_source(self)
    }

    pub fn get_texture(&self, skill: Skill) -> Option<TextureId> {
        match self {
            Self::Empty => None,
            Self::Automatic => match skill {
                Skill::Unknown => TextureManager::get_texture(&IconSource::Unknown),
                Skill::WeaponSwap => TextureManager::get_weapon_swap(),
                Skill::BundleDrop => TextureManager::get_bundle_drop(),
                Skill::Id(id) => match Internal::get_skill_icon(id) {
                    Some(tex) => Some(tex.as_raw().into()),
                    None => TextureManager::get_texture(&IconSource::Unknown),
                },
            },
            _ => TextureManager::get_texture(self),
        }
    }

    pub fn generate_id(&self) -> String {
        match self {
            Self::Unknown => Self::UNKNOWN_ID.into(),
            Self::Empty | Self::Automatic => String::new(),
            Self::File(path) => format!("REFFECT_ICON_FILE_\"{}\"", path.display()),
            Self::Url(url) => format!("REFFECT_ICON_URL_\"{url}\""),
        }
    }

    pub fn pretty_print(&self) -> String {
        match self {
            Self::Unknown => "unknown".into(),
            Self::Empty => "empty".into(),
            Self::Automatic => "automatic".into(),
            Self::File(path) => format!("file \"{}\"", path.display()),
            Self::Url(url) => format!("url \"{url}\""),
        }
    }

    pub fn render_select(&mut self, ui: &Ui, ctx: &RenderCtx) -> DynAction<Self> {
        let mut action = DynAction::empty();

        let validation = match self {
            Self::Automatic if !ctx.settings.use_game_icons => {
                Validation::Error("Requires experimental reuse game icons setting")
            }
            _ => Validation::Ok,
        };
        validation.for_item(ui, || enum_combo(ui, "Icon", self, ComboBoxFlags::empty()));
        action.render_copy_all_cloned(ui, "iconsrc", self, |target, source| {
            *target = source.clone()
        });

        // we assume this stays in place, otherwise we consider the file dialog invalidated
        let id = self as *mut _ as usize;

        match self {
            Self::Unknown | Self::Empty | Self::Automatic => {}
            Self::File(path) => {
                let validation = if path.is_absolute() {
                    Validation::Warn("Non-shareable absolute icon path")
                } else {
                    Validation::Ok
                };
                validation.for_item(ui, || {
                    ui.input_text("##path", &mut path.display().to_string())
                        .hint("No file")
                        .auto_select_all(true)
                        .read_only(true)
                        .build()
                });

                static FILE: Lockbox<usize, PathBuf> = Lockbox::new();

                ui.same_line();
                if ui.button("Select") {
                    thread::spawn(move || {
                        let dir = Addon::icons_dir();
                        if let Some(file) = FileDialog::new()
                            .set_title("Select Icon")
                            .set_directory(&dir)
                            .add_filter("Image", &["png", "jpg", "jpeg"])
                            .pick_file()
                        {
                            // try to get the relative path from icons folder
                            let file = match file.strip_prefix(dir) {
                                Ok(relative) => relative.to_path_buf(),
                                Err(_) => {
                                    log::warn!("Absolute icon path \"{}\"", file.display());
                                    file
                                }
                            };
                            FILE.write(id, file);
                        }
                    });
                }

                if let Some(file) = FILE.try_take(id) {
                    *path = file;
                    self.load();
                }
            }
            Self::Url(url) => {
                ui.input_text("##url", url)
                    .hint("https://wiki.guildwars2.com/...")
                    .auto_select_all(true)
                    .build();
                input_text_simple_menu(ui, "##urlctx", url);

                ui.same_line();
                if ui.button("Load") {
                    self.load();
                }
            }
        }

        action
    }
}
