use super::Addon;
use nexus::imgui::{ChildWindow, Condition, StyleVar, Ui, Window};

impl Addon {
    pub fn render(&mut self, ui: &Ui) {
        self.context.update(ui.time());

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
                let ctx = self.context.as_render();
                for pack in &mut self.packs {
                    pack.render(ui, &ctx);
                }
            });

        if self.debug {
            self.render_debug(ui);
        }
    }

    pub fn render_options(&mut self, ui: &Ui) {
        if ui.button("Reload packs") {
            self.packs.clear();
            self.load_packs();
        }
        ui.same_line();
        if ui.button("Save changes") {
            self.save_packs();
        }
        ui.same_line();
        if ui.button("Open folder") {
            if let Err(err) = open::that(Self::addon_dir()) {
                log::error!("Failed to open packs folder: {err}");
            }
        }
        ui.same_line();
        ui.checkbox("Debug window", &mut self.debug);

        ui.spacing();
        ChildWindow::new("element-select")
            .size([250.0, 0.0])
            .build(ui, || {
                ui.text_disabled("Elements");
                for pack in &mut self.packs {
                    pack.render_select_tree(ui, &mut self.context.edit);
                }
            });

        ui.same_line();
        ChildWindow::new("element-options").build(ui, || {
            let _style = ui.push_style_var(StyleVar::FramePadding([2.0, 2.0]));
            // short circuit after we find the element that has to render
            self.packs
                .iter_mut()
                .any(|pack| pack.try_render_options(ui, &self.context.edit));
        });
    }

    pub fn render_debug(&mut self, ui: &Ui) {
        Window::new("Reffect Debug")
            .collapsible(false)
            .always_auto_resize(true)
            .opened(&mut self.debug)
            .build(ui, || {
                const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
                const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

                let ctx = &self.context;

                ui.text(format!("Show elements: {}", ctx.ui.should_show()));

                ui.text("Buffs status:");
                ui.same_line();
                match ctx.buffs {
                    Ok(_) => ui.text_colored(GREEN, "ok"),
                    Err(err) => ui.text_colored(RED, err.to_string()),
                }

                ui.text(format!("Combat: {}", ctx.ui.combat));
                ui.text(format!("Player profession: {}", ctx.player.prof));
                ui.text(format!("Player specialization: {}", ctx.player.spec));
                ui.text(format!("Player race: {}", ctx.player.race));
                ui.text(format!("Player mount: {}", ctx.player.mount));
                ui.text(format!("Map id: {}", ctx.map.id));
                ui.text(format!("Map category: {}", ctx.map.category));
            });
    }
}
