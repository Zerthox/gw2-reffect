use crate::{
    colors::Color,
    context::{Context, EditState},
    elements::RenderState,
};
use nexus::imgui::Ui;

/// Render UI element.
pub trait Render<T = ()> {
    /// Renders the UI element.
    fn render(&mut self, ui: &Ui, ctx: &Context, state: &RenderState) -> T;
}

/// Render options UI.
pub trait RenderOptions<T = ()> {
    /// Renders options for the type.
    fn render_options(&mut self, ui: &Ui, state: &mut EditState) -> T;

    /// Renders special option tabs for the type.
    fn render_tabs(&mut self, _ui: &Ui, _state: &mut EditState) {}
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
