use super::Addon;
use crate::context::Context;
use nexus::imgui::{Condition, StyleColor, Ui, Window};

impl Addon {
    pub fn render(&mut self, ui: &Ui) {
        self.perform_updates();

        if let Ok(buffs) = &self.buffs {
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
                    let ctx = Context::new(self.edit_all, &self.player, buffs);
                    for pack in &mut self.packs {
                        pack.render(ui, &ctx);
                    }
                });
        }

        if self.debug {
            Window::new("Reffect Debug")
                .collapsible(false)
                .always_auto_resize(true)
                .opened(&mut self.debug)
                .build(ui, || {
                    ui.text("Buffs status:");
                    ui.same_line();
                    match self.buffs {
                        Ok(_) => ui.text_colored([0.0, 1.0, 0.0, 1.0], "Ok"),
                        Err(err) => ui.text_colored([1.0, 0.0, 0.0, 1.0], err.to_string()),
                    }
                    ui.text(format!("Show elements: {}", self.player.should_render()));
                    ui.text(format!("Player profession: {}", self.player.prof));
                    ui.text(format!("Player specialization: {}", self.player.spec));
                    ui.text(format!("Player combat: {}", self.player.combat));
                    ui.text(format!("Map id: {}", self.player.map.id));
                    ui.text(format!("Map category: {}", self.player.map.category));
                });
        }
    }

    pub fn render_options(&mut self, ui: &Ui) {
        ui.text(format!("Packs loaded: {}", self.packs.len()));
        for (i, pack) in self.packs.iter_mut().enumerate() {
            ui.checkbox(format!("{}##pack{i}", pack.name), &mut pack.enabled);
            if ui.is_item_hovered() {
                ui.tooltip(|| {
                    let [r, g, b, a] = ui.style_color(StyleColor::Text);
                    ui.text_colored([r, g, b, a * 0.5], pack.file.display().to_string());
                });
            }
            ui.same_line();
            pack.edit = if pack.edit {
                !ui.button(format!("Done##pack{i}"))
            } else {
                ui.button(format!("Edit##pack{i}"))
            };
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
        ui.checkbox("Show all", &mut self.edit_all);
        ui.checkbox("Debug window", &mut self.debug);
    }
}
