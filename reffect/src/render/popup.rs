use crate::colors;
use nexus::imgui::{StyleColor, Ui, sys};
use std::ffi::CString;

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

pub fn confirm_modal(ui: &Ui, title: impl AsRef<str>, body: impl FnOnce()) -> bool {
    let mut confirm = false;

    ui.popup_modal(title)
        .always_auto_resize(true)
        .save_settings(false)
        .build(ui, || {
            body();

            ui.spacing();

            if ui.button("Confirm") {
                confirm = true;
                ui.close_current_popup();
            }
            ui.set_item_default_focus();

            ui.same_line();
            {
                let _style = ui.push_style_color(StyleColor::Button, colors::DELETE);
                let _style = ui.push_style_color(StyleColor::ButtonHovered, colors::DELETE_HOVER);
                let _style = ui.push_style_color(StyleColor::ButtonActive, colors::DELETE_ACTIVE);
                if ui.button("Cancel") {
                    ui.close_current_popup();
                }
            }
        });

    confirm
}

pub fn delete_confirm_modal(ui: &Ui, title: impl AsRef<str>, body: impl FnOnce()) -> bool {
    let mut confirm = false;

    ui.popup_modal(title)
        .always_auto_resize(true)
        .save_settings(false)
        .build(ui, || {
            body();

            ui.spacing();
            {
                let _style = ui.push_style_color(StyleColor::Button, colors::DELETE);
                let _style = ui.push_style_color(StyleColor::ButtonHovered, colors::DELETE_HOVER);
                let _style = ui.push_style_color(StyleColor::ButtonActive, colors::DELETE_ACTIVE);
                if ui.button("Delete") {
                    confirm = true;
                    ui.close_current_popup();
                }
                ui.set_item_default_focus();
            }

            ui.same_line();
            if ui.button("Cancel") {
                ui.close_current_popup();
            }
        });

    confirm
}
