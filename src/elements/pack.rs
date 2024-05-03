use super::{render_or_children, Anchor, Common, Element, RenderState};
use crate::{
    context::{EditState, RenderContext},
    render_util::{delete_confirm_modal, enum_combo, item_context_menu, tree_select_empty},
    traits::{Node, RenderOptions},
    util::file_name,
};
use nexus::imgui::{ComboBoxFlags, MenuItem, StyleVar, Ui};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::PathBuf,
};

// TODO: tag pack with version before serializing

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
    pub fn create(file: PathBuf) -> Option<Self> {
        let mut pack = Self {
            enabled: true,
            file,
            ..Self::default()
        };
        pack.load();
        pack.save_to_file().then_some(pack)
    }

    pub fn load_from_file(path: impl Into<PathBuf>) -> Option<Self> {
        let path = path.into();
        let file = File::open(&path)
            .inspect_err(|err| {
                log::error!("Failed to open pack file \"{}\": {err}", file_name(&path))
            })
            .ok()?;
        let reader = BufReader::new(file);
        serde_json::from_reader::<_, Self>(reader)
            .inspect_err(|err| {
                log::error!("Failed to parse pack file \"{}\": {err}", file_name(&path))
            })
            .ok()
            .map(|mut pack| {
                pack.file = path;
                pack.load();
                pack
            })
    }

    pub fn save_to_file(&self) -> bool {
        match File::create(&self.file) {
            Ok(file) => {
                let writer = BufWriter::new(file);
                serde_json::to_writer_pretty(writer, self).expect("failed to serialize pack");
                true
            }
            Err(err) => {
                log::error!(
                    "Failed to save pack \"{}\" to \"{}\": {err}",
                    self.common.name,
                    file_name(&self.file)
                );
                false
            }
        }
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
        let (token, clicked) = tree_select_empty(ui, &id, active, children.is_empty());
        if clicked {
            state.select(self.common.id);
        }

        let mut open = false;
        item_context_menu(&id, || {
            self.common.render_context_menu(ui, state, Some(children));
            open = MenuItem::new("Delete").build(ui);
        });
        let title = format!("Confirm Delete##{id}");
        if open {
            ui.open_popup(&title);
        }

        self.common.render_tree_label(ui, "Pack");
        if token.is_some() {
            self.common.render_tree_children(ui, state, children);
        }

        delete_confirm_modal(ui, &title, || {
            ui.text(format!("Delete Pack {}?", self.common.name))
        })
    }

    /// Attempts to render options if selected.
    /// Returns `true` if the pack or a child rendered.
    pub fn try_render_options(&mut self, ui: &Ui, state: &EditState) -> bool {
        let found = render_or_children!(self, ui, state);
        self.edit = state.is_allowed() && found;
        found
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
