use super::{Anchor, Common, Element, RenderState};
use crate::{
    context::{Context, EditState},
    render_util::{
        delete_confirm_modal, enum_combo, item_context_menu, style_disabled, tree_select_empty,
    },
    traits::{RenderOptions, TreeNode},
    util::file_name,
    visit::{Loader, VisitMut},
};
use nexus::imgui::{ComboBoxFlags, MenuItem, Ui};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::PathBuf,
};

// TODO: tag pack with version before serializing. maybe versioned with helper trait? struct with field? internally tagged enum?

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
}

impl Pack {
    pub fn create(file: PathBuf) -> Option<Self> {
        let mut pack = Self {
            enabled: true,
            ..Self::default()
        };
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
        let file = File::open(&path)
            .inspect_err(|err| {
                log::error!("Failed to open pack file \"{}\": {err}", file_name(&path))
            })
            .ok()?;
        let reader = BufReader::new(file);
        serde_json::from_reader::<_, Self>(reader)
            .inspect_err(|err| {
                log::warn!("Failed to parse pack file \"{}\": {err}", file_name(&path))
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
                if let Err(err) = serde_json::to_writer_pretty(writer, self) {
                    log::error!(
                        "Failed to serialize pack \"{}\" to \"{}\": {err}",
                        self.common.name,
                        file_name(&self.file)
                    );
                }
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
    pub fn render(&mut self, ui: &Ui, ctx: &Context) {
        let show = self.enabled && ctx.ui.should_show();
        let edit = ctx.edit.is_selected_or_parent(self.common.id);
        if show || edit {
            let pos = self.anchor.calc_pos(ui);
            let state = RenderState::new(pos, &self.common);
            self.common.render(ui, ctx, &state, |state| {
                for element in &mut self.elements {
                    element.render(ui, ctx, state);
                }
            });
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
        let (token, selected) = tree_select_empty(ui, &id, selected, children.is_empty());
        if selected {
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
        let id = self.common.id;
        if state.is_selected(id) {
            self.render_options(ui);
            return true;
        } else if state.is_parent(id) {
            for child in &mut self.elements {
                if child.try_render_options(ui, state) {
                    return true;
                }
            }
        }
        false
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
                    let _style = style_disabled(ui);
                    let mut layer = self.layer;
                    ui.input_int("Layer", &mut layer)
                        .step(0)
                        .step_fast(0)
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
