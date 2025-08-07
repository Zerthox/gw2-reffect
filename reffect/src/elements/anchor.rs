use crate::{
    enums::check_variant_array,
    render::{ComponentWise, enum_combo},
};
use nexus::imgui::{ComboBoxFlags, Ui};
use serde::{Deserialize, Serialize};
use std::{iter::Copied, slice};
use strum::{AsRefStr, EnumCount, EnumIter, IntoEnumIterator, VariantArray};

/// Element anchor point.
#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    EnumCount,
    AsRefStr,
    Serialize,
    Deserialize,
)]
pub enum ElementAnchor {
    #[default]
    Parent,

    Screen(Anchor),
}

impl VariantArray for ElementAnchor {
    const VARIANTS: &'static [Self] = &[Self::Parent, Self::Screen(Anchor::Center)];
}

impl IntoEnumIterator for ElementAnchor {
    type Iterator = Copied<slice::Iter<'static, Self>>;

    fn iter() -> Self::Iterator {
        Self::VARIANTS.iter().copied()
    }
}

const _: () = check_variant_array::<ElementAnchor>();

impl ElementAnchor {
    /// Calculates the root position.
    pub fn root(ui: &Ui) -> [f32; 2] {
        Anchor::Center.screen_pos(ui)
    }

    /// Calculates the anchor position.
    pub fn pos(&self, ui: &Ui, parent: [f32; 2]) -> [f32; 2] {
        match self {
            Self::Parent => parent,
            Self::Screen(screen) => screen.screen_pos(ui),
        }
    }

    pub fn render_select(&mut self, ui: &Ui) {
        enum_combo(ui, "Anchor", self, ComboBoxFlags::empty());

        match self {
            Self::Parent => {}
            Self::Screen(screen) => {
                enum_combo(ui, "Screen Anchor", screen, ComboBoxFlags::empty());
            }
        }
    }
}

/// Fixed anchor point.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    EnumIter,
    VariantArray,
    AsRefStr,
    Serialize,
    Deserialize,
)]
pub enum Anchor {
    #[strum(serialize = "Top Left")]
    TopLeft,

    #[strum(serialize = "Top Center")]
    TopCenter,

    #[strum(serialize = "Top Right")]
    TopRight,

    #[strum(serialize = "Left Center")]
    LeftCenter,

    Center,

    #[strum(serialize = "Right Center")]
    RightCenter,

    #[strum(serialize = "Bottom Left")]
    BottomLeft,

    #[strum(serialize = "Bottom Center")]
    BottomCenter,

    #[strum(serialize = "Bottom Right")]
    BottomRight,
}

impl Anchor {
    /// Calculates the anchor position.
    pub fn pos(&self, size: [f32; 2]) -> [f32; 2] {
        let [width, height] = size;
        match self {
            Self::TopLeft => [0.0, 0.0],
            Self::TopRight => [width, 0.0],
            Self::BottomLeft => [0.0, height],
            Self::BottomRight => [width, height],
            Self::Center => [0.5 * width, 0.5 * height],
            Self::TopCenter => [0.5 * width, 0.0],
            Self::BottomCenter => [0.5 * width, height],
            Self::LeftCenter => [0.0, 0.5 * height],
            Self::RightCenter => [width, 0.5 * height],
        }
    }

    /// Calculates the screen position.
    pub fn screen_pos(&self, ui: &Ui) -> [f32; 2] {
        let size = ui.io().display_size;
        self.pos(size)
    }

    /// Calculates the item alignment.
    pub fn align(&self, size: [f32; 2]) -> [f32; 2] {
        // align is inverse of pos
        self.pos(size).neg()
    }
}

impl From<Anchor> for ElementAnchor {
    fn from(screen: Anchor) -> Self {
        Self::Screen(screen)
    }
}
