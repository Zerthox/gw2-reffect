use super::{ElementType, Node, Render};
use crate::{
    context::RenderContext,
    state::{render_or_children, OptionsState, RenderState},
    util::position_input,
};
use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// TODO: conditions, e.g. lower opacity out of combat, color change based on stack threshold

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Element {
    pub name: String,
    pub offset: [f32; 2],

    #[serde(flatten)]
    pub kind: ElementType,

    #[serde(skip)]
    pub guid: Uuid,
}

impl Element {
    pub fn render(&mut self, ui: &Ui, ctx: &RenderContext, state: &mut RenderState) {
        state.with_offset(self.offset, |state| self.kind.render(ui, ctx, state))
    }

    pub fn render_select_tree(&mut self, ui: &Ui, state: &mut OptionsState) {
        state.render_select_tree(ui, self.guid, &self.name, self.kind.children())
    }

    pub fn try_render_options(&mut self, ui: &Ui, state: &OptionsState) -> bool {
        render_or_children!(self, ui, state)
    }

    pub fn render_options(&mut self, ui: &Ui) {
        ui.group(|| {
            ui.align_text_to_frame_padding();
            ui.text("Name");
            ui.same_line();
            ui.input_text("##name", &mut self.name).build();

            let [x, y] = &mut self.offset;
            position_input(ui, x, y);

            self.kind.render_options(ui);
        });
    }
}

impl Node for Element {
    fn load(&mut self) {
        self.kind.load();
    }

    fn children(&mut self) -> &mut [Element] {
        self.kind.children()
    }
}

impl Default for Element {
    fn default() -> Self {
        Self {
            name: "Unnamed".into(),
            offset: [0.0, 0.0],
            kind: ElementType::default(),
            guid: Uuid::new_v4(),
        }
    }
}
