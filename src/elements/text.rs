use super::{AlignHorizontal, RenderState, TextDecoration};
use crate::{
    bounds::Bounds,
    component_wise::ComponentWise,
    context::{Context, ContextUpdate},
    render_util::{
        draw_text_bg, helper, input_float_with_format, input_text_multi_with_menu, Rect,
    },
    traits::{Render, RenderOptions},
    tree::TreeLeaf,
    trigger::{ActiveBuff, BuffTrigger},
};
use nexus::imgui::{ColorEdit, InputTextFlags, Ui};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Text {
    pub buff: BuffTrigger,
    pub text: String,
    pub size: f32,
    pub align: AlignHorizontal,
    pub color: [f32; 4],
    pub decoration: TextDecoration,

    #[serde(skip)]
    text_memo: Option<String>,
}

impl Text {
    pub fn update_text(&mut self, ctx: &Context, state: &RenderState) {
        if ctx.has_update_or_edit(ContextUpdate::Buffs) {
            self.text_memo = self
                .buff
                .active_or_edit(ctx, state)
                .map(|active| Self::process_text(&self.text, &active, ctx, state));
        }
    }

    fn process_text(text: &str, active: &ActiveBuff, ctx: &Context, state: &RenderState) -> String {
        const PREFIX: char = '%';

        let mut result = String::with_capacity(text.len()); // always same or larger size

        let mut prefix = false;
        for el in text.chars() {
            if prefix {
                prefix = false;
                match el {
                    'n' => result.push_str(&state.common.name),
                    's' => result.push_str(&active.stacks.to_string()),
                    'r' => match ctx.time_until(active.runout) {
                        Some(remain) => result.push_str(&format!("{:.1}", remain as f32 / 1000.0)),
                        None => result.push('?'),
                    },
                    'f' => {
                        let full = active.full_duration() as f32 / 1000.0;
                        result.push_str(&format!("{full:.1}"))
                    }
                    'p' => {
                        let progress = ctx
                            .progress_remaining(active.apply, active.runout)
                            .unwrap_or(0.0);
                        let percent = (100.0 * progress) as u32;
                        result.push_str(&percent.to_string());
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
        let offset = self.align.text_offset(ui, text, self.size);
        pos.add(offset)
    }
}

impl TreeLeaf for Text {}

impl Render for Text {
    fn render(&mut self, ui: &Ui, ctx: &Context, state: &RenderState) {
        self.update_text(ctx, state);

        if let Some(text) = &self.text_memo {
            let font_scale = self.size;
            let font_size = font_scale * ui.current_font_size();
            let pos = self.calc_pos(ui, state.pos, text);
            let [r, g, b, a] = self.color;
            let alpha = a * ui.clone_style().alpha;
            let color = [r, g, b, alpha];

            self.decoration
                .render(ui, text, pos, font_size, [0.0, 0.0, 0.0, alpha]);
            draw_text_bg(ui, text, pos, font_size, color);
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
                (pos, pos.add(size.mul_scalar(self.size)))
            })
            .unwrap_or_default()
    }
}

impl RenderOptions for Text {
    fn render_options(&mut self, ui: &Ui) {
        // TODO: we rely on buffs interval refreshing the text memo

        self.buff.render_options(ui);

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
            ui.text("%r for duration remaining");
            ui.text("%f for duration full");
            ui.text("%p for duration progress");
            ui.text("%% for % sign");
        });

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
            .alpha(false)
            .build(ui);

        self.decoration.render_select(ui);
    }
}

impl Default for Text {
    fn default() -> Self {
        Self {
            text: String::new(),
            buff: BuffTrigger::default(),
            align: AlignHorizontal::Center,
            size: 1.0,
            color: [1.0, 1.0, 1.0, 1.0],
            decoration: TextDecoration::default(),
            text_memo: None,
        }
    }
}
