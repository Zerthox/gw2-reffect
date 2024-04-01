mod anchor;
mod animation;
mod direction;
mod group;
mod icon;
mod icon_element;
mod icon_group;
mod icon_source;
mod pack;
mod render_state;
mod text;
mod text_align;
mod text_decoration;

pub use self::{
    anchor::*, animation::*, direction::*, group::*, icon::*, icon_element::*, icon_group::*,
    icon_source::*, pack::*, render_state::*, text::*, text_align::*, text_decoration::*,
};

use crate::context::Context;
use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};

// TODO: conditions, e.g. lower opacity out of combat, color change based on stack threshold

pub trait Render {
    fn load(&mut self);

    fn render(&mut self, ui: &Ui, ctx: &Context, state: &mut RenderState);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Element {
    Group(Group),
    IconGroup(IconGroup),
    Icon(IconElement),
    Text(Text),
    Animation(Animation),
}

impl Render for Element {
    fn load(&mut self) {
        match self {
            Self::Group(anchor) => anchor.load(),
            Self::IconGroup(group) => group.load(),
            Self::Icon(icon) => icon.load(),
            Self::Text(text) => text.load(),
            Self::Animation(animation) => animation.load(),
        }
    }

    fn render(&mut self, ui: &Ui, ctx: &Context, state: &mut RenderState) {
        match self {
            Self::Group(anchor) => anchor.render(ui, ctx, state),
            Self::IconGroup(group) => group.render(ui, ctx, state),
            Self::Icon(icon) => icon.render(ui, ctx, state),
            Self::Text(text) => text.render(ui, ctx, state),
            Self::Animation(animation) => animation.render(ui, ctx, state),
        }
    }
}
