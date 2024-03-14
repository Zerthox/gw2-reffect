use super::Render;
use crate::add_pos;
use arcdps_imgui::{TextureId, Ui};
use std::{ffi::c_void, mem};
use windows::Win32::Graphics::Direct3D11::ID3D11Texture2D;

#[derive(Debug, Clone)]
pub struct Icon {
    texture: TextureId,
    size: [f32; 2],
    pos: [f32; 2],
}

impl Icon {
    fn from_dx11(texture: &ID3D11Texture2D, size: [f32; 2]) -> Self {
        // avoid dropping by transmuting the reference
        let ptr = *unsafe { mem::transmute::<_, &*const c_void>(texture) };
        Self {
            texture: ptr.into(),
            size,
            pos: [0.0, 0.0],
        }
    }
}

impl Render for Icon {
    fn render(&mut self, ui: Ui) {
        let cursor = ui.cursor_screen_pos();
        let pos = add_pos(cursor, self.pos);
        let end = add_pos(pos, self.size);
        let draw_list = ui.get_window_draw_list();
        draw_list.add_image(self.texture, pos, end).build();
    }
}
