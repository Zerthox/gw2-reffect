use nexus::imgui::{Slider, SliderFlags, Ui};

pub fn slider_percent_capped(ui: &Ui, label: impl AsRef<str>, value: &mut f32, cap: f32) -> bool {
    let mut percent = *value * 100.0;
    if Slider::new(label, 0.0, cap)
        .flags(SliderFlags::ALWAYS_CLAMP)
        .display_format("%.2f")
        .build(ui, &mut percent)
    {
        *value = percent / 100.0;
        true
    } else {
        false
    }
}

pub fn slider_percent(ui: &Ui, label: impl AsRef<str>, value: &mut f32) -> bool {
    slider_percent_capped(ui, label, value, 100.0)
}
