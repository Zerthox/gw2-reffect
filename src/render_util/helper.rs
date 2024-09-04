use crate::render::colors;
use nexus::imgui::Ui;

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

pub fn helper_slider(ui: &Ui) {
    helper(ui, || ui.text("Ctrl+click to type a number"))
}
