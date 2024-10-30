use super::input_text_simple_menu;
use crate::chat_code::{decode_skill, decode_trait};
use nexus::imgui::{sys, InputTextFlags, Ui};
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
    {
        if let Ok(new) = u32::try_from(int) {
            *value = new;
            return true;
        }
    }
    false
}

pub fn input_float_with_format(
    label: impl Into<String>,
    value: &mut f32,
    step: f32,
    step_fast: f32,
    format: impl Into<String>,
    flags: InputTextFlags,
) -> bool {
    if let (Ok(label), Ok(format)) = (CString::new(label.into()), CString::new(format.into())) {
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
    label: impl Into<String>,
    value: &mut f32,
    step: f32,
    step_fast: f32,
    format: impl Into<String>,
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

pub fn input_percent(label: impl Into<String>, value: &mut f32) -> bool {
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

pub fn input_percent_inverse(label: impl Into<String>, value: &mut f32) -> bool {
    let mut inverse = if *value == 0.0 { 0.0 } else { 1.0 / *value };
    if input_percent(label, &mut inverse) {
        *value = if inverse == 0.0 { 0.0 } else { 1.0 / inverse };
        true
    } else {
        false
    }
}

pub fn input_seconds(label: impl Into<String>, ms: &mut u32) -> bool {
    let mut secs = *ms as f32 / 1000.0;
    if input_positive_with_format(label, &mut secs, 0.5, 1.0, "%.3f", InputTextFlags::empty()) {
        *ms = (secs * 1000.0) as u32;
        true
    } else {
        false
    }
}

pub fn input_chatcode(
    ui: &Ui,
    label: impl AsRef<str>,
    id: &mut u32,
    flags: InputTextFlags,
    decode: impl FnOnce(&str) -> Option<u32>,
) -> bool {
    let label = label.as_ref();
    let mut text = id.to_string(); // TODO: switch to faster int/float to string conversion libraries
    let changed = ui
        .input_text(label, &mut text)
        .flags(flags | InputTextFlags::AUTO_SELECT_ALL | InputTextFlags::CALLBACK_RESIZE)
        .build();
    if changed {
        if let Ok(new) = text.parse() {
            *id = new;
        } else if let Some(new) = decode(text.trim()) {
            *id = new;
        }
    }
    input_text_simple_menu(ui, format!("##{label}ctx"), &mut text);
    changed
}

pub fn input_skill_id(
    ui: &Ui,
    label: impl AsRef<str>,
    id: &mut u32,
    flags: InputTextFlags,
) -> bool {
    input_chatcode(ui, label, id, flags, decode_skill)
}

pub fn input_trait_id(
    ui: &Ui,
    label: impl AsRef<str>,
    id: &mut u32,
    flags: InputTextFlags,
) -> bool {
    input_chatcode(ui, label, id, flags, decode_trait)
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
