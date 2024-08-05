use super::{Common, Element, ScreenAnchor};
use crate::{
    bounds::Bounds,
    component_wise::ComponentWise,
    context::{Context, EditState},
    render_util::{
        delete_confirm_modal, enum_combo, item_context_menu, style_disabled, style_disabled_if,
        tree_select_empty,
    },
    schema::Schema,
    traits::RenderOptions,
    tree::{Loader, TreeNode, VisitMut},
};
use nexus::imgui::{ComboBoxFlags, MenuItem, Ui};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Pack {
    #[serde(flatten)]
    pub common: Common,

    pub anchor: ScreenAnchor,
    pub layer: i32,
    pub elements: Vec<Element>,

    #[serde(skip)]
    pub file: PathBuf,
}

impl Pack {
    pub fn create(file: PathBuf) -> Option<Self> {
        let mut pack = Self::default();
        if let Some(name) = file.file_stem() {
            pack.common.name = name.to_string_lossy().into_owned();
        }
        pack.file = file;
        pack.load();
        pack.save_to_file().then_some(pack)
    }

    pub fn load(&mut self) {
        Loader.visit_elements(&mut self.elements);
    }

    pub fn load_from_file(path: impl Into<PathBuf>) -> Option<Self> {
        let path = path.into();
        Schema::load_from_file(&path).map(|schema| {
            let mut pack = schema.into_pack();
            pack.file = path;
            pack.load();
            pack
        })
    }

    pub fn save_to_file(&self) -> bool {
        Schema::latest(self).save_to_file(&self.file)
    }

    /// Renders the pack.
    pub fn render(&mut self, ui: &Ui, ctx: &Context) {
        let edit = ctx.edit.show_all && ctx.edit.is_edited_or_parent(self.common.id);
        let anchor_pos = self.anchor.calc_pos(ui);
        self.common
            .render_initial(ui, ctx, edit, anchor_pos, |state| {
                for element in &mut self.elements {
                    element.render(ui, ctx, &state);
                }
            });

        if ctx.edit.is_edited(self.common.id) {
            let pos = anchor_pos.add(self.common.pos);
            let bounds = Bounds::combined_bounds(self.elements.iter(), ui, ctx, pos);
            self.common.render_edit_indicators(ui, pos, bounds)
        }
    }

    /// Renders the select tree.
    ///
    /// Returns `true` if the pack should be deleted.
    #[must_use]
    pub fn render_select_tree(&mut self, ui: &Ui, state: &mut EditState) -> bool {
        let id = self.common.id_string();
        let selected = state.is_selected(self.common.id);
        let children = &mut self.elements;

        let (token, selected) = {
            let _style = style_disabled_if(ui, !self.common.enabled);
            tree_select_empty(ui, &id, selected, children.is_empty())
        };
        if selected {
            state.select(self.common.id);
        }

        let mut open = false;
        item_context_menu(&id, || {
            self.common.render_context_menu(ui, state, Some(children));
            open = MenuItem::new("Delete").build(ui);
        });

        {
            let _style = style_disabled_if(ui, !self.common.enabled);
            self.common.render_tree_label(ui, "Pack");
        }

        if token.is_some() {
            self.common.render_tree_children(ui, state, children);
        }

        let title = format!("Confirm Delete##reffect{id}");
        if open {
            ui.open_popup(&title);
        }
        delete_confirm_modal(ui, &title, || {
            ui.text(format!("Delete Pack {}?", self.common.name))
        })
    }

    /// Attempts to render options if selected.
    /// Returns `true` if the pack or a child rendered.
    pub fn try_render_options(&mut self, ui: &Ui, state: &mut EditState) -> bool {
        let id = self.common.id;
        if state.is_selected(id) {
            self.render_options(ui, state);
            return true;
        } else if state.is_selected_parent(id) {
            for child in &mut self.elements {
                if child.try_render_options(ui, state) {
                    return true;
                }
            }
        }
        false
    }

    /// Renders the pack options.
    fn render_options(&mut self, ui: &Ui, state: &mut EditState) {
        if let Some(_token) = ui.tab_bar(self.common.id_string()) {
            if let Some(_token) = ui.tab_item("Pack") {
                self.common.render_options(ui, state);

                ui.spacing();

                enum_combo(ui, "Anchor", &mut self.anchor, ComboBoxFlags::empty());

                {
                    // TODO: layer input
                    let _style = style_disabled(ui);
                    ui.input_int("Layer", &mut self.layer)
                        .step(1)
                        .step_fast(10)
                        .read_only(true)
                        .build();
                }
            }
            if let Some(_token) = ui.tab_item("?") {
                self.common.render_debug(ui);

                ui.text("File:");
                if let Some(file) = self.file.file_name().and_then(|file| file.to_str()) {
                    ui.same_line();
                    ui.text_disabled(file);
                }
            }
        }
    }
}

impl TreeNode for Pack {
    fn children(&mut self) -> Option<&mut Vec<Element>> {
        Some(&mut self.elements)
    }
}
