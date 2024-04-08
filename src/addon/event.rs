use super::Addon;
use crate::{element::Pack, texture_manager::TextureManager};
use nexus::{
    gui::{register_render, RenderType},
    paths::get_addon_dir,
};
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

        Self::lock().load_packs();
    }

    pub fn unload() {
        log::info!("Reffect v{VERSION} unload");
        // TODO: enable when editor
        // Self::lock().save_packs();
    }

    pub fn load_packs(&mut self) {
        let addon_dir = get_addon_dir("reffect").expect("invalid addon directory");
        log::info!("Loading packs from \"{}\"", addon_dir.display());

        let _ = fs::create_dir(&addon_dir);
        let files = fs::read_dir(&addon_dir)
            .expect("failed to read addon directory")
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
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

    pub fn add_pack(&mut self, new: Pack) {
        let index = self.packs.partition_point(|entry| entry.layer <= new.layer);
        self.packs.insert(index, new);
    }

    pub fn save_packs(&self) {
        log::info!("Saving packs");
        for pack in &self.packs {
            pack.save_to_file();
        }
    }
}
