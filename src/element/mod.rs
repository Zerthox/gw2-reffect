mod anchor;
mod animation;
mod direction;
mod group;
mod icon;
mod icon_element;
mod icon_grid;
mod icon_source;
mod pack;
mod render_state;
mod text;
mod text_align;
mod text_decoration;

pub use self::{
    anchor::*, animation::*, direction::*, group::*, icon::*, icon_element::*, icon_grid::*,
    icon_source::*, pack::*, render_state::*, text::*, text_align::*, text_decoration::*,
};

use crate::context::RenderContext;
use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};

// TODO: conditions, e.g. lower opacity out of combat, color change based on stack threshold

pub trait Render {
    fn load(&mut self);

    fn render(&mut self, ui: &Ui, ctx: &RenderContext, state: &mut RenderState);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Element {
    Group(Group),
    IconGrid(IconGrid),
    Icon(IconElement),
    Text(Text),
}

impl Render for Element {
    fn load(&mut self) {
        match self {
            Self::Group(group) => group.load(),
            Self::IconGrid(group) => group.load(),
            Self::Icon(icon) => icon.load(),
            Self::Text(text) => text.load(),
        }
    }

    fn render(&mut self, ui: &Ui, ctx: &RenderContext, state: &mut RenderState) {
        match self {
            Self::Group(anchor) => anchor.render(ui, ctx, state),
            Self::IconGrid(group) => group.render(ui, ctx, state),
            Self::Icon(icon) => icon.render(ui, ctx, state),
            Self::Text(text) => text.render(ui, ctx, state),
        }
    }
}
