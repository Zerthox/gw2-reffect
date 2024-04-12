use nexus::imgui::{sys, InputTextFlags, Selectable, StyleColor, StyleVar, TreeNode, Ui};
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
    tree_id: impl AsRef<str>,
    select_label: impl AsRef<str>,
    selected: bool,
    leaf: bool,
    children: impl FnOnce(),
) -> bool {
    let _style = ui.push_style_var(StyleVar::IndentSpacing(10.0));
    let token = {
        let transparent = [0.0, 0.0, 0.0, 0.0];
        let _color = ui.push_style_color(StyleColor::Header, transparent);
        let _color = ui.push_style_color(StyleColor::HeaderHovered, transparent);
        let _color = ui.push_style_color(StyleColor::HeaderActive, transparent);

        TreeNode::new(tree_id)
            .label::<&str, _>("") // FIXME: unnecessary type param in imgui-rs
            .allow_item_overlap(true)
            .open_on_arrow(true)
            .default_open(false)
            .leaf(leaf)
            .push(ui)
    };

    ui.same_line();
    let clicked = Selectable::new(select_label)
        .close_popups(false)
        .selected(selected)
        .build(ui);

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
                sys::ImGuiPopupFlags_MouseButtonRight as i32,
            )
        } {
            contents();
            unsafe { sys::igEndPopup() };
        }
    }
}
