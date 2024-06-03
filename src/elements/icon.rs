use super::{IconSource, RenderState, TextAlign, TextDecoration};
use crate::{
    colors::{self, with_alpha},
    component_wise::ComponentWise,
    context::Context,
    render_util::{draw_spinner_bg, draw_text_bg},
    traits::RenderOptions,
    trigger::{BuffTrigger, Trigger},
};
use nexus::imgui::{ColorEdit, Style, Ui};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Icon {
    pub buff: BuffTrigger,

    #[serde(rename = "icon")]
    pub source: IconSource,

    pub stacks: bool,

    #[serde(alias = "color")]
    pub tint: [f32; 3],
}

impl Icon {
    pub fn load(&mut self) {
        self.source.load();
    }

    fn texture_color(&self, ui: &Ui) -> [f32; 4] {
        let Style { alpha, .. } = ui.clone_style();
        let [r, g, b] = self.tint;
        [r, g, b, alpha]
    }

    pub fn is_visible(&mut self, ctx: &Context, state: &RenderState) -> bool {
        self.buff.is_active_or_edit(ctx, state)
    }

    pub fn render(&mut self, ui: &Ui, ctx: &Context, state: &RenderState, size: [f32; 2]) {
        if let Some(texture) = self.source.get_texture() {
            // render icon
            let half_size = size.mul_scalar(0.5);
            let start = state.pos.sub(half_size);
            let end = state.pos.add(half_size);
            let color = self.texture_color(ui);
            ui.get_background_draw_list()
                .add_image(texture, start, end)
                .col(color)
                .build();

            // render stack count
            if self.stacks {
                if let Some(stacks) = self.buff.active_stacks_or_edit(ctx, state) {
                    let text = stacks.to_string();

                    let [_, height] = size;
                    let font_size = 0.5 * height;
                    let font_scale = font_size / ui.current_font_size();
                    let [x_offset, _] = TextAlign::Right.calc_pos(ui, &text, font_scale);
                    let pad = [1.0, 1.0];
                    let line_height = font_scale * ui.text_line_height();
                    let text_pos = end.add([x_offset, -line_height]).sub(pad);

                    let alpha = 0.8; // FIXME: animation alpha ignored
                    let color = with_alpha(colors::WHITE, alpha);
                    let shadow_color = with_alpha(colors::BLACK, alpha);

                    TextDecoration::Shadow.render(ui, &text, text_pos, font_size, shadow_color);
                    draw_text_bg(ui, &text, text_pos, font_size, color);
                }
            }
        } else {
            let [x, _] = size;
            draw_spinner_bg(
                ui,
                state.pos,
                0.4 * x,
                colors::WHITE,
                with_alpha(colors::WHITE, 0.3),
            )
        }
    }
}

impl RenderOptions for Icon {
    fn render_options(&mut self, ui: &Ui) {
        ui.spacing();
        self.buff.render_options(ui);

        ui.spacing();
        self.source.render_select(ui);

        ColorEdit::new("Tint", &mut self.tint).build(ui);

        ui.checkbox("Show Stacks", &mut self.stacks);
        // TODO: customizable stacks text offset
    }
}

impl Default for Icon {
    fn default() -> Self {
        Self {
            buff: BuffTrigger::default(),
            source: IconSource::Unknown,
            stacks: false,
            tint: [1.0, 1.0, 1.0],
        }
    }
}
