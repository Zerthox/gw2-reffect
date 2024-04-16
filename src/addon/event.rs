use super::Addon;
use crate::{elements::Pack, texture_manager::TextureManager};
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

        let _ = fs::create_dir(Self::addon_dir());
        Self::lock().load_packs();
    }

    pub fn unload() {
        log::info!("Reffect v{VERSION} unload");
        Self::lock().save_packs();
        TextureManager::unload();
    }

    pub fn load_packs(&mut self) {
        let dir = Self::addon_dir();
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

    pub fn add_pack(&mut self, new: Pack) {
        let index = self.packs.partition_point(|entry| entry.layer <= new.layer);
        self.packs.insert(index, new);
    }

    pub fn delete_pack(&mut self, index: usize) {
        let Pack { file, .. } = self.packs.remove(index);
        if let Err(err) = fs::remove_file(&file) {
            log::error!("Failed to delete pack file \"{}\": {err}", file.display());
        }
    }

    pub fn save_packs(&self) {
        log::info!("Saving packs");
        for pack in &self.packs {
            pack.save_to_file();
        }
    }
}
