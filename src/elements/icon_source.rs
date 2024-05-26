use crate::{
    render_util::{enum_combo, impl_static_variants},
    texture_manager::TextureManager,
    util::file_name,
};
use nexus::{
    imgui::{ComboBoxFlags, TextureId, Ui},
    paths::get_game_dir,
};
use rfd::FileDialog;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, sync::Mutex, thread};
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

    pub fn render_select(mut self: &mut Self, ui: &Ui) {
        if let Some(prev) = enum_combo(ui, "Icon", self, ComboBoxFlags::empty()) {
            match (prev, &mut self) {
                (Self::Url(url), Self::File(new)) => {
                    *new = url.into();
                }
                (Self::File(path), Self::Url(new)) => {
                    *new = path.to_string_lossy().into_owned();
                }
                _ => {}
            }
        }

        match self {
            Self::Unknown => return,
            Self::File(path) => {
                ui.text(file_name(path));

                ui.same_line();
                static FILE: Mutex<Option<PathBuf>> = Mutex::new(None);
                match FILE.lock().unwrap().take() {
                    Some(file) => {
                        *path = file;
                        ui.text("Selecting");
                    }
                    None => {
                        if ui.button("Select") {
                            thread::spawn(|| {
                                let game_dir = get_game_dir().expect("no game directory");
                                if let Some(file) = FileDialog::new()
                                    .set_title("Select Icon")
                                    .set_directory(&game_dir)
                                    .add_filter("Image", &["png", "jpg", "jpeg"])
                                    .pick_file()
                                {
                                    // try to get the relative path from game directory
                                    let file = match file.strip_prefix(game_dir) {
                                        Ok(relative) => relative.to_path_buf(),
                                        Err(_) => {
                                            log::warn!("Absolute icon path \"{}\"", file.display());
                                            file
                                        }
                                    };
                                    *FILE.lock().unwrap() = Some(file);
                                }
                            });
                        }
                    }
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
