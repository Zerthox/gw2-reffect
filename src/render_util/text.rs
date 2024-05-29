use nexus::imgui::{sys, ImColor32, Ui};
use std::ptr;

pub fn draw_text_bg(
    ui: &Ui,
    text: impl AsRef<str>,
    pos: [f32; 2],
    font_size: f32,
    color: impl Into<ImColor32>,
) {
    let text = text.as_ref();
    let text_bytes = text.as_bytes();
    let text_ptr = text_bytes.as_ptr().cast();

    let _bg = ui.get_background_draw_list();
    unsafe {
        let bg = sys::igGetBackgroundDrawList();
        let font = sys::igGetFont();

        sys::ImDrawList_AddText_FontPtr(
            bg,
            font,
            font_size,
            pos.into(),
            color.into().into(),
            text_ptr,
            text_ptr.byte_add(text_bytes.len()),
            0.0,
            ptr::null_mut(),
        );
    }
}
