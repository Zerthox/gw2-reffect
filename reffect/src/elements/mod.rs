pub mod align;
pub mod bar;
pub mod icon;
pub mod list;
pub mod text;

mod anchor;
mod animation;
mod common;
mod direction;
mod dnd;
mod element;
mod group;
mod pack;
mod props;
mod render_ctx;
mod unit;

pub use self::{
    anchor::*,
    animation::*,
    bar::Bar,
    common::*,
    direction::*,
    dnd::*,
    element::*,
    group::*,
    icon::{Icon, IconElement},
    list::IconList,
    pack::*,
    props::*,
    render_ctx::*,
    text::Text,
    unit::*,
};

use crate::id::IdGen;

pub static ELEMENT_ID: IdGen = IdGen::new();
