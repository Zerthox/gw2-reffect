use super::{util::add_pos, Context, IconSource};
use crate::trigger::{BuffTrigger, Trigger};
use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Icon {
    pub name: String,
    pub buff: BuffTrigger,
    pub icon: IconSource,
}

impl Icon {
    pub fn load(&mut self) {
        self.icon.load();
    }

    pub fn is_active(&self, ctx: &Context) -> bool {
        self.buff.is_active(ctx)
    }

    pub fn render(&mut self, ui: &Ui, size: [f32; 2]) {
        let cursor = ui.cursor_screen_pos();
        let end = add_pos(cursor, size);
        let texture = self.icon.get_texture();
        let draw_list = ui.get_window_draw_list();
        draw_list.add_image(texture, cursor, end).build();
    }
}

impl Default for Icon {
    fn default() -> Self {
        Self {
            buff: BuffTrigger::default(),
            name: "Unnamed".into(),
            icon: IconSource::Empty,
        }
    }
}
