use super::cycle_progress;
use nexus::imgui::{sys, ImColor32, Ui};
use std::f32::consts::PI;

pub fn draw_spinner_bg(
    ui: &Ui,
    pos: [f32; 2],
    radius: f32,
    color: impl Into<ImColor32>,
    background: impl Into<ImColor32>,
) {
    let _draw_list = ui.get_background_draw_list();
    draw_list_spinner(
        ui,
        unsafe { sys::igGetBackgroundDrawList() },
        pos,
        radius,
        color,
        background,
    );
}

fn draw_list_spinner(
    ui: &Ui,
    draw_list: *mut sys::ImDrawList,
    pos: [f32; 2],
    radius: f32,
    color: impl Into<ImColor32>,
    background: impl Into<ImColor32>,
) {
    let thickness = 0.2 * radius;
    let segments = 30;
    let progress = cycle_progress(ui, 1000);
    let start = 2.0 * PI * progress;
    let end = start + 1.5 * PI;

    unsafe {
        sys::ImDrawList_PathClear(draw_list);
        sys::ImDrawList_PathArcTo(draw_list, pos.into(), radius, 0.0, 2.0 * PI, segments);
        sys::ImDrawList_PathStroke(draw_list, background.into().into(), false, thickness);

        sys::ImDrawList_PathClear(draw_list);
        sys::ImDrawList_PathArcTo(draw_list, pos.into(), radius, start, end, segments);
        sys::ImDrawList_PathStroke(draw_list, color.into().into(), false, thickness);
    }
}
