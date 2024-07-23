use crate::{context::Context, render_util::Rect};
use nexus::imgui::Ui;

/// UI element bounds.
pub trait Bounds {
    /// Calculates the bounding box of the element.
    fn bounding_box(&self, ui: &Ui, ctx: &Context, pos: [f32; 2]) -> Rect;

    /// Calculates the combined bounding box of the elements.
    fn combined_bounds<'a>(
        iter: impl IntoIterator<Item = &'a Self>,
        ui: &Ui,
        ctx: &Context,
        pos: [f32; 2],
    ) -> Rect
    where
        Self: 'a,
    {
        iter.into_iter()
            .map(|el| el.bounding_box(ui, ctx, pos))
            .reduce(|a, b| {
                let ([a1, a2], [a3, a4]) = a;
                let ([b1, b2], [b3, b4]) = b;
                ([a1.min(b1), a2.min(b2)], [a3.max(b3), a4.max(b4)])
            })
            .unwrap_or((pos, pos))
    }
}
