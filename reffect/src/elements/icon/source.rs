use crate::{
    action::DynAction,
    addon::Addon,
    elements::RenderCtx,
    enums::check_variant_array,
    lockbox::Lockbox,
    render::{Validation, enum_combo, input_text_simple_menu, item_context_menu},
};
use const_default::ConstDefault;
use nexus::imgui::{ComboBoxFlags, MenuItem, Ui};
use rfd::FileDialog;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, thread};
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

    pub fn render_select(&mut self, ui: &Ui, ctx: &RenderCtx) -> IconSelectResult {
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
                    reload = true;
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
                    reload = true;
                }
            }
        }

        IconSelectResult { reload, action }
    }
}

#[derive(Debug)]
pub struct IconSelectResult {
    pub reload: bool,
    pub action: DynAction<IconSource>,
}
