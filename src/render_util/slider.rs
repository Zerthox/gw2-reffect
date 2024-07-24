use nexus::imgui::{Slider, SliderFlags, Ui};

pub fn slider_percent(ui: &Ui, label: impl AsRef<str>, value: &mut f32) -> bool {
    let mut percent = *value * 100.0;
    if Slider::new(label, 0.0, 100.0)
        .flags(SliderFlags::ALWAYS_CLAMP)
        .display_format("%.1f")
        .build(ui, &mut percent)
    {
        *value = percent / 100.0;
        true
    } else {
        false
    }
}
