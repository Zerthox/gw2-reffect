use super::Addon;
use crate::{
    elements::{Pack, TextDecoration},
    id::IdGen,
    render_util::{
        enum_combo, input_float_with_format, input_u32, next_window_size_constraints,
        style_disabled,
    },
};
use nexus::imgui::{
    ChildWindow, ComboBoxFlags, Condition, InputTextFlags, StyleVar, TreeNodeFlags, Ui, Window,
};
use rfd::FileDialog;
use std::{
    fmt,
    sync::atomic::{AtomicBool, Ordering},
    thread,
};

impl Addon {
    pub fn render(&mut self, ui: &Ui) {
        self.context.update(ui.time()); // TODO: perform update in separate thread?

        self.render_displays(ui);

        if self.debug {
            self.render_debug(ui);
        }
    }

    pub fn render_displays(&mut self, ui: &Ui) {
        let screen_size = ui.io().display_size;
        let _style = ui.push_style_var(StyleVar::WindowPadding([0.0, 0.0]));
        Window::new("##reffect-displays")
            .position([0.0, 0.0], Condition::Always)
            .size(screen_size, Condition::Always)
            .draw_background(false)
            .no_decoration()
            .no_inputs()
            .movable(false)
            .focus_on_appearing(false)
            .build(ui, || {
                for pack in &mut self.packs {
                    pack.render(ui, &self.context);
                }
            });
    }

    pub fn render_options(&mut self, ui: &Ui) {
        if let Some(_token) = ui.tab_bar("options") {
            if let Some(_token) = ui.tab_item("Elements") {
                self.render_element_options(ui);
            }

            if let Some(_token) = ui.tab_item("Settings") {
                ui.checkbox("Allow edit in combat", &mut self.context.edit.during_combat);
                if ui.collapsing_header(
                    "Stacks Display (coming soon...)",
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

                if ui.collapsing_header("Advanced", TreeNodeFlags::SPAN_AVAIL_WIDTH) {
                    let mut buffs = (1000.0 * self.context.get_buffs_interval()) as u32;
                    if input_u32(ui, "Effect update interval", &mut buffs, 10, 100) {
                        self.context.replace_buffs_interval(buffs as f64 / 1000.0);
                    }

                    let mut player = (1000.0 * self.context.get_player_interval()) as u32;
                    if input_u32(ui, "Player update interval", &mut player, 10, 100) {
                        self.context.replace_player_interval(player as f64 / 1000.0);
                    }

                    if ui.button("Reset update intervals") {
                        self.context.reset_intervals();
                    }

                    ui.checkbox("Debug window", &mut self.debug);
                }
            }
        }
    }

    pub fn render_element_options(&mut self, ui: &Ui) {
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
                match &ctx.buffs_state {
                    Ok(()) => {
                        ui.text_colored(GREEN, "ok");
                        if ui.is_item_hovered() {
                            ui.tooltip(|| {
                                for buff in &ctx.buffs {
                                    ui.text(format!("{}x {}", buff.count, buff.id));
                                }
                            });
                        }
                    }
                    Err(err) => {
                        ui.text_colored(RED, "error");
                        if ui.is_item_hovered() {
                            ui.tooltip_text(err.to_string());
                        }
                    }
                }

                ui.text(format!("Combat: {}", ctx.ui.combat));
                ui.text(format!(
                    "Profession: {}",
                    name_or_unknown_id(ctx.player.prof)
                ));
                ui.text(format!(
                    "Specialization: {}",
                    name_or_unknown_id(ctx.player.spec)
                ));
                ui.text(format!("Race: {}", name_or_unknown_id(ctx.player.race)));
                ui.text(format!("Mount: {}", name_or_unknown_id(ctx.player.mount)));
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
                        ui.text(&pack.common.name);
                        ui.same_line();
                        ui.text_disabled(pack.common.id_string());
                    }
                }
                ui.unindent();
            });
    }
}

fn name_or_unknown_id<T, N>(value: Result<T, N>) -> String
where
    T: fmt::Display,
    N: fmt::Display,
{
    match value {
        Ok(value) => value.to_string(),
        Err(id) => format!("Unknown {id}"),
    }
}
