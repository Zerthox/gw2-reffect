use super::Addon;
use crate::{
    elements::{Pack, TextDecoration},
    id::IdGen,
    render_util::{
        enum_combo, input_float_with_format, input_u32, next_window_size_constraints,
        style_disabled,
    },
    traits::Colored,
};
use nexus::imgui::{
    ChildWindow, ComboBoxFlags, InputTextFlags, StyleColor, StyleVar, TreeNodeFlags, Ui, Window,
};
use rfd::FileDialog;
use std::{
    fmt,
    sync::atomic::{AtomicBool, Ordering},
    thread,
};

impl Addon {
    pub fn render(&mut self, ui: &Ui) {
        self.context.update(); // TODO: perform update in separate thread?

        self.render_displays(ui);

        if self.debug {
            self.render_debug(ui);
        }

        self.context.edit.reset_allowed();
    }

    pub fn render_displays(&mut self, ui: &Ui) {
        for pack in &mut self.packs {
            pack.render(ui, &self.context);
        }
    }

    pub fn render_options(&mut self, ui: &Ui) {
        self.context.edit.update_allowed(&self.context.ui);

        if let Some(_token) = ui.tab_bar("options") {
            if let Some(_token) = ui.tab_item("Editor") {
                self.render_editor(ui);
            }

            if let Some(_token) = ui.tab_item("Settings") {
                ui.checkbox(
                    "Allow edit mode in combat",
                    &mut self.context.edit.during_combat,
                );

                ui.checkbox(
                    "Show all pack elements during edit mode",
                    &mut self.context.edit.show_all,
                );

                if ui.collapsing_header(
                    "Stacks Display (WIP)",
                    TreeNodeFlags::SPAN_AVAIL_WIDTH | TreeNodeFlags::DEFAULT_OPEN,
                ) {
                    // TODO: stacks settings
                    let _style = style_disabled(ui);
                    enum_combo(
                        ui,
                        "Decoration",
                        &mut TextDecoration::Shadow,
                        ComboBoxFlags::empty(),
                    );

                    input_float_with_format(
                        "Size",
                        &mut 100.0,
                        1.0,
                        10.0,
                        "%.2f",
                        InputTextFlags::READ_ONLY,
                    );

                    input_float_with_format(
                        "Position x",
                        &mut 0.0,
                        10.0,
                        100.0,
                        "%.2f",
                        InputTextFlags::READ_ONLY,
                    );
                    input_float_with_format(
                        "Position y",
                        &mut 0.0,
                        10.0,
                        100.0,
                        "%.2f",
                        InputTextFlags::READ_ONLY,
                    );
                }

                // TODO: duration settings

                if ui.collapsing_header("Advanced", TreeNodeFlags::SPAN_AVAIL_WIDTH) {
                    let mut buffs = self.context.get_buffs_interval();
                    if input_u32(ui, "Effect update interval", &mut buffs, 10, 100) {
                        self.context.replace_buffs_interval(buffs);
                    }

                    let mut player = self.context.get_player_interval();
                    if input_u32(ui, "Player update interval", &mut player, 10, 100) {
                        self.context.replace_player_interval(player);
                    }

                    if ui.button("Reset update intervals") {
                        self.context.reset_intervals();
                    }

                    ui.checkbox("Debug window", &mut self.debug);
                }
            }
        }
    }

    pub fn render_editor(&mut self, ui: &Ui) {
        if ui.button("Reload packs") {
            self.packs.clear();
            self.context.edit = Default::default();
            IdGen::reset();
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

        const POPUP_TITLE: &str = "Pack Creation Error";
        static ERROR: AtomicBool = AtomicBool::new(false);

        ui.same_line();
        if ui.button("New pack") {
            // just spawn a thread to not have to deal with futures
            thread::spawn(move || {
                let packs = Self::packs_dir();
                if let Some(file) = FileDialog::new()
                    .set_title("Save Pack")
                    .set_directory(&packs)
                    .add_filter("JSON", &["json"])
                    .save_file()
                {
                    log::debug!("request to create {}", file.display());
                    if let Some(dir) = file.parent() {
                        if dir == packs {
                            if let Some(pack) = Pack::create(file) {
                                Self::lock().add_pack(pack);
                            }
                        } else {
                            ERROR.store(true, Ordering::Relaxed);
                            log::warn!("Unable to create pack in \"{}\"", dir.display());
                        }
                    }
                }
            });
        }

        if ERROR.swap(false, Ordering::Relaxed) {
            ui.open_popup(POPUP_TITLE)
        }
        ui.popup_modal(POPUP_TITLE)
            .always_auto_resize(true)
            .build(ui, || {
                ui.text("Can not create outside of packs folder");
                if ui.button("Ok") {
                    ui.close_current_popup();
                }
                ui.set_item_default_focus();
            });

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

                ui.text(format!("Show elements: {}", ctx.ui.should_show()));

                ui.text("Buffs status:");
                ui.same_line();
                match &ctx.buffs_state {
                    true => {
                        ui.text_colored(GREEN, "available");
                        if ui.is_item_hovered() {
                            ui.tooltip(|| {
                                for (id, buff) in &ctx.buffs {
                                    ui.text(format!("{}x {id}", buff.stacks));
                                    if let Some(remain) = ctx.time_until(buff.runout_time) {
                                        let full = buff.runout_time - buff.apply_time;
                                        let progress = remain as f32 / full as f32;
                                        ui.same_line();
                                        ui.text(format!(
                                            "for {:.1}/{:.1}s {:.1}%",
                                            remain as f32 / 1000.0,
                                            full as f32 / 1000.0,
                                            progress * 100.0,
                                        ));
                                    }
                                }
                            });
                        }
                    }
                    false => {
                        ui.text_colored(RED, "unavailable");
                    }
                }

                ui.text(format!("Combat: {}", ctx.ui.combat));

                ui.text("Profession:");
                ui.same_line();
                name_or_unknown_id_colored(ui, ctx.player.prof);

                ui.text("Specialization:");
                ui.same_line();
                name_or_unknown_id_colored(ui, ctx.player.spec);

                ui.text("Race:");
                ui.same_line();
                ui.text(match ctx.player.race {
                    Ok(value) => value.to_string(),
                    Err(id) => format!("Unknown ({id})"),
                });

                ui.text("Mount:");
                ui.same_line();
                name_or_unknown_id_colored(ui, ctx.player.mount);

                ui.text(format!("Map id: {}", ctx.map.id));
                ui.text(format!("Map category: {}", ctx.map.category));

                ui.spacing();
                ui.separator();
                ui.spacing();

                self.context.edit.debug(ui);
            });
    }
}

fn name_or_unknown_id_colored<T, N>(ui: &Ui, value: Result<T, N>)
where
    T: AsRef<str> + Colored,
    N: fmt::Display,
{
    match value {
        Ok(value) => {
            let _color = value
                .colored()
                .map(|color| ui.push_style_color(StyleColor::Text, color));
            ui.text(value);
        }
        Err(id) => ui.text(format!("Unknown ({id})")),
    }
}
