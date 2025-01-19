use nexus::imgui::Ui;
use std::num::NonZero;

pub fn cycle_progress(ui: &Ui, period_ms: NonZero<u32>) -> f32 {
    let time = (1000.0 * ui.time()) as u32;
    let passed = time % period_ms;
    passed as f32 / period_ms.get() as f32
}
