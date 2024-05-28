use super::{RenderState, TextAlign, TextDecoration};
use crate::{
    component_wise::ComponentWise,
    context::{Context, ContextUpdate},
    render_util::input_float_with_format,
    traits::{Render, RenderOptions, TreeLeaf},
    trigger::BuffTrigger,
};
use nexus::imgui::{ColorEdit, ColorPreview, InputTextFlags, Ui};
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

    #[serde(skip)]
    text_memo: Option<String>,
}

mod replace {
    pub const NAME: &str = "%n";

    pub const STACKS: &str = "%s";
}

impl Text {
    pub fn update_text(&mut self, ctx: &Context, state: &RenderState) {
        if ctx.has_update_or_edit(ContextUpdate::Buffs) {
            self.text_memo = self.buff.active_stacks_or_edit(ctx, state).map(|stacks| {
                self.text
                    .replace(replace::NAME, state.name)
                    .replace(replace::STACKS, &stacks.to_string())
            });
        }
    }
}

impl TreeLeaf for Text {}

impl Render for Text {
    fn render(&mut self, ui: &Ui, ctx: &Context, state: &RenderState) {
        self.update_text(ctx, state);

        if let Some(text) = &self.text_memo {
            ui.set_window_font_scale(self.size);

            let align = self.align.calc_pos(ui, text);
            let pos = state.pos.add(align);
            ui.set_cursor_screen_pos(pos);
            let color @ [_, _, _, alpha] = self.color;
            self.decoration.render(ui, text, [0.0, 0.0, 0.0, alpha]);
            ui.text_colored(color, text);

            ui.set_window_font_scale(1.0);
        }
    }
}

impl RenderOptions for Text {
    fn render_options(&mut self, ui: &Ui) {
        self.buff.render_options(ui);

        ui.input_text("Text", &mut self.text).build();
        if ui.is_item_hovered() {
            ui.tooltip_text("%n replaced by name");
            ui.tooltip_text("%s replaced by effect stacks");
        }

        let mut size = 100.0 * self.size;
        if input_float_with_format(
            "Size",
            &mut size,
            10.0,
            100.0,
            "%.2f",
            InputTextFlags::empty(),
        ) {
            self.size = size / 100.0;
        }

        self.align.render_combo(ui);

        ColorEdit::new("Color", &mut self.color)
            .preview(ColorPreview::Alpha)
            .build(ui);

        self.decoration.render_select(ui);
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
            decoration: TextDecoration::default(),
            text_memo: None,
        }
    }
}
