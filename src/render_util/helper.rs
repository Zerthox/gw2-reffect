use nexus::imgui::Ui;

use crate::colors;

pub fn helper(ui: &Ui, contents: impl FnOnce()) {
    ui.same_line();
    ui.text_disabled("(?)");
    if ui.is_item_hovered() {
        ui.tooltip(contents);
    }
}

pub fn helper_warn(ui: &Ui, tooltip: impl FnOnce()) {
    ui.same_line();
    ui.text_colored(colors::RED, "(!)");
    if ui.is_item_hovered() {
        ui.tooltip(tooltip);
    }
}
