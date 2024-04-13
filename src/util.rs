use nexus::imgui::{sys, InputTextFlags, Selectable, StyleVar, TreeNode, Ui};
use std::{ffi::CString, mem};
use strum::IntoEnumIterator;

pub fn input_u32(ui: &Ui, label: impl AsRef<str>, value: &mut u32) {
    let mut int = *value as _;
    if ui.input_int(label, &mut int).step(0).step_fast(0).build() {
        *value = int as _;
    }
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

pub fn enum_combo<T>(ui: &Ui, label: impl AsRef<str>, current: &mut T) -> bool
where
    T: AsRef<str> + IntoEnumIterator,
{
    let mut changed = false;
    if let Some(_token) = ui.begin_combo(label, current.as_ref()) {
        for entry in T::iter() {
            // distinguish only discriminants
            let selected = mem::discriminant(&entry) == mem::discriminant(current);
            if Selectable::new(entry.as_ref()).selected(selected).build(ui) {
                changed = true;
                *current = entry;
            }

            // handle focus
            if selected {
                ui.set_item_default_focus();
            }
        }
    }
    changed
}

pub fn tree_select(
    ui: &Ui,
    id: impl AsRef<str>,
    label: impl AsRef<str>,
    selected: bool,
    leaf: bool,
    children: impl FnOnce(),
) -> bool {
    let _style = ui.push_style_var(StyleVar::IndentSpacing(10.0));
    let token = TreeNode::new(id)
        .label::<&str, _>(label.as_ref()) // FIXME: unnecessary type param in imgui-rs
        .open_on_arrow(true)
        .selected(selected)
        .leaf(leaf)
        .tree_push_on_open(!leaf)
        .push(ui);
    let clicked = ui.is_item_clicked() && !ui.is_item_toggled_open();
    if token.is_some() {
        children();
    }
    clicked
}

pub fn item_context_menu(str_id: impl Into<String>, contents: impl FnOnce()) {
    if let Ok(str_id) = CString::new(str_id.into()) {
        if unsafe {
            sys::igBeginPopupContextItem(
                str_id.as_ptr(),
                sys::ImGuiPopupFlags_MouseButtonRight as _,
            )
        } {
            contents();
            unsafe { sys::igEndPopup() };
        }
    }
}
