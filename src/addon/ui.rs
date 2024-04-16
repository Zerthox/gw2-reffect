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
        if ui.button("New pack") {
            // TODO: create new pack
        }

        ui.same_line();
        ui.checkbox("Debug window", &mut self.debug);

        ui.spacing();

        ChildWindow::new("element-select")
            .size([250.0, 0.0])
            .build(ui, || {
                // TODO: search?
                ui.text_disabled("Elements");
                ui.separator();
                ui.spacing();

                let _style = ui.push_style_var(StyleVar::IndentSpacing(10.0));
                let mut remove = None;
                for (i, pack) in self.packs.iter_mut().enumerate() {
                    pack.edit = false;
                    let deleted = pack.render_select_tree(ui, &mut self.context.edit);
                    if deleted {
                        remove = Some(i);
                    }
                }
                if let Some(index) = remove {
                    self.delete_pack(index);
                }
            });

        ui.same_line();
        ChildWindow::new("element-options").build(ui, || {
            let _style = ui.push_style_var(StyleVar::FramePadding([2.0, 2.0]));
            for pack in &mut self.packs {
                let rendered = pack.try_render_options(ui, &self.context.edit);
                if rendered {
                    // end after we find the element that has to render
                    pack.edit = true;
                    break;
                }
            }
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

                ui.text(format!("Show Elements: {}", ctx.ui.should_show()));

                ui.text("Buffs status:");
                ui.same_line();
                match ctx.buffs {
                    Ok(_) => ui.text_colored(GREEN, "ok"),
                    Err(err) => ui.text_colored(RED, err.to_string()),
                }

                ui.text(format!("Combat: {}", ctx.ui.combat));
                ui.text(format!(
                    "Profession: {}",
                    match ctx.player.prof {
                        Ok(prof) => prof.to_string(),
                        Err(id) => format!("Unknown {id}"),
                    }
                ));
                ui.text(format!(
                    "Specialization: {}",
                    match ctx.player.spec {
                        Ok(spec) => spec.to_string(),
                        Err(id) => format!("Unknown {id}"),
                    }
                ));
                ui.text(format!("Race: {}", ctx.player.race));
                ui.text(format!("Mount: {}", ctx.player.mount));
                ui.text(format!("Map id: {}", ctx.map.id));
                ui.text(format!("Map category: {}", ctx.map.category));

                ui.spacing();
                ui.separator();
                ui.spacing();

                self.context.edit.debug(ui);
                ui.text("Edited Packs:");
                ui.indent();
                for pack in &mut self.packs {
                    if pack.edit {
                        ui.text(format!(
                            "{} ({})",
                            pack.common.name,
                            pack.common.id_string()
                        ))
                    }
                }
                ui.unindent();
            });
    }
}
