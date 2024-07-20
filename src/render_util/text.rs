use nexus::imgui::{sys, ImColor32, Ui};
use std::ptr;

pub fn draw_text_bg(
    ui: &Ui,
    text: impl AsRef<str>,
    pos: [f32; 2],
    font_scale: f32,
    color: impl Into<ImColor32>,
) {
    let range = text.as_ref().as_bytes().as_ptr_range();

    let _bg = ui.get_background_draw_list();
    unsafe {
        let bg = sys::igGetBackgroundDrawList();
        let font = sys::igGetFont();
        let font_size = sys::igGetFontSize();

        sys::ImDrawList_AddText_FontPtr(
            bg,
            font,
            font_scale * font_size,
            pos.into(),
            color.into().into(),
            range.start.cast(),
            range.end.cast(),
            0.0,
            ptr::null_mut(),
        );
    }
}
