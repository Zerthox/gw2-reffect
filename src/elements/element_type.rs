use super::{Element, Group, IconElement, IconGrid, Node, Render, RenderState, Text};
use crate::context::RenderContext;
use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ElementType {
    Group(Group),
    IconGrid(IconGrid),
    Icon(IconElement),
    Text(Text),
}

impl ElementType {
    pub fn type_name(&self) -> &'static str {
        match self {
            Self::Group(_) => "Group",
            Self::IconGrid(_) => "IconGrid",
            Self::Icon(_) => "Icon",
            Self::Text(_) => "Text",
        }
    }
}

impl Node for ElementType {
    fn load(&mut self) {
        match self {
            Self::Group(group) => group.load(),
            Self::IconGrid(grid) => grid.load(),
            Self::Icon(icon) => icon.load(),
            Self::Text(text) => text.load(),
        }
    }

    fn children(&mut self) -> &mut [Element] {
        match self {
            Self::Group(group) => group.children(),
            Self::IconGrid(grid) => grid.children(),
            Self::Icon(icon) => icon.children(),
            Self::Text(text) => text.children(),
        }
    }
}

impl Render for ElementType {
    fn render(&mut self, ui: &Ui, ctx: &RenderContext, state: &RenderState) {
        match self {
            Self::Group(group) => group.render(ui, ctx, state),
            Self::IconGrid(grid) => grid.render(ui, ctx, state),
            Self::Icon(icon) => icon.render(ui, ctx, state),
            Self::Text(text) => text.render(ui, ctx, state),
        }
    }

    fn render_options(&mut self, ui: &Ui) {
        match self {
            Self::Group(group) => group.render_options(ui),
            Self::IconGrid(grid) => grid.render_options(ui),
            Self::Icon(icon) => icon.render_options(ui),
            Self::Text(text) => text.render_options(ui),
        }
    }
}

impl Default for ElementType {
    fn default() -> Self {
        Self::Icon(IconElement::default())
    }
}
