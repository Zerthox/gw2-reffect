use super::{render_or_children, Anchor, Common, Element, Node, RenderState};
use crate::{
    context::{EditState, RenderContext},
    util::enum_combo,
};
use nexus::imgui::{StyleColor, Ui};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::PathBuf,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
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
        if (self.enabled && ctx.ui.should_show()) || ctx.edit.is_active_or_parent(self.common.id) {
            let pos = self.anchor.calc_pos(ui);
            let state = RenderState::new(pos);
            self.common.render(ui, ctx, &state, |state| {
                for element in &mut self.elements {
                    element.render(ui, ctx, state);
                }
            });
        }
    }

    /// Renders the select tree.
    pub fn render_select_tree(&mut self, ui: &Ui, state: &mut EditState) {
        let child_selected = self
            .common
            .render_select_tree(ui, state, "Pack", &mut self.elements);
        if child_selected {
            state.parent_pack = self.common.id;
        }
    }

    /// Attempts to render options if selected.
    /// Returns `true` if the pack or a child rendered.
    pub fn try_render_options(&mut self, ui: &Ui, state: &EditState) -> bool {
        render_or_children!(self, ui, state)
    }

    /// Renders the pack options.
    fn render_options(&mut self, ui: &Ui) {
        ui.checkbox("Enabled", &mut self.enabled);

        ui.text("File:");
        let [r, g, b, a] = ui.style_color(StyleColor::Text);
        ui.same_line();
        ui.text_colored([r, g, b, a * 0.5], self.file.display().to_string());

        self.common.render_options(ui);

        enum_combo(ui, "Anchor", &mut self.anchor);

        ui.text("Layer");
        ui.same_line();
        ui.text_disabled("coming soon"); // TODO: layer input, then sort packs
    }
}

impl Node for Pack {
    fn children(&mut self) -> &mut [Element] {
        &mut self.elements
    }
}

impl Default for Pack {
    fn default() -> Self {
        Self {
            enabled: false,
            common: Common::default(),
            layer: 0,
            anchor: Anchor::TopLeft,
            elements: Vec::new(),
            file: PathBuf::new(),
            edit: false,
        }
    }
}
