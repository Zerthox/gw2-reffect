use crate::render::colors;
use nexus::imgui::{ColorStackToken, StyleColor, Ui};

pub fn helper(ui: &Ui, contents: impl FnOnce()) {
    ui.same_line();
    ui.text_disabled("(?)");
    if ui.is_item_hovered() {
        ui.tooltip(contents);
    }
}

pub fn helper_warn(ui: &Ui, tooltip: impl FnOnce()) {
    ui.same_line();
    ui.text_colored(colors::YELLOW, "(!)");
    if ui.is_item_hovered() {
        ui.tooltip(tooltip);
    }
}

#[allow(unused)]
pub fn helper_error(ui: &Ui, tooltip: impl FnOnce()) {
    ui.same_line();
    ui.text_colored(colors::RED, "(!)");
    if ui.is_item_hovered() {
        ui.tooltip(tooltip);
    }
}

pub fn helper_slider(ui: &Ui) {
    helper(ui, || ui.text("Ctrl+click to type a number"))
}

#[derive(Debug, Clone)]
#[must_use]
pub enum Validation<T> {
    Ok,
    Confirm(T),
    Warn(T),
    Error(T),
}

impl<T> Validation<T> {
    pub fn push_color<'ui>(&self, ui: &'ui Ui) -> ColorStackToken<'ui> {
        const STYLE_COLOR: StyleColor = StyleColor::FrameBg;
        const INTENSITY: f32 = 0.3;

        let color = ui.style_color(STYLE_COLOR);
        match self {
            Self::Ok | Self::Confirm(_) => ui.push_style_color(STYLE_COLOR, color),
            Self::Warn(_) => {
                ui.push_style_color(STYLE_COLOR, colors::lerp(color, colors::YELLOW, INTENSITY))
            }
            Self::Error(_) => {
                ui.push_style_color(STYLE_COLOR, colors::lerp(color, colors::RED, INTENSITY))
            }
        }
    }

    pub fn render_tooltip(&self, ui: &Ui)
    where
        T: AsRef<str>,
    {
        if ui.is_item_hovered() {
            match self {
                Self::Ok => {}
                Validation::Confirm(text) => ui.tooltip(|| ui.text_colored(colors::GREEN, text)),
                Validation::Warn(text) => ui.tooltip(|| ui.text_colored(colors::YELLOW, text)),
                Validation::Error(text) => ui.tooltip(|| ui.text_colored(colors::RED, text)),
            }
        }
    }

    pub fn for_item<R>(&self, ui: &Ui, item: impl FnOnce() -> R) -> R
    where
        T: AsRef<str>,
    {
        let _color = self.push_color(ui);
        let result = item();
        self.render_tooltip(ui);
        result
    }
}
