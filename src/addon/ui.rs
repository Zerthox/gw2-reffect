use super::Addon;
use crate::{
    context::Context,
    element::{Render, State},
    get_buffs::get_buffs,
    texture_manager::TextureManager,
};
use nexus::imgui::{Condition, StyleColor, Ui, Window};

impl Addon {
    pub fn render(&mut self, ui: &Ui) {
        self.perform_updates();

        match unsafe { get_buffs() } {
            Ok(buffs) => {
                let screen_size = ui.io().display_size;
                Window::new("##reffect-displays")
                    .position([0.0, 0.0], Condition::Always)
                    .content_size(screen_size)
                    .draw_background(false)
                    .no_decoration()
                    .no_inputs()
                    .movable(false)
                    .focus_on_appearing(false)
                    .build(ui, || {
                        let ctx = Context::new(self.editing, &self.player, buffs);
                        let mut state = State::new();

                        for pack in &mut self.packs {
                            pack.render(ui, &ctx, &mut state);
                        }
                    });
            }
            Err(err) => {
                Window::new("Reffect Error##reffect-getbuffs-error")
                    .collapsible(false)
                    .always_auto_resize(true)
                    .build(ui, || {
                        ui.text_colored([1.0, 0.0, 0.0, 1.0], format!("{err}"));
                    });
            }
        }
    }

    pub fn render_options(&mut self, ui: &Ui) {
        ui.text(format!("Packs loaded: {}", self.packs.len()));
        for pack in &mut self.packs {
            ui.checkbox(
                format!("{} by {}:", pack.name, pack.author),
                &mut pack.enabled,
            );
            ui.same_line();
            let [r, g, b, a] = ui.style_color(StyleColor::Text);
            let file = pack
                .file
                .as_path()
                .file_name()
                .unwrap_or_default()
                .to_string_lossy();
            ui.text_colored([r, g, b, a * 0.5], file);
        }

        ui.spacing();
        if ui.button("Reload pack files") {
            self.packs.clear();
            self.load_packs();
        }
        ui.same_line();
        if ui.button("Save pack changes") {
            self.save_packs();
        }

        ui.spacing();
        ui.checkbox("Edit mode", &mut self.editing);

        ui.spacing();
        if ui.button("Reload icons") {
            log::debug!("Reloading icons");
            TextureManager::clear();
            for pack in &mut self.packs {
                pack.load();
            }
        }
    }
}
