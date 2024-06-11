use super::Addon;
use crate::{addon::Settings, elements::Pack, texture_manager::TextureManager, util::file_name};
use nexus::gui::{register_render, RenderType};
use std::fs;

const VERSION: &str = env!("CARGO_PKG_VERSION");

impl Addon {
    pub fn load() {
        log::info!("Reffect v{VERSION} load");
        TextureManager::load();

        register_render(
            RenderType::Render,
            nexus::gui::render!(|ui| Addon::lock().render(ui)),
        )
        .revert_on_unload();

        register_render(
            RenderType::OptionsRender,
            nexus::gui::render!(|ui| Addon::lock().render_options(ui)),
        )
        .revert_on_unload();

        let _ = fs::create_dir_all(Self::packs_dir());

        let mut plugin = Self::lock();

        if let Some(settings) = Settings::try_load() {
            plugin.context.load(settings.context);
        }

        plugin.load_packs();
    }

    pub fn unload() {
        log::info!("Reffect v{VERSION} unload");
        let plugin = Self::lock();

        Settings {
            context: plugin.context.settings(),
        }
        .save();

        plugin.save_packs();

        TextureManager::unload();
    }

    pub fn load_packs(&mut self) {
        let dir = Self::packs_dir();
        log::info!("Loading packs from \"{}\"", dir.display());

        match fs::read_dir(&dir) {
            Ok(iter) => {
                let files = iter.filter_map(|entry| entry.ok()).filter(|entry| {
                    matches!(
                        entry.path().extension().and_then(|ext| ext.to_str()),
                        Some("json" | "yml" | "yaml")
                    )
                });

                for file in files {
                    if let Some(pack) = Pack::load_from_file(&file.path()) {
                        self.add_pack(pack);
                    }
                }
                log::info!("Loaded {} packs", self.packs.len());
            }

            Err(err) => log::error!("Failed to read pack directory: {err}"),
        }
    }

    pub fn add_pack(&mut self, pack: Pack) {
        let index = self
            .packs
            .partition_point(|entry| entry.layer <= pack.layer);
        self.packs.insert(index, pack);
    }

    pub fn delete_pack(&mut self, index: usize) {
        let pack = &self.packs[index];
        match fs::remove_file(&pack.file) {
            Ok(_) => {
                let pack = self.packs.remove(index);
                log::info!(
                    "Deleted pack \"{}\" file \"{}\"",
                    pack.common.name,
                    file_name(&pack.file)
                );
            }
            Err(err) => log::error!(
                "Failed to delete pack \"{}\" file \"{}\": {err}",
                pack.common.name,
                file_name(&pack.file)
            ),
        }
    }

    pub fn save_packs(&self) {
        log::info!("Saving packs");
        for pack in &self.packs {
            pack.save_to_file();
        }
    }
}
