use crate::{
    addon::Addon,
    enums::check_variant_array,
    lockbox::Lockbox,
    render::{Validation, enum_combo, input_text_simple_menu},
};
use nexus::imgui::{ComboBoxFlags, Ui};
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

    pub fn render_select(&mut self, ui: &Ui) -> bool {
        let mut reload = false;

        reload |= enum_combo(ui, "Texture", self, ComboBoxFlags::empty()).is_some();

        reload |= match self {
            Self::File(path) => Self::render_file_input(ui, path),
            Self::Url(url) => Self::render_url_input(ui, url),
        };

        reload
    }

    pub fn render_file_input(ui: &Ui, path: &mut PathBuf) -> bool {
        let mut reload = false;

        // we assume this stays in place, otherwise we consider the file dialog invalidated
        let id = path as *mut _ as usize;

        let validation = if path.is_absolute() {
            Validation::Warn("Non-shareable absolute file path")
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
                    .set_title("Select Image")
                    .set_directory(&dir)
                    .add_filter("Image", &["png", "jpg", "jpeg"])
                    .pick_file()
                {
                    // try to get the relative path from icons folder
                    let file = match file.strip_prefix(dir) {
                        Ok(relative) => relative.to_path_buf(),
                        Err(_) => {
                            log::warn!("Absolute file path \"{}\"", file.display());
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

        reload
    }

    pub fn render_url_input(ui: &Ui, url: &mut String) -> bool {
        ui.input_text("##url", url)
            .hint("https://wiki.guildwars2.com/...")
            .auto_select_all(true)
            .build();
        input_text_simple_menu(ui, "##urlctx", url);

        ui.same_line();
        let reload = ui.button("Load");
        reload
    }
}

pub trait AsTextureSource {
    fn as_texture_source(&self) -> Option<TextureSource>;
}

impl<T> AsTextureSource for T
where
    T: AsRef<TextureSource>,
{
    fn as_texture_source(&self) -> Option<TextureSource> {
        Some(self.as_ref().clone())
    }
}
