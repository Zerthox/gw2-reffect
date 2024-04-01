use super::Addon;
use nexus::imgui::{Condition, StyleColor, Ui, Window};

impl Addon {
    pub fn render(&mut self, ui: &Ui) {
        self.context.update(ui.time());

        if let Some(ctx) = self.context.as_render() {
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
                    const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
                    const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

                    let ctx = &self.context;

                    ui.text(format!("Show elements: {}", ctx.ui.should_render()));

                    ui.text("Buffs status:");
                    ui.same_line();
                    match ctx.buffs {
                        Ok(_) => ui.text_colored(GREEN, "Ok"),
                        Err(err) => ui.text_colored(RED, err.to_string()),
                    }

                    ui.text(format!("Combat: {}", ctx.ui.combat));
                    ui.text(format!("Player profession: {}", ctx.player.prof));
                    ui.text(format!("Player specialization: {}", ctx.player.spec));
                    ui.text(format!("Map id: {}", ctx.map.id));
                    ui.text(format!("Map category: {}", ctx.map.category));
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
        ui.checkbox("Show all", &mut self.context.edit);
        ui.checkbox("Debug window", &mut self.debug);
    }
}
