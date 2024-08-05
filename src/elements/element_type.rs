use super::{Bar, Element, Group, IconElement, IconList, RenderState, Text};
use crate::{
    bounds::Bounds,
    context::{Context, EditState},
    render_util::{impl_static_variants, Rect},
    traits::{Render, RenderOptions},
    tree::TreeNode,
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

    Bar(Bar),
}

impl_static_variants!(ElementType);

impl TreeNode for ElementType {
    fn children(&mut self) -> Option<&mut Vec<Element>> {
        match self {
            Self::Group(group) => group.children(),
            Self::Icon(icon) => icon.children(),
            Self::IconList(list) => list.children(),
            Self::Text(text) => text.children(),
            Self::Bar(bar) => bar.children(),
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
            Self::Bar(bar) => bar.render(ui, ctx, state),
        }
    }
}

impl Bounds for ElementType {
    fn bounding_box(&self, ui: &Ui, ctx: &Context, pos: [f32; 2]) -> Rect {
        match self {
            Self::Group(group) => group.bounding_box(ui, ctx, pos),
            Self::Icon(icon) => icon.bounding_box(ui, ctx, pos),
            Self::IconList(list) => list.bounding_box(ui, ctx, pos),
            Self::Text(text) => text.bounding_box(ui, ctx, pos),
            Self::Bar(bar) => bar.bounding_box(ui, ctx, pos),
        }
    }
}

impl RenderOptions for ElementType {
    fn render_options(&mut self, ui: &Ui, state: &mut EditState) {
        match self {
            Self::Group(group) => group.render_options(ui, state),
            Self::Icon(icon) => icon.render_options(ui, state),
            Self::IconList(list) => list.render_options(ui, state),
            Self::Text(text) => text.render_options(ui, state),
            Self::Bar(bar) => bar.render_options(ui, state),
        }
    }

    fn render_tabs(&mut self, ui: &Ui, state: &mut EditState) {
        match self {
            Self::Group(group) => group.render_tabs(ui, state),
            Self::Icon(icon) => icon.render_tabs(ui, state),
            Self::IconList(list) => list.render_tabs(ui, state),
            Self::Text(text) => text.render_tabs(ui, state),
            Self::Bar(bar) => bar.render_tabs(ui, state),
        }
    }
}

impl Default for ElementType {
    fn default() -> Self {
        Self::Icon(IconElement::default())
    }
}
