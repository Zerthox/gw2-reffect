use crate::{
    context::{Context, EditState},
    elements::{Bar, Group, IconElement, IconList, RenderState, Text},
    render::{Bounds, Render, RenderDebug, RenderOptions},
    render_util::{impl_static_variants, Rect},
    tree::TreeNode,
};
use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumIter, IntoStaticStr};

use super::Element;

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
    fn bounds(&self, ui: &Ui, ctx: &Context) -> Rect {
        match self {
            Self::Group(group) => group.bounds(ui, ctx),
            Self::Icon(icon) => icon.bounds(ui, ctx),
            Self::IconList(list) => list.bounds(ui, ctx),
            Self::Text(text) => text.bounds(ui, ctx),
            Self::Bar(bar) => bar.bounds(ui, ctx),
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

impl RenderDebug for ElementType {
    fn render_debug(&mut self, ui: &Ui) {
        match self {
            Self::Group(group) => group.render_debug(ui),
            Self::Icon(icon) => icon.render_debug(ui),
            Self::IconList(list) => list.render_debug(ui),
            Self::Text(text) => text.render_debug(ui),
            Self::Bar(bar) => bar.render_debug(ui),
        }
    }
}

impl Default for ElementType {
    fn default() -> Self {
        Self::Icon(IconElement::default())
    }
}
