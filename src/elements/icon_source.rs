use crate::{
    addon::Addon,
    lockbox::Lockbox,
    render_util::{enum_combo, impl_static_variants},
    texture_manager::TextureManager,
};
use nexus::imgui::{ComboBoxFlags, TextureId, Ui};
use rfd::FileDialog;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, thread};
use strum::{AsRefStr, EnumIter, IntoStaticStr};

// TODO: id gen for loaded icons? handle duplicates on load?

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
    File(PathBuf),
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
            Self::Unknown => {}
            Self::File(path) => {
                ui.input_text("##path", &mut path.display().to_string())
                    .hint("No file")
                    .auto_select_all(true)
                    .read_only(true)
                    .build();

                // we assume this stays in place, otherwise we consider the file dialog invalidated
                let id = path.as_os_str().as_encoded_bytes().as_ptr() as usize;

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
                    .build();
                ui.same_line();
                if ui.button("Load") {
                    self.load();
                }
            }
        }
    }
}
