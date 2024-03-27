use super::{util::add_pos, IconSource, State};
use nexus::imgui::{TextureId, Ui};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Icon {
    pub name: String,
    pub buff: u32,
    pub icon: IconSource,

    #[serde(skip)]
    texture: Option<TextureId>,
}

impl Icon {
    pub fn load(&mut self) {
        self.texture = self.icon.load_texture();
    }

    pub fn is_loaded(&self) -> bool {
        self.texture.is_some()
    }

    pub fn needs_render(&self, state: &State) -> bool {
        self.is_loaded() && state.has_buff(self.buff)
    }

    pub fn render(&mut self, ui: &Ui, size: [f32; 2]) {
        if let Some(texture) = self.texture {
            let cursor = ui.cursor_screen_pos();
            let end = add_pos(cursor, size);
            let draw_list = ui.get_window_draw_list();
            draw_list.add_image(texture, cursor, end).build();
        }
    }
}

impl Default for Icon {
    fn default() -> Self {
        Self {
            buff: 0,
            name: "Unnamed".into(),
            icon: IconSource::Empty,
            texture: None,
        }
    }
}
