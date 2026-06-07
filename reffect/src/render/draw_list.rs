use nexus::imgui::{
    Ui,
    sys::{self, ImDrawList},
};

pub fn push_window_clip_rect_fullscreen<'ui>(ui: &Ui<'ui>) -> impl Drop + 'ui {
    push_clip_rect_fullscreen(ui, unsafe { sys::igGetWindowDrawList() })
}

pub fn push_clip_rect_fullscreen<'ui>(
    _ui: &Ui<'ui>,
    draw_list: *mut ImDrawList,
) -> impl Drop + 'ui {
    unsafe { sys::ImDrawList_PushClipRectFullScreen(draw_list) };

    pub struct Token {
        draw_list: *mut ImDrawList,
    }

    impl Drop for Token {
        fn drop(&mut self) {
            unsafe {
                sys::ImDrawList_PopClipRect(self.draw_list);
            }
        }
    }

    Token { draw_list }
}
