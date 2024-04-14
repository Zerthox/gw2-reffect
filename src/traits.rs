use crate::{
    colors::Color,
    context::RenderContext,
    elements::{Element, RenderState},
};
use nexus::imgui::Ui;

/// [`Element`] tree node.
pub trait Node {
    /// Performs necessary loads.
    fn load(&mut self) {
        for child in self.children() {
            child.load();
        }
    }

    /// Returns the child [`Elements`].
    fn children(&mut self) -> &mut [Element];
}

/// [`Element`] tree node that is a leaf.
pub trait Leaf {
    /// Performs necessary loads.
    fn load(&mut self);
}

impl<T> Node for T
where
    T: Leaf,
{
    fn load(&mut self) {
        Leaf::load(self)
    }

    fn children(&mut self) -> &mut [Element] {
        &mut []
    }
}

/// Render UI element.
pub trait Render: RenderOptions {
    /// Renders the UI element.
    fn render(&mut self, ui: &Ui, ctx: &RenderContext, state: &RenderState);
}

/// Render options UI.
pub trait RenderOptions {
    /// Renders options for the type.
    fn render_options(&mut self, ui: &Ui);
}

/// Associated color.
pub trait Colored {
    /// Returns the color.
    fn colored(&self) -> Option<Color>;
}
