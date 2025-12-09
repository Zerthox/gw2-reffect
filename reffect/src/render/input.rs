use super::item_context_menu;
use nexus::imgui::{InputTextFlags, MenuItem, Ui, sys};
use std::ffi::CString;

pub fn input_u32(
    ui: &Ui,
    label: impl AsRef<str>,
    value: &mut u32,
    step: u32,
    step_fast: u32,
) -> bool {
    let mut int = i32::try_from(*value).unwrap_or(i32::MAX);
    if ui
        .input_int(label, &mut int)
        .step(step as _)
        .step_fast(step_fast as _)
        .build()
        && let Ok(new) = u32::try_from(int)
    {
        *value = new;
        true
    } else {
        false
    }
}

pub fn input_float_with_format(
    label: impl AsRef<str>,
    value: &mut f32,
    step: f32,
    step_fast: f32,
    format: impl AsRef<str>,
    flags: InputTextFlags,
) -> bool {
    if let Ok(label) = CString::new(label.as_ref())
        && let Ok(format) = CString::new(format.as_ref())
    {
        unsafe {
            sys::igInputFloat(
                label.as_ptr(),
                value as *mut f32,
                step,
                step_fast,
                format.as_ptr(),
                flags.bits() as i32,
            )
        }
    } else {
        false
    }
}

pub fn input_positive_with_format(
    label: impl AsRef<str>,
    value: &mut f32,
    step: f32,
    step_fast: f32,
    format: impl AsRef<str>,
    flags: InputTextFlags,
) -> bool {
    if input_float_with_format(label, value, step, step_fast, format, flags) {
        *value = value.max(0.0);
        true
    } else {
        false
    }
}

pub fn input_pos([x, y]: &mut [f32; 2]) {
    input_float_with_format("Position x", x, 1.0, 10.0, "%.2f", InputTextFlags::empty());
    input_float_with_format("Position y", y, 1.0, 10.0, "%.2f", InputTextFlags::empty());
}

pub fn input_size([x, y]: &mut [f32; 2]) {
    input_positive_with_format("Size x", x, 1.0, 10.0, "%.2f", InputTextFlags::empty());
    input_positive_with_format("Size y", y, 1.0, 10.0, "%.2f", InputTextFlags::empty());
}

pub fn input_percent(label: impl AsRef<str>, value: &mut f32) -> bool {
    let mut percent = *value * 100.0;
    if input_positive_with_format(
        label,
        &mut percent,
        1.0,
        10.0,
        "%.2f",
        InputTextFlags::empty(),
    ) {
        *value = percent / 100.0;
        true
    } else {
        false
    }
}

pub fn input_percent_inverse(label: impl AsRef<str>, value: &mut f32) -> bool {
    let mut inverse = if *value == 0.0 { 0.0 } else { 1.0 / *value };
    if input_percent(label, &mut inverse) {
        *value = if inverse == 0.0 { 0.0 } else { 1.0 / inverse };
        true
    } else {
        false
    }
}

pub fn input_seconds(ui: &Ui, label: impl AsRef<str>, ms: &mut u32) -> bool {
    let label = label.as_ref();
    let mut secs = if *ms != u32::MAX {
        *ms as f32 / 1000.0
    } else {
        f32::INFINITY
    };
    let changed =
        input_positive_with_format(label, &mut secs, 0.5, 1.0, "%.3f", InputTextFlags::empty());
    if changed {
        *ms = if secs.is_finite() {
            (secs * 1000.0) as u32
        } else {
            0
        };
    }
    item_context_menu(format!("##ctx{label}"), || {
        if MenuItem::new("Set to infinite").build(ui) {
            *ms = u32::MAX;
        }
    });

    changed
}

pub fn input_optional<T, R>(
    ui: &Ui,
    label: impl AsRef<str>,
    value: &mut Option<T>,
    default: impl FnOnce() -> T,
    input: impl FnOnce(&mut T) -> R,
) -> Option<R> {
    let label = label.as_ref();
    let [start, _] = ui.cursor_pos();
    let width = ui.calc_item_width();

    let mut is_some = value.is_some();
    if ui.checkbox(format!("##{label}"), &mut is_some) {
        *value = is_some.then(default);
    }

    ui.same_line();
    match value {
        Some(value) => {
            let [end, _] = ui.cursor_pos();
            let moved = end - start;
            let width = width - moved;

            ui.set_next_item_width(width);
            Some(input(value))
        }
        None => {
            ui.text_disabled(label);
            None
        }
    }
}
