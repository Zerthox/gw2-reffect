mod decoration;
mod props;

pub use self::{decoration::*, props::*};

use super::{align::AlignHorizontal, Props, RenderState};
use crate::{
    bounds::Bounds,
    component_wise::ComponentWise,
    context::{Context, ContextUpdate, EditState},
    render_util::{
        debug_optional, draw_text_bg, font_select, helper, input_text_multi_with_menu, Font, Rect,
    },
    traits::{Render, RenderDebug, RenderOptions},
    tree::TreeLeaf,
    trigger::ProgressActive,
};
use nexus::imgui::{InputTextFlags, Ui};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Text {
    pub text: String,

    #[serde(rename = "font")]
    pub font_name: Option<String>,

    pub align: AlignHorizontal,

    #[serde(flatten)]
    pub props: Props<TextProps>,

    #[serde(skip)]
    loaded_font: Option<Font>,

    #[serde(skip)]
    frequent: bool,

    #[serde(skip)]
    text_memo: Option<String>,
}

impl Text {
    pub fn update(&mut self, ctx: &Context, state: &RenderState) {
        let active = state.trigger_active();
        self.props.update(ctx, active);
        if self.frequent || ctx.has_update_or_edit(ContextUpdate::OwnCharacter) {
            self.frequent = false; // reset frequent, only enable while active
            self.text_memo = active.map(|active| self.process_text(active, ctx, state));
        }
    }

    fn process_text(
        &mut self,
        active: &ProgressActive,
        ctx: &Context,
        state: &RenderState,
    ) -> String {
        const PREFIX: char = '%';

        let mut result = String::with_capacity(self.text.len()); // always same or larger size

        let mut prefix = false;
        let is_timed = active.is_timed();
        for el in self.text.chars() {
            if prefix {
                prefix = false;
                match el {
                    'n' => result.push_str(&state.common.name),
                    's' => result.push_str(&active.intensity().to_string()),
                    'r' => {
                        result.push_str(&active.current_text(ctx.now));
                        self.frequent = is_timed;
                    }
                    'f' => result.push_str(&active.max_text()),
                    'p' => {
                        let progress = active.progress_or_default(ctx.now);
                        result.push_str(&format!("{:.1}", (100.0 * progress)));
                        self.frequent = is_timed;
                    }
                    PREFIX => result.push(PREFIX),
                    other => {
                        result.push(PREFIX);
                        result.push(other);
                    }
                }
            } else if el == PREFIX {
                prefix = true;
            } else {
                result.push(el);
            }
        }
        if prefix {
            result.push(PREFIX); // handle ending prefix
        }

        result
    }

    fn calc_pos(&self, ui: &Ui, pos: [f32; 2], text: &str) -> [f32; 2] {
        let offset = self.align.text_offset(ui, text, self.props.scale);
        pos.add(offset)
    }

    pub fn load(&mut self) {
        self.loaded_font = self.font_name.as_ref().and_then(Font::from_name_or_warn);
    }
}

impl TreeLeaf for Text {}

impl Render for Text {
    fn render(&mut self, ui: &Ui, ctx: &Context, state: &RenderState) {
        self.update(ctx, state);

        if let Some(text) = &self.text_memo {
            let _font = self.loaded_font.map(|font| font.push());
            let font_scale = self.props.scale;
            let pos = self.calc_pos(ui, state.pos, text);
            let [r, g, b, a] = self.props.color;
            let alpha = a * ui.clone_style().alpha;
            let color = [r, g, b, alpha];

            self.props
                .decoration
                .render(ui, text, pos, font_scale, [0.0, 0.0, 0.0, alpha]);
            draw_text_bg(ui, text, pos, font_scale, color);
        }
    }
}

impl Bounds for Text {
    fn bounding_box(&self, ui: &Ui, _ctx: &Context, pos: [f32; 2]) -> Rect {
        self.text_memo
            .as_ref()
            .map(|text| {
                let pos = self.calc_pos(ui, pos, text);
                let size = ui.calc_text_size(text);
                (pos, pos.add(size.mul_scalar(self.props.scale)))
            })
            .unwrap_or_default()
    }
}

impl RenderOptions for Text {
    fn render_options(&mut self, ui: &Ui, state: &mut EditState) {
        input_text_multi_with_menu(
            ui,
            "##text",
            &mut self.text,
            [0.0, 3.0 * ui.text_line_height()],
            InputTextFlags::ALLOW_TAB_INPUT,
        );

        ui.same_line();
        ui.text("Text"); // own label to fix helper position
        helper(ui, || {
            ui.text("%n for name");
            ui.text("%s for stacks");
            ui.text("%r for progress remaining");
            ui.text("%f for progress full");
            ui.text("%p for progress percent");
            ui.text("%% for % sign");
        });

        self.align.render_combo(ui);

        if font_select(ui, "Font", &mut self.loaded_font) {
            self.font_name = self.loaded_font.map(|font| font.name_owned());
        }

        self.props.base.render_options(ui, state);
    }

    fn render_tabs(&mut self, ui: &Ui, state: &mut EditState) {
        if let Some(_token) = ui.tab_item("Condition") {
            self.props.render_condition_options(ui, state);
        }
    }
}

impl RenderDebug for Text {
    fn render_debug(&mut self, ui: &Ui) {
        debug_optional(
            ui,
            "Font",
            self.loaded_font.as_ref().map(|font| font.as_ptr()),
        );
        ui.text(format!("Frequent: {}", self.frequent));
    }
}

impl Default for Text {
    fn default() -> Self {
        Self {
            text: String::new(),
            align: AlignHorizontal::Center,
            props: Props::default(),
            font_name: None,
            loaded_font: None,
            frequent: false,
            text_memo: None,
        }
    }
}

impl Clone for Text {
    fn clone(&self) -> Self {
        Self {
            text: self.text.clone(),
            align: self.align,
            props: self.props.clone(),
            font_name: self.font_name.clone(),
            loaded_font: self.loaded_font,
            frequent: self.frequent,
            text_memo: None, // dont clone the memo
        }
    }
}
