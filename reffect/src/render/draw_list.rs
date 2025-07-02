use nexus::imgui::{
    Ui,
    sys::{self, ImDrawList},
};
use std::marker::PhantomData;

pub fn push_window_clip_rect_fullscreen<'ui>(ui: &Ui<'ui>) -> ClipRectToken<'ui> {
    push_clip_rect_fullscreen(ui, unsafe { sys::igGetWindowDrawList() })
}

pub fn push_clip_rect_fullscreen<'ui>(
    _ui: &Ui<'ui>,
    draw_list: *mut ImDrawList,
) -> ClipRectToken<'ui> {
    unsafe { sys::ImDrawList_PushClipRectFullScreen(draw_list) };
    ClipRectToken::new(draw_list)
}

pub struct ClipRectToken<'a> {
    draw_list: *mut ImDrawList,
    _phantom: PhantomData<&'a ()>,
}

impl ClipRectToken<'_> {
    pub fn new(draw_list: *mut ImDrawList) -> Self {
        Self {
            draw_list,
            _phantom: PhantomData,
        }
    }
}

impl Drop for ClipRectToken<'_> {
    fn drop(&mut self) {
        unsafe {
            sys::ImDrawList_PopClipRect(self.draw_list);
        }
    }
}
