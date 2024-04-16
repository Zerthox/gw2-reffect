use super::Addon;
use crate::{
    elements::Pack,
    render_util::{input_u32, next_window_size_constraints},
};
use nexus::imgui::{ChildWindow, CollapsingHeader, Condition, StyleVar, TreeNodeFlags, Ui, Window};
use rfd::FileDialog;
use std::thread;

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
        if let Some(_token) = ui.tab_bar("options") {
            if let Some(_token) = ui.tab_item("Elements") {
                self.render_element_options(ui);
            }

            if let Some(_token) = ui.tab_item("Settings") {
                if CollapsingHeader::new("Advanced")
                    .flags(TreeNodeFlags::SPAN_AVAIL_WIDTH)
                    .build(ui)
                {
                    let mut buffs = (1000.0 * self.context.get_buffs_interval()) as u32;
                    if input_u32(ui, "Buffs update interval", &mut buffs, 10, 100) {
                        self.context.replace_buffs_interval((buffs / 1000) as f64);
                    }

                    let mut player = (1000.0 * self.context.get_player_interval()) as u32;
                    if input_u32(ui, "Player update interval", &mut player, 10, 100) {
                        self.context
                            .replace_player_intervals((player / 1000) as f64);
                    }
                }
            }
        }
    }

    pub fn render_element_options(&mut self, ui: &Ui) {
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
            if let Err(err) = open::that(Self::packs_dir()) {
                log::error!("Failed to open packs folder: {err}");
            }
        }
        ui.same_line();
        if ui.button("New pack") {
            // just spawn a thread to not have to deal with futures
            thread::spawn(move || {
                if let Some(file) = FileDialog::new()
                    .set_title("Save Pack")
                    .set_directory(Self::packs_dir())
                    .add_filter("JSON", &["json"])
                    .save_file()
                {
                    // TODO: verify file is in packs folder?
                    if let Some(pack) = Pack::create(file) {
                        Self::lock().add_pack(pack);
                    }
                }
            });
        }

        ui.same_line();
        ui.checkbox("Debug window", &mut self.debug);

        ui.spacing();

        ChildWindow::new("editor")
            .horizontal_scrollbar(true)
            .build(ui, || {
                next_window_size_constraints([200.0, -1.0], [300.0, -1.0]);
                ChildWindow::new("element-select")
                    .size([0.33 * ui.window_content_region_width(), 0.0])
                    .build(ui, || {
                        // TODO: search?
                        ui.text_disabled("Select Element");
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

                next_window_size_constraints([250.0, -1.0], [f32::INFINITY, -1.0]);
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
