mod anchor;
mod group;
mod icon;
mod icon_element;
mod icon_group;
mod icon_source;
mod pack;
mod text;
mod util;

pub use self::{
    anchor::*, group::*, icon::*, icon_element::*, icon_group::*, icon_source::*, pack::*, text::*,
};

use crate::context::Context;
use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};

pub trait Render {
    fn load(&mut self);

    fn render(&mut self, ui: &Ui, ctx: &Context);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Element {
    Group(Group),
    IconGroup(IconGroup),
    Icon(IconElement),
    Text(Text),
}

impl Render for Element {
    fn load(&mut self) {
        match self {
            Self::Group(anchor) => anchor.load(),
            Self::IconGroup(group) => group.load(),
            Self::Icon(icon) => icon.load(),
            Self::Text(text) => text.load(),
        }
    }

    fn render(&mut self, ui: &Ui, ctx: &Context) {
        match self {
            Self::Group(anchor) => anchor.render(ui, ctx),
            Self::IconGroup(group) => group.render(ui, ctx),
            Self::Icon(icon) => icon.render(ui, ctx),
            Self::Text(text) => text.render(ui, ctx),
        }
    }
}
