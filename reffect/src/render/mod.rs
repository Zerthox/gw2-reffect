mod animation;
mod bounds;
mod button;
mod combo;
mod component_wise;
mod draw_list;
mod font;
mod helper;
mod input;
mod input_color;
mod input_text;
mod map_ids;
mod popup;
mod slider;
mod spinner;
mod style;
mod text;
mod tree;
mod window;

pub mod colors;

pub use self::{
    animation::*, bounds::*, button::*, combo::*, component_wise::*, draw_list::*, font::*,
    helper::*, input::*, input_color::*, input_text::*, map_ids::*, popup::*, slider::*,
    spinner::*, style::*, text::*, tree::*, window::*,
};

use crate::{context::Context, elements::RenderState};
use nexus::imgui::Ui;

pub type Point = [f32; 2];

pub type Rect = (Point, Point);

/// Render UI element.
pub trait Render<T = ()> {
    /// Renders the UI element.
    fn render(&mut self, ui: &Ui, ctx: &Context, state: &RenderState) -> T;
}

/// Render options UI.
pub trait RenderOptions<O = (), C = ()>
where
    C: Default,
{
    /// Renders options for the type.
    fn render_options(&mut self, ui: &Ui, ctx: &Context) -> O;

    /// Renders special option tabs for the type.
    fn render_tabs(&mut self, _ui: &Ui, _ctx: &Context) -> C {
        C::default()
    }
}

/// Render debug UI.
pub trait RenderDebug {
    /// Renders debug info for the type.
    // TODO: const ref instead?
    fn render_debug(&mut self, ui: &Ui, ctx: &Context);
}
