use super::{Element, Group, IconElement, IconList, RenderState, Text};
use crate::{
    context::Context,
    render_util::impl_static_variants,
    traits::{Render, RenderOptions, TreeNode},
};
use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumIter, IntoStaticStr};

#[derive(Debug, Clone, EnumIter, AsRefStr, IntoStaticStr, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ElementType {
    Group(Group),

    Icon(IconElement),

    #[serde(alias = "List")]
    #[strum(serialize = "List")]
    IconList(IconList),

    Text(Text),
}

impl_static_variants!(ElementType);

impl TreeNode for ElementType {
    fn children(&mut self) -> Option<&mut Vec<Element>> {
        match self {
            Self::Group(group) => group.children(),
            Self::Icon(icon) => icon.children(),
            Self::IconList(list) => list.children(),
            Self::Text(text) => text.children(),
        }
    }
}

impl Render for ElementType {
    fn render(&mut self, ui: &Ui, ctx: &Context, state: &RenderState) {
        match self {
            Self::Group(group) => group.render(ui, ctx, state),
            Self::Icon(icon) => icon.render(ui, ctx, state),
            Self::IconList(list) => list.render(ui, ctx, state),
            Self::Text(text) => text.render(ui, ctx, state),
        }
    }
}

impl RenderOptions for ElementType {
    fn render_options(&mut self, ui: &Ui) {
        match self {
            Self::Group(group) => group.render_options(ui),
            Self::Icon(icon) => icon.render_options(ui),
            Self::IconList(list) => list.render_options(ui),
            Self::Text(text) => text.render_options(ui),
        }
    }
}

impl Default for ElementType {
    fn default() -> Self {
        Self::Icon(IconElement::default())
    }
}
