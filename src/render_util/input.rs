use crate::util::decode_skill;
use nexus::imgui::{sys, InputTextFlags, Ui};
use std::ffi::CString;

pub fn input_u32(
    ui: &Ui,
    label: impl AsRef<str>,
    value: &mut u32,
    step: u32,
    step_fast: u32,
) -> bool {
    let mut int = *value as _;
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

pub fn input_size(x: &mut f32, y: &mut f32) {
    input_float_with_format("Size x", x, 1.0, 10.0, "%.2f", InputTextFlags::empty());
    input_float_with_format("Size y", y, 1.0, 10.0, "%.2f", InputTextFlags::empty());
}

pub fn input_buff_id(ui: &Ui, label: impl AsRef<str>, id: &mut u32, flags: InputTextFlags) -> bool {
    let mut text = id.to_string(); // TODO: switch to faster int/float to string conversion libraries
    if ui
        .input_text(label, &mut text)
        .flags(flags | InputTextFlags::CALLBACK_RESIZE)
        .build()
    {
        if let Ok(new) = text.parse() {
            *id = new;
        } else if let Some(new) = decode_skill(text.trim()) {
            *id = new;
        }
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {}
