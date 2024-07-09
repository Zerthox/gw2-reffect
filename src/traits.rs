use crate::{colors::Color, context::Context, elements::RenderState, render_util::Rect};
use nexus::imgui::Ui;

/// Render UI element.
pub trait Render<T = ()> {
    /// Renders the UI element.
    fn render(&mut self, ui: &Ui, ctx: &Context, state: &RenderState) -> T;
}

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
            .unwrap_or_default()
    }
}

/// Render options UI.
pub trait RenderOptions<T = ()> {
    /// Renders options for the type.
    fn render_options(&mut self, ui: &Ui) -> T;
}

/// Associated color.
pub trait Colored {
    /// Returns the color.
    fn colored(&self) -> Option<Color>;
}

/// Associated short name.
pub trait ShortName {
    /// Returns the short name.
    fn short_name(&self) -> &'static str;
}
