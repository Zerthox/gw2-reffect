use nexus::imgui::{sys, StyleColor, Ui};
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

pub fn delete_confirm_modal(ui: &Ui, title: impl AsRef<str>, body: impl FnOnce()) -> bool {
    let mut confirm = false;

    ui.popup_modal(title)
        .always_auto_resize(true)
        .build(ui, || {
            body();

            ui.spacing();
            {
                let _style = ui.push_style_color(StyleColor::Button, [0.60, 0.24, 0.24, 1.0]);
                let _style =
                    ui.push_style_color(StyleColor::ButtonHovered, [0.70, 0.21, 0.21, 1.0]);
                let _style = ui.push_style_color(StyleColor::ButtonActive, [0.80, 0.16, 0.16, 1.0]);
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
