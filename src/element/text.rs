use super::{Context, Render, State, TextAlign, TextDecoration};
use crate::component_wise::ComponentWise;
use crate::trigger::BuffTrigger;
use nexus::imgui::{ImColor32, Ui};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Text {
    pub text: String,
    pub buff: BuffTrigger,
    pub offset: [f32; 2],
    pub align: TextAlign,
    pub size: f32,
    pub color: [u8; 4],
    pub decoration: TextDecoration,
}

mod replace {
    pub const STACKS: &str = "%stacks";
}

impl Text {
    pub fn color(&self) -> [f32; 4] {
        let [r, g, b, a] = self.color;
        ImColor32::from_rgba(r, g, b, a).to_rgba_f32s()
    }

    pub fn process_text(&self, ctx: &Context) -> Option<String> {
        self.buff
            .get_stacks(ctx)
            .map(|stacks| self.text.replace(replace::STACKS, &stacks.to_string()))
    }
}

impl Render for Text {
    fn load(&mut self) {}

    fn render(&mut self, ui: &Ui, ctx: &Context, state: &mut State) {
        if let Some(text) = self.process_text(ctx) {
            ui.set_window_font_scale(self.size);

            let offset = self.align.calc_pos(ui, &text).add(self.offset);
            let pos = state.pos.add(offset);
            ui.set_cursor_screen_pos(pos);
            let color @ [_, _, _, alpha] = self.color();
            self.decoration.render(ui, &text, [0.0, 0.0, 0.0, alpha]);
            ui.text_colored(color, &text);

            ui.set_window_font_scale(1.0);
        }
    }
}

impl Default for Text {
    fn default() -> Self {
        Self {
            text: String::new(),
            buff: BuffTrigger::default(),
            offset: [0.0, 0.0],
            align: TextAlign::Center,
            size: 1.0,
            color: [255, 255, 255, 255],
            decoration: TextDecoration::Shadow,
        }
    }
}
