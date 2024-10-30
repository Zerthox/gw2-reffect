use nexus::imgui::{Style, StyleStackToken, StyleVar, Ui};

pub fn small_padding<'ui>(ui: &'ui Ui) -> StyleStackToken<'ui> {
    ui.push_style_var(StyleVar::FramePadding([2.0, 2.0]))
}

pub fn style_disabled<'ui>(ui: &'ui Ui) -> StyleStackToken<'ui> {
    ui.push_style_var(StyleVar::Alpha(0.5))
}

pub fn style_disabled_if<'ui>(ui: &'ui Ui, disabled: bool) -> Option<StyleStackToken<'ui>> {
    disabled.then(|| style_disabled(ui))
}

pub fn push_alpha_change<'ui>(ui: &'ui Ui, change: f32) -> StyleStackToken<'ui> {
    let Style { alpha, .. } = ui.clone_style();
    ui.push_style_var(StyleVar::Alpha(alpha * change))
}
