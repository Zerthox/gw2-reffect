use super::ComponentWise;
use crate::{
    context::Context,
    render_util::{Point, Rect},
};
use nexus::imgui::Ui;

/// UI element bounds.
pub trait Bounds {
    /// Calculates the relative bounding box of the element.
    fn bounds(&self, ui: &Ui, ctx: &Context) -> Rect;

    /// Calculates the bounding box of the element with the given offset.
    fn bounds_with_offset(&self, ui: &Ui, ctx: &Context, offset: Point) -> Rect {
        let (start, end) = self.bounds(ui, ctx);
        (offset.add(start), offset.add(end))
    }

    /// Calculates the combined bounding box of the elements.
    fn combined_bounds<'a>(iter: impl IntoIterator<Item = &'a Self>, ui: &Ui, ctx: &Context) -> Rect
    where
        Self: 'a,
    {
        iter.into_iter()
            .map(|el| el.bounds(ui, ctx))
            .reduce(|a, b| {
                let ([a1, a2], [a3, a4]) = a;
                let ([b1, b2], [b3, b4]) = b;
                ([a1.min(b1), a2.min(b2)], [a3.max(b3), a4.max(b4)])
            })
            .unwrap_or_default()
    }
}
