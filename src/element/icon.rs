use super::{
    util::{text_outline, ComponentWise},
    Context, IconSource, TextAlign,
};
use crate::trigger::{BuffTrigger, Trigger};
use nexus::imgui::{ImColor32, Ui};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Icon {
    pub name: String,
    pub buff: BuffTrigger,
    pub icon: IconSource,
    pub stacks: bool,
    pub tint: [u8; 3],
    pub opacity: f32,
}

impl Icon {
    pub fn load(&mut self) {
        self.icon.load();
    }

    pub fn is_active(&self, ctx: &Context) -> bool {
        self.buff.is_active(ctx)
    }

    fn color(&self) -> [f32; 4] {
        let [r, g, b] = self.tint;
        let [r, g, b, _] = ImColor32::from_rgb(r, g, b).to_rgba_f32s();
        [r, g, b, self.opacity]
    }

    pub fn render(&mut self, ui: &Ui, ctx: &Context, pos: [f32; 2], size: [f32; 2]) {
        if let Some(texture) = self.icon.get_texture() {
            let end = pos.add(size);
            let draw_list = ui.get_window_draw_list();
            let color = self.color();
            draw_list.add_image(texture, pos, end).col(color).build();
            drop(draw_list);

            // render stack count
            if self.stacks {
                if let Some(stacks @ 2..) = self.buff.get_stacks(ctx) {
                    ui.set_window_font_scale(1.0);

                    let [_, height] = size;
                    let font_scale = 0.5 * height / ui.current_font_size();
                    ui.set_window_font_scale(font_scale);
                    let text = stacks.to_string();
                    let [x_offset, _] = TextAlign::Right.calc_pos(ui, &text);
                    let pad = [2.0, 0.0]; // TODO: customizable offset?
                    let line_height = ui.text_line_height();
                    let text_pos = end.add([x_offset, -line_height]).sub(pad);

                    ui.set_cursor_screen_pos(text_pos);
                    text_outline(ui, &text, [0.0, 0.0, 0.0, self.opacity]);
                    ui.text_colored(color, &text);

                    ui.set_window_font_scale(1.0);
                }
            }
        }
    }
}

impl Default for Icon {
    fn default() -> Self {
        Self {
            name: "Unnamed".into(),
            buff: BuffTrigger::default(),
            icon: IconSource::Empty,
            stacks: false,
            tint: [255, 255, 255],
            opacity: 1.0,
        }
    }
}
