use crate::render_util::{enum_combo, impl_static_variants};
use nexus::imgui::{ComboBoxFlags, Ui};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumIter, VariantArray};

/// Anchor point.
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
    EnumIter,
    AsRefStr,
    Serialize,
    Deserialize,
)]
pub enum Anchor {
    #[default]
    Parent,

    Screen(ScreenAnchor),
}

impl_static_variants!(Anchor);

impl Anchor {
    /// Calculates the root position.
    pub fn root(ui: &Ui) -> [f32; 2] {
        ScreenAnchor::Center.calc_pos(ui)
    }

    /// Calculates the anchor position.
    pub fn pos(&self, ui: &Ui, parent: [f32; 2]) -> [f32; 2] {
        match self {
            Self::Parent => parent,
            Self::Screen(screen) => screen.calc_pos(ui),
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

/// Screen anchor point.
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
    EnumIter,
    VariantArray,
    AsRefStr,
    Serialize,
    Deserialize,
)]
pub enum ScreenAnchor {
    #[strum(serialize = "Top Left")]
    TopLeft,

    #[strum(serialize = "Top Center")]
    TopCenter,

    #[strum(serialize = "Top Right")]
    TopRight,

    #[strum(serialize = "Left Center")]
    LeftCenter,

    #[default]
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

impl ScreenAnchor {
    /// Calculates the screen position.
    pub fn calc_pos(&self, ui: &Ui) -> [f32; 2] {
        let [screen_x, screen_y] = ui.io().display_size;
        match self {
            Self::TopLeft => [0.0, 0.0],
            Self::TopRight => [screen_x, 0.0],
            Self::BottomLeft => [0.0, screen_y],
            Self::BottomRight => [screen_x, screen_y],
            Self::Center => [0.5 * screen_x, 0.5 * screen_y],
            Self::TopCenter => [0.5 * screen_x, 0.0],
            Self::BottomCenter => [0.5 * screen_x, screen_y],
            Self::LeftCenter => [0.0, 0.5 * screen_y],
            Self::RightCenter => [screen_x, 0.5 * screen_y],
        }
    }
}

impl From<ScreenAnchor> for Anchor {
    fn from(screen: ScreenAnchor) -> Self {
        Self::Screen(screen)
    }
}
