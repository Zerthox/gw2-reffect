use super::Element;
use crate::{
    context::Context,
    elements::{Bar, Common, Group, IconElement, IconList, RenderCtx, Text},
    render::{Bounds, Rect, impl_static_variants},
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

impl ElementType {
    pub fn render(&mut self, ui: &Ui, ctx: &RenderCtx, common: &Common) {
        match self {
            Self::Group(group) => group.render(ui, ctx, common),
            Self::Icon(icon) => icon.render(ui, ctx, common),
            Self::IconList(list) => list.render(ui, ctx, common),
            Self::Text(text) => text.render(ui, ctx, common),
            Self::Bar(bar) => bar.render(ui, ctx, common),
        }
    }

    pub fn render_options(&mut self, ui: &Ui, ctx: &RenderCtx) {
        match self {
            Self::Group(group) => group.render_options(ui, ctx),
            Self::Icon(icon) => icon.render_options(ui, ctx),
            Self::IconList(list) => list.render_options(ui, ctx),
            Self::Text(text) => text.render_options(ui, ctx),
            Self::Bar(bar) => bar.render_options(ui, ctx),
        }
    }

    pub fn render_tabs(&mut self, ui: &Ui, ctx: &RenderCtx) {
        match self {
            Self::Group(group) => group.render_tabs(ui, ctx),
            Self::Icon(icon) => icon.render_tabs(ui, ctx),
            Self::IconList(list) => list.render_tabs(ui, ctx),
            Self::Text(text) => text.render_tabs(ui, ctx),
            Self::Bar(bar) => bar.render_tabs(ui, ctx),
        }
    }

    pub fn render_filters(&mut self, ui: &Ui, ctx: &Context) {
        match self {
            Self::IconList(list) => list.render_filters(ui, ctx),
            Self::Group(_) | Self::Icon(_) | Self::Text(_) | Self::Bar(_) => {}
        }
    }

    pub fn render_debug(&mut self, ui: &Ui, ctx: &RenderCtx) {
        match self {
            Self::Group(group) => group.render_debug(ui, ctx),
            Self::Icon(icon) => icon.render_debug(ui, ctx),
            Self::IconList(list) => list.render_debug(ui, ctx),
            Self::Text(text) => text.render_debug(ui, ctx),
            Self::Bar(bar) => bar.render_debug(ui, ctx),
        }
    }
}

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

impl Default for ElementType {
    fn default() -> Self {
        Self::Icon(IconElement::default())
    }
}
