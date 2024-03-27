mod group;
mod icon;
mod icon_element;
mod icon_group;
mod icon_source;
mod text;
mod util;

pub use self::{group::*, icon::*, icon_element::*, icon_group::*, icon_source::*, text::*};

use crate::state::State;
use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::BufReader, path::Path};

pub trait Render {
    fn load(&mut self);

    fn render(&mut self, ui: &Ui, state: &State);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Element {
    Group(Group),
    IconGroup(IconGroup),
    Icon(IconElement),
    Text(Text),
}

impl Element {
    pub fn from_file(path: impl AsRef<Path>) -> Option<Self> {
        let file = File::open(path.as_ref()).ok()?;
        serde_json::from_reader(BufReader::new(file)).ok()
    }
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

    fn render(&mut self, ui: &Ui, state: &State) {
        match self {
            Self::Group(anchor) => anchor.render(ui, state),
            Self::IconGroup(group) => group.render(ui, state),
            Self::Icon(icon) => icon.render(ui, state),
            Self::Text(text) => text.render(ui, state),
        }
    }
}
