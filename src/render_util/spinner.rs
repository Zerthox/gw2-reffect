use super::cycle_progress;
use nexus::imgui::{sys, ImColor32, Ui};
use std::f32::consts::PI;

pub fn spinner(
    ui: &Ui,
    radius: f32,
    color: impl Into<ImColor32>,
    background: impl Into<ImColor32>,
) {
    let cursor = ui.cursor_screen_pos();
    let thickness = 0.2 * radius;
    let segments = 30;
    let progress = cycle_progress(ui, 1000);
    let start = 2.0 * PI * progress;
    let end = start + 1.5 * PI;
    let color: ImColor32 = color.into();
    let background: ImColor32 = background.into();

    let _draw_list = ui.get_window_draw_list();
    unsafe {
        let draw_list = sys::igGetWindowDrawList();

        sys::ImDrawList_PathClear(draw_list);
        sys::ImDrawList_PathArcTo(draw_list, cursor.into(), radius, 0.0, 2.0 * PI, segments);
        sys::ImDrawList_PathStroke(draw_list, background.into(), false, thickness);

        sys::ImDrawList_PathClear(draw_list);
        sys::ImDrawList_PathArcTo(draw_list, cursor.into(), radius, start, end, segments);
        sys::ImDrawList_PathStroke(draw_list, color.into(), false, thickness);
    }
}
