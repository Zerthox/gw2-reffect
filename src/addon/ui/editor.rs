use super::Addon;
use crate::{
    colors,
    id::IdGen,
    internal::Error,
    render_util::{next_window_size_constraints, small_padding},
};
use nexus::imgui::{ChildWindow, StyleVar, Ui};

impl Addon {
    pub fn render_editor(&mut self, ui: &Ui) {
        if ui.button("Reload packs") {
            self.packs.clear();
            self.context.edit = Default::default();
            IdGen::reset();
            self.load_packs();
        }
        if ui.is_item_hovered() {
            ui.tooltip_text("Reloads from pack files on disk");
        }

        ui.same_line();
        if ui.button("Save changes") {
            self.save_packs();
        }
        if ui.is_item_hovered() {
            ui.tooltip_text("Saves all changes made to pack files on disk");
        }

        ui.same_line();
        if ui.button("Open folder") {
            self.open_addon_folder();
        }

        ui.same_line();
        if ui.button("New pack") {
            self.open_create_dialog();
        }

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

                        if self.packs.is_empty() {
                            ui.spacing();
                            ui.text("No packs loaded");
                            ui.text("Do you want to...");
                            if ui.button("Read the docs") {
                                self.open_doc("getting-started");
                            }
                            if ui.button("Install existing") {
                                self.open_packs_folder()
                            }
                            if ui.button("Create my own") {
                                self.open_create_dialog();
                            }
                        } else {
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
                        }
                    });

                next_window_size_constraints([250.0, -1.0], [f32::INFINITY, -1.0]);
                ui.same_line();
                ChildWindow::new("element-options").build(ui, || {
                    let _style = small_padding(ui);
                    for pack in &mut self.packs {
                        let rendered = pack.try_render_options(ui, &self.context.edit);
                        if rendered {
                            // end after we find the element that has to render
                            break;
                        }
                    }
                });
            });

        render_errors(
            ui,
            [
                ("Buffs", self.context.own_buffs.as_ref().err()),
                ("Resources", self.context.resources.as_ref().err()),
                ("Traits", self.context.player.traits.as_ref().err()),
            ],
        );
    }
}
fn render_errors<'a, 'b>(ui: &Ui, errors: impl IntoIterator<Item = (&'a str, Option<&'b Error>)>) {
    let [_, max_y] = ui.window_content_region_max();
    for (i, (name, err)) in errors
        .into_iter()
        .filter_map(|(name, err)| err.map(|err| (name, err)))
        .enumerate()
    {
        ui.set_cursor_pos([0.0, max_y - (1.0 + i as f32) * ui.text_line_height()]);
        ui.text_colored(colors::RED, format!("{name} Error: {err}"));
    }
}
