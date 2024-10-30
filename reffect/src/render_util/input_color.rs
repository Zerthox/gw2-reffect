use nexus::imgui::{ColorEdit, ColorPreview, Ui};

pub fn input_color_alpha(ui: &Ui, label: impl AsRef<str>, color: &mut [f32; 4]) -> bool {
    ColorEdit::new(label, color)
        .preview(ColorPreview::Alpha)
        .alpha_bar(true)
        .build(ui)
}
