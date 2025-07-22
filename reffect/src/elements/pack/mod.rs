use super::{Anchor, Common, Element, ScreenAnchor};
use crate::{
    colors,
    context::EditState,
    elements::RenderCtx,
    render::{
        Bounds, delete_confirm_modal, item_context_menu, style_disabled, style_disabled_if,
        tree_select_empty,
    },
    schema::Schema,
    tree::{FontReloader, Loader, Resizer, TreeNode, VisitMut},
};
use nexus::imgui::{MenuItem, StyleColor, Ui};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Pack {
    #[serde(flatten)]
    pub common: Common,

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
        Loader.visit_pack(self);
    }

    pub fn reload_fonts(&mut self) {
        FontReloader.visit_pack(self);
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
    pub fn render(&mut self, ui: &Ui, ctx: &RenderCtx) {
        if self.common.is_visible(ctx) {
            self.common.update(ctx, None);
            {
                let _token = ctx.push_child(ui, &self.common);
                let _style = self.common.push_style(ui);

                for element in &mut self.elements {
                    element.render(ui, ctx, &self.common);
                }
            }

            if ctx.edit.is_edited(self.common.id) {
                let bounds = Bounds::combined_bounds(self.elements.iter(), ui, ctx);
                let pos = Anchor::root(ui);
                self.common.render_edit_indicators(ui, pos, bounds)
            }
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

        let mut open_delete = false;
        let mut open_resize = false;

        item_context_menu(&id, || {
            self.common.render_context_menu(ui, Some(children));

            open_resize = MenuItem::new("Resize").build(ui);

            let _color = ui.push_style_color(StyleColor::HeaderHovered, colors::DELETE_HOVER);
            open_delete = MenuItem::new("Delete").build(ui);
        });

        {
            let _style = style_disabled_if(ui, !self.common.enabled);
            self.common.render_tree_label(ui, "Pack");
        }

        if token.is_some() {
            self.common.render_tree_children(ui, state, children);
        }

        if let Some(factor) = self.common.render_resize(ui, open_resize) {
            Resizer::resize_pack(self, factor);
        }

        let title = format!("Confirm Delete##reffect{id}");
        if open_delete {
            ui.open_popup(&title);
        }
        delete_confirm_modal(ui, &title, || {
            ui.text(format!("Delete Pack {}?", self.common.name))
        })
    }

    /// Attempts to render options if selected.
    /// Returns `true` if the pack or a child rendered.
    pub fn try_render_options(&mut self, ui: &Ui, ctx: &RenderCtx) -> bool {
        let id = self.common.id;
        if ctx.edit.is_selected(id) {
            self.render_options(ui, ctx);
            return true;
        } else if ctx.edit.is_selected_parent(id) {
            for child in &mut self.elements {
                if child.try_render_options(ui, ctx) {
                    return true;
                }
            }
        }
        false
    }

    /// Renders the pack options.
    fn render_options(&mut self, ui: &Ui, ctx: &RenderCtx) {
        if let Some(_token) = ui.tab_bar(self.common.id_string()) {
            if let Some(_token) = ui.tab_item("Pack") {
                self.common.render_options(ui);

                ui.spacing();

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

            if let Some(_token) = ui.tab_item("Filter") {
                self.common.render_filters(ui, ctx);
            }

            if let Some(_token) = ui.tab_item("?") {
                self.render_debug(ui, ctx);
            }
        }
    }

    fn render_debug(&mut self, ui: &Ui, ctx: &RenderCtx) {
        self.common.render_debug(ui, ctx);

        ui.text("File:");
        if let Some(file) = self.file.file_name().and_then(|file| file.to_str()) {
            ui.same_line();
            ui.text(file);
        }

        ui.text(format!("Children: {}", self.elements.len()));
    }
}

impl TreeNode for Pack {
    fn children(&mut self) -> Option<&mut Vec<Element>> {
        Some(&mut self.elements)
    }
}

impl Default for Pack {
    fn default() -> Self {
        Self {
            common: Common {
                anchor: Anchor::Screen(ScreenAnchor::default()),
                ..Common::default()
            },
            layer: 0,
            elements: Vec::new(),
            file: PathBuf::new(),
        }
    }
}
