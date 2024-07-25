use super::{AlignHorizontal, RenderState, TextDecoration};
use crate::{
    bounds::Bounds,
    component_wise::ComponentWise,
    context::{Context, ContextUpdate, EditState},
    render_util::{
        draw_text_bg, font_select, helper, input_percent, input_text_multi_with_menu, Font, Rect,
    },
    traits::{Render, RenderOptions},
    tree::TreeLeaf,
    trigger::{ProgressActive, ProgressTrigger},
};
use nexus::imgui::{ColorEdit, InputTextFlags, Ui};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Text {
    #[serde(alias = "buff")]
    pub progress: ProgressTrigger,

    pub text: String,

    #[serde(rename = "font")]
    pub font_name: Option<String>,

    #[serde(alias = "size")]
    pub scale: f32,
    pub align: AlignHorizontal,
    pub color: [f32; 4],
    pub decoration: TextDecoration,

    #[serde(skip)]
    loaded_font: Option<Font>,

    #[serde(skip)]
    text_memo: Option<String>,
}

impl Text {
    pub fn update_text(&mut self, ctx: &Context, state: &RenderState) {
        if ctx.has_update_or_edit(ContextUpdate::OwnCharacter) {
            self.text_memo = self
                .progress
                .active_or_edit(ctx, state)
                .map(|active| Self::process_text(&self.text, &active, ctx, state));
        }
    }

    fn process_text(
        text: &str,
        active: &ProgressActive,
        ctx: &Context,
        state: &RenderState,
    ) -> String {
        const PREFIX: char = '%';

        let mut result = String::with_capacity(text.len()); // always same or larger size

        let mut prefix = false;
        for el in text.chars() {
            if prefix {
                prefix = false;
                match el {
                    'n' => result.push_str(&state.common.name),
                    's' => result.push_str(&active.intensity().to_string()),
                    'r' => result.push_str(&active.current_text(ctx.now)),
                    'f' => result.push_str(&active.max_text()),
                    'p' => {
                        let progress = active.progress_or_default(ctx.now);
                        result.push_str(&format!("{:.1}", (100.0 * progress)));
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
        let offset = self.align.text_offset(ui, text, self.scale);
        pos.add(offset)
    }

    pub fn load(&mut self) {
        self.loaded_font = self.font_name.as_ref().and_then(Font::from_name_or_warn);
    }
}

impl TreeLeaf for Text {}

impl Render for Text {
    fn render(&mut self, ui: &Ui, ctx: &Context, state: &RenderState) {
        self.update_text(ctx, state);

        if let Some(text) = &self.text_memo {
            let _font = self.loaded_font.map(|font| font.push());
            let font_scale = self.scale;
            let pos = self.calc_pos(ui, state.pos, text);
            let [r, g, b, a] = self.color;
            let alpha = a * ui.clone_style().alpha;
            let color = [r, g, b, alpha];

            self.decoration
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
                (pos, pos.add(size.mul_scalar(self.scale)))
            })
            .unwrap_or_default()
    }
}

impl RenderOptions for Text {
    fn render_options(&mut self, ui: &Ui, state: &mut EditState) {
        // TODO: we rely on buffs interval refreshing the text memo

        self.progress.render_options(ui, state);

        ui.spacing();

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

        input_percent("Scale", &mut self.scale);

        if font_select(ui, "Font", &mut self.loaded_font) {
            self.font_name = self.loaded_font.map(|font| font.name_owned());
        }

        self.align.render_combo(ui);

        ColorEdit::new("Color", &mut self.color)
            .alpha(false)
            .build(ui);

        self.decoration.render_select(ui);
    }
}

impl Default for Text {
    fn default() -> Self {
        Self {
            text: String::new(),
            progress: ProgressTrigger::default(),
            align: AlignHorizontal::Center,
            scale: 1.0,
            font_name: None,
            color: [1.0, 1.0, 1.0, 1.0],
            decoration: TextDecoration::default(),
            loaded_font: None,
            text_memo: None,
        }
    }
}
