use super::{IconSource, RenderState, TextAlign, TextDecoration};
use crate::{
    component_wise::ComponentWise,
    context::RenderContext,
    traits::{Leaf, RenderOptions},
    trigger::BuffTrigger,
};
use nexus::imgui::{ColorEdit, ColorPreview, Ui};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Icon {
    pub buff: BuffTrigger,
    pub icon: IconSource,
    pub stacks: bool,
    pub color: [f32; 4],
}

impl Icon {
    fn color(&self, ui: &Ui) -> [f32; 4] {
        let [r, g, b, a] = self.color;
        let global_alpha = ui.clone_style().alpha;
        [r, g, b, a * global_alpha]
    }

    pub fn render(&mut self, ui: &Ui, ctx: &RenderContext, state: &RenderState, size: [f32; 2]) {
        if let Some(texture) = self.icon.get_texture() {
            let half_size = size.mul_scalar(0.5);
            let start = state.pos.sub(half_size);
            let end = state.pos.add(half_size);
            let draw_list = ui.get_window_draw_list();
            let color = self.color(ui);
            draw_list.add_image(texture, start, end).col(color).build();
            drop(draw_list);

            // render stack count
            if self.stacks {
                if let Some(stacks) = self.buff.get_stacks_or_edit(ctx, state) {
                    ui.set_window_font_scale(1.0);

                    let [_, height] = size;
                    let font_scale = 0.5 * height / ui.current_font_size();
                    ui.set_window_font_scale(font_scale);
                    let text = stacks.to_string();
                    let [x_offset, _] = TextAlign::Right.calc_pos(ui, &text);
                    let pad = [1.0, 1.0];
                    let line_height = ui.text_line_height();
                    let text_pos = end.add([x_offset, -line_height]).sub(pad);

                    let white = [1.0, 1.0, 1.0, 1.0];
                    let black = [0.0, 0.0, 0.0, 1.0];
                    ui.set_cursor_screen_pos(text_pos);
                    TextDecoration::Shadow.render(ui, &text, black);
                    ui.text_colored(white, &text);

                    ui.set_window_font_scale(1.0);
                }
            }
        }
    }
}

impl Leaf for Icon {
    fn load(&mut self) {
        self.icon.load();
    }
}

impl RenderOptions for Icon {
    fn render_options(&mut self, ui: &Ui) {
        self.buff.render_options(ui);

        self.icon.render_select(ui);

        ColorEdit::new("Color", &mut self.color)
            .preview(ColorPreview::Alpha)
            .build(ui);

        ui.checkbox("Stacks", &mut self.stacks);
        // TODO: customizable stacks text offset
    }
}

impl Default for Icon {
    fn default() -> Self {
        Self {
            buff: BuffTrigger::default(),
            icon: IconSource::Empty,
            stacks: false,
            color: [1.0, 1.0, 1.0, 1.0],
        }
    }
}
