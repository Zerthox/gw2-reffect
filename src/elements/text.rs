use super::{Element, Node, Render, RenderState, TextAlign, TextDecoration};
use crate::{
    component_wise::ComponentWise, context::RenderContext, trigger::BuffTrigger, util::enum_combo,
};
use nexus::imgui::{ColorEdit, ColorPreview, Ui};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Text {
    pub buff: BuffTrigger,
    pub text: String,
    pub size: f32,
    pub align: TextAlign,
    pub color: [f32; 4],
    pub decoration: TextDecoration,
}

mod replace {
    pub const STACKS: &str = "%stacks";
}

impl Text {
    pub fn process_text(&self, ctx: &RenderContext, state: &RenderState) -> Option<String> {
        self.buff
            .get_stacks_or_edit(ctx, state)
            .map(|stacks| self.text.replace(replace::STACKS, &stacks.to_string()))
    }
}

impl Node for Text {
    fn children(&mut self) -> &mut [Element] {
        &mut []
    }
}

impl Render for Text {
    fn render(&mut self, ui: &Ui, ctx: &RenderContext, state: &RenderState) {
        if let Some(text) = self.process_text(ctx, state) {
            ui.set_window_font_scale(self.size);

            let align = self.align.calc_pos(ui, &text);
            let pos = state.pos.add(align);
            ui.set_cursor_screen_pos(pos);
            let color @ [_, _, _, alpha] = self.color;
            self.decoration.render(ui, &text, [0.0, 0.0, 0.0, alpha]);
            ui.text_colored(color, &text);

            ui.set_window_font_scale(1.0);
        }
    }

    fn render_options(&mut self, ui: &Ui) {
        self.buff.render_options(ui);

        ui.input_text("Text", &mut self.text).build();

        ui.input_float("Size", &mut self.size).build();

        self.align.render_combo(ui);

        ColorEdit::new("Color", &mut self.color)
            .preview(ColorPreview::Alpha)
            .build(ui);

        enum_combo(ui, "Decoration", &mut self.decoration);
    }
}

impl Default for Text {
    fn default() -> Self {
        Self {
            text: String::new(),
            buff: BuffTrigger::default(),
            align: TextAlign::Center,
            size: 1.0,
            color: [1.0, 1.0, 1.0, 1.0],
            decoration: TextDecoration::Shadow,
        }
    }
}
