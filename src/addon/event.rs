use super::Addon;
use crate::{element::Pack, get_buffs::get_buffs, texture_manager::TextureManager};
use nexus::{
    data_link::get_mumble_link,
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
                self.packs.push(pack);
            }
        }
        log::info!("Loaded {} packs", self.packs.len());
    }

    pub fn save_packs(&self) {
        log::info!("Saving packs");
        for pack in &self.packs {
            pack.save_to_file();
        }
    }

    pub fn perform_updates(&mut self) {
        if let Some(mumble) = unsafe { get_mumble_link().as_ref() } {
            let tick = mumble.ui_tick;
            if self.buffs_update.triggered(tick) {
                self.buffs = unsafe { get_buffs() }.map(|buffs| buffs.into());
            }
            if self.player_update.triggered(tick) {
                self.player.update(mumble);
            }
        }
    }
}
