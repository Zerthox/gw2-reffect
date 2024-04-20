use crate::{
    colors::Color,
    context::RenderContext,
    elements::{Element, RenderState},
};
use nexus::imgui::Ui;

/// [`Element`] tree node.
pub trait Node {
    /// Performs necessary initial loads.
    fn load(&mut self) {
        if let Some(children) = self.children() {
            for child in children {
                child.load();
            }
        }
    }

    /// Performs slow updates from the [`RenderContext`].
    fn slow_update(&mut self, ctx: &RenderContext) {
        if let Some(children) = self.children() {
            for child in children {
                child.slow_update(ctx);
            }
        }
    }

    /// Returns the child [`Elements`].
    fn children(&mut self) -> Option<&mut Vec<Element>>;
}

/// [`Element`] tree node that is a leaf.
pub trait Leaf {
    /// Performs necessary loads.
    fn load(&mut self);

    /// Performs slow updates from the [`RenderContext`].
    fn slow_update(&mut self, ctx: &RenderContext);
}

impl<T> Node for T
where
    T: Leaf,
{
    fn load(&mut self) {
        Leaf::load(self)
    }

    fn slow_update(&mut self, ctx: &RenderContext) {
        Leaf::slow_update(self, ctx)
    }

    fn children(&mut self) -> Option<&mut Vec<Element>> {
        None
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

/// Associated short name.
pub trait ShortName {
    /// Returns the short name.
    fn short_name(&self) -> &'static str;
}
