mod bounds;
pub mod colors;
mod component_wise;

pub use self::{bounds::*, component_wise::*};

use crate::{
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

/// Render debug UI.
pub trait RenderDebug {
    /// Renders debug info for the type.
    fn render_debug(&mut self, ui: &Ui);
}
