use super::{util::add_pos, Context, IconSource};
use crate::trigger::{BuffTrigger, Trigger};
use nexus::imgui::{ImColor32, Ui};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Icon {
    pub name: String,
    pub buff: BuffTrigger,
    pub icon: IconSource,
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

    pub fn render(&mut self, ui: &Ui, pos: [f32; 2], size: [f32; 2]) {
        if let Some(texture) = self.icon.get_texture() {
            let end = add_pos(pos, size);
            let draw_list = ui.get_window_draw_list();
            draw_list
                .add_image(texture, pos, end)
                .col(self.color())
                .build();
        }
    }
}

impl Default for Icon {
    fn default() -> Self {
        Self {
            buff: BuffTrigger::default(),
            name: "Unnamed".into(),
            icon: IconSource::Empty,
            tint: [255, 255, 255],
            opacity: 1.0,
        }
    }
}
