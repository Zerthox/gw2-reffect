use super::{render_or_children, Anchor, Common, Element, RenderState};
use crate::{
    context::{EditState, RenderContext},
    render_util::{delete_confirm_modal, enum_combo, item_context_menu, tree_select_empty},
    traits::{Node, RenderOptions},
};
use nexus::imgui::{ComboBoxFlags, MenuItem, StyleVar, Ui};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::PathBuf,
};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Pack {
    pub enabled: bool, // TODO: store enabled separately in addon settings?

    #[serde(flatten)]
    pub common: Common,

    pub anchor: Anchor,
    pub layer: i32,
    pub elements: Vec<Element>,

    #[serde(skip)]
    pub file: PathBuf,

    #[serde(skip)]
    pub edit: bool,
}

impl Pack {
    pub fn load_from_file(path: impl Into<PathBuf>) -> Option<Self> {
        let path = path.into();
        let file = File::open(&path).ok()?;
        let reader = BufReader::new(file);
        match serde_json::from_reader::<_, Self>(reader) {
            Ok(mut pack) => {
                pack.file = path;
                pack.load();
                Some(pack)
            }
            Err(err) => {
                log::warn!("Failed to parse pack \"{}\": {}", path.display(), err);
                None
            }
        }
    }

    pub fn save_to_file(&self) -> Option<()> {
        let file = File::create(&self.file).ok()?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, self).ok()
    }

    /// Renders the pack.
    pub fn render(&mut self, ui: &Ui, ctx: &RenderContext) {
        if self.edit || (self.enabled && ctx.ui.should_show()) {
            let pos = self.anchor.calc_pos(ui);
            let state = RenderState::new(self.edit, pos);
            self.common.render(ui, ctx, &state, |state| {
                for element in &mut self.elements {
                    element.render(ui, ctx, state);
                }
            });
        }
    }

    /// Renders the select tree.
    /// Returns `true` if the pack should be deleted.
    #[must_use]
    pub fn render_select_tree(&mut self, ui: &Ui, state: &mut EditState) -> bool {
        let id = self.common.id_string();
        let active = state.is_active(self.common.id);
        let children = &mut self.elements;
        let (token, clicked) = tree_select_empty(ui, &id, active, false);
        if clicked {
            state.select(self.common.id);
        }

        let mut open = false;
        item_context_menu(&id, || {
            self.common.render_context_menu(ui, state, Some(children));
            open = MenuItem::new("Delete").build(ui);
        });
        let title = format!("Delete Pack?##{id}");
        if open {
            ui.open_popup(&title);
        }

        self.common.render_tree_label(ui, "Pack");
        if token.is_some() {
            self.common.render_tree_children(ui, state, children);
        }

        delete_confirm_modal(ui, &title)
    }

    /// Attempts to render options if selected.
    /// Returns `true` if the pack or a child rendered.
    pub fn try_render_options(&mut self, ui: &Ui, state: &EditState) -> bool {
        self.edit = render_or_children!(self, ui, state);
        self.edit
    }

    /// Renders the pack options.
    fn render_options(&mut self, ui: &Ui) {
        if let Some(_token) = ui.tab_bar(self.common.id_string()) {
            if let Some(_token) = ui.tab_item("Pack") {
                ui.checkbox("Enabled", &mut self.enabled);

                self.common.render_options(ui);

                enum_combo(ui, "Anchor", &mut self.anchor, ComboBoxFlags::empty());

                {
                    // TODO: layer input
                    let _style = ui.push_style_var(StyleVar::Alpha(0.5));
                    let mut layer = self.layer;
                    ui.input_int("Layer", &mut layer)
                        .step(0)
                        .step_fast(0)
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

impl Node for Pack {
    fn children(&mut self) -> Option<&mut Vec<Element>> {
        Some(&mut self.elements)
    }
}
