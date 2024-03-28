use super::Addon;
use crate::{
    context::Context, element::Render, get_buffs::get_buffs, texture_manager::TextureManager,
};
use nexus::imgui::{Ui, Window};

impl Addon {
    pub fn render(&mut self, ui: &Ui) {
        self.perform_updates();

        match unsafe { get_buffs() } {
            Ok(buffs) => {
                let ctx = Context::new(self.editing, &self.player, buffs);

                for pack in &mut self.packs {
                    pack.render(ui, &ctx);
                }
            }
            Err(err) => {
                Window::new("Reffect Error##reffect-getbuffs-error")
                    .always_auto_resize(true)
                    .build(ui, || {
                        ui.text_colored([1.0, 0.0, 0.0, 1.0], format!("{err}"));
                    });
            }
        }
    }

    pub fn render_options(&mut self, ui: &Ui) {
        ui.text(format!("Packs loaded: {} total", self.packs.len()));

        ui.indent();
        for pack in &self.packs {
            ui.text(format!(
                "{} by {}: {}",
                pack.name,
                pack.author,
                pack.file.display()
            ));
        }
        ui.unindent();

        if ui.button("Reload pack files") {
            self.packs.clear();
            self.load_packs();
        }
        if ui.button("Save pack changes") {
            self.save_packs();
        }

        ui.spacing();
        if ui.button("Reload icons") {
            TextureManager::clear();
            for pack in &mut self.packs {
                pack.load();
            }
        }
    }
}
