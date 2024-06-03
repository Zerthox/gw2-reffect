use super::{Element, Group, IconElement, IconGrid, RenderState, Text};
use crate::{
    context::Context,
    render_util::impl_static_variants,
    traits::{Render, RenderOptions, TreeNode},
};
use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumIter, IntoStaticStr};

// TODO dynamic grid, fixed grid for all elements

#[derive(Debug, Clone, EnumIter, AsRefStr, IntoStaticStr, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ElementType {
    Group(Group),

    #[strum(serialize = "Grid")]
    IconGrid(IconGrid),

    Icon(IconElement),

    Text(Text),
}

impl_static_variants!(ElementType);

impl TreeNode for ElementType {
    fn children(&mut self) -> Option<&mut Vec<Element>> {
        match self {
            Self::Group(group) => group.children(),
            Self::IconGrid(grid) => grid.children(),
            Self::Icon(icon) => icon.children(),
            Self::Text(text) => text.children(),
        }
    }
}

impl Render for ElementType {
    fn render(&mut self, ui: &Ui, ctx: &Context, state: &RenderState) {
        match self {
            Self::Group(group) => group.render(ui, ctx, state),
            Self::IconGrid(grid) => grid.render(ui, ctx, state),
            Self::Icon(icon) => icon.render(ui, ctx, state),
            Self::Text(text) => text.render(ui, ctx, state),
        }
    }
}

impl RenderOptions for ElementType {
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
