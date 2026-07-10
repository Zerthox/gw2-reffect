use super::Addon;
use crate::render::Io;
use nexus::font::{add_font_from_file, font_receive};
use std::fs;

impl Addon {
    pub fn load_font_files(size: f32) {
        let dir = Self::fonts_dir();
        log::debug!("Loading font files from \"{}\"", dir.display());

        match fs::read_dir(&dir) {
            Ok(iter) => {
                for entry in iter.filter_map(|entry| entry.ok()) {
                    let path = entry.path();
                    match path.extension().and_then(|ext| ext.to_str()) {
                        Some("ttf" | "otf") => {
                            let name = path
                                .file_stem()
                                .and_then(|stem| stem.to_str())
                                .unwrap_or("REFFECT_FONT");
                            add_font_from_file(name, &path, size, None, font_receive!(|_, _| {}))
                                .revert_on_unload();
                        }
                        Some(_) => log::warn!("Unsupported font file: \"{}\"", path.display()),
                        None => {}
                    }
                }
            }
            Err(err) => log::error!("Failed to read fonts directory: {err}"),
        }
    }

    pub fn reload_fonts(&mut self, io: Io) {
        log::debug!("Reloading fonts");
        self.settings.font.load(io);
        for pack in &mut self.packs {
            pack.load_fonts(io);
        }
    }
}
