use crate::{
    colors::Color,
    context::Context,
    elements::{Element, RenderState},
};
use nexus::imgui::Ui;

/// [`Element`] tree node.
pub trait TreeNode {
    /// Returns the child [`Elements`].
    fn children(&mut self) -> Option<&mut Vec<Element>>;
}

/// [`Element`] tree node that is a leaf.
pub trait TreeLeaf {}

impl<T> TreeNode for T
where
    T: TreeLeaf,
{
    fn children(&mut self) -> Option<&mut Vec<Element>> {
        None
    }
}

/// Render UI element.
pub trait Render: RenderOptions {
    /// Renders the UI element.
    fn render(&mut self, ui: &Ui, ctx: &Context, state: &RenderState);
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

/// Associated short name.
pub trait ShortName {
    /// Returns the short name.
    fn short_name(&self) -> &'static str;
}
