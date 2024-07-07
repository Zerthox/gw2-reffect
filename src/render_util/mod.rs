mod button;
mod combo;
mod input;
mod input_text;
mod popup;
mod spinner;
mod text;
mod tree;

pub use self::{
    button::*, combo::*, input::*, input_text::*, popup::*, spinner::*, text::*, tree::*,
};

use nexus::imgui::{sys, Style, StyleStackToken, StyleVar, Ui};
use std::ptr;

pub fn next_window_size_constraints(size_min: [f32; 2], size_max: [f32; 2]) {
    unsafe {
        sys::igSetNextWindowSizeConstraints(size_min.into(), size_max.into(), None, ptr::null_mut())
    }
}

pub fn small_padding<'ui>(ui: &'ui Ui) -> StyleStackToken<'ui> {
    ui.push_style_var(StyleVar::FramePadding([2.0, 2.0]))
}

pub fn style_disabled<'ui>(ui: &'ui Ui) -> StyleStackToken<'ui> {
    ui.push_style_var(StyleVar::Alpha(0.5))
}

pub fn style_disabled_if<'ui>(ui: &'ui Ui, disabled: bool) -> Option<StyleStackToken<'ui>> {
    disabled.then(|| style_disabled(ui))
}

pub fn push_alpha_change<'ui>(ui: &'ui Ui, change: f32) -> StyleStackToken<'ui> {
    let Style { alpha, .. } = ui.clone_style();
    ui.push_style_var(StyleVar::Alpha(alpha * change))
}

pub fn cycle_progress(ui: &Ui, period_ms: u32) -> f32 {
    let time = (1000.0 * ui.time()) as u32;
    let passed = time % period_ms;
    passed as f32 / period_ms as f32
}

pub fn helper(ui: &Ui, contents: impl FnOnce()) {
    ui.text_disabled("(?)");
    if ui.is_item_hovered() {
        ui.tooltip(contents);
    }
}
