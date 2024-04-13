use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumIter};

use crate::util::impl_static_variants;

/// Screen anchor point.
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
    AsRefStr,
    Serialize,
    Deserialize,
)]
pub enum Anchor {
    #[strum(serialize = "Top left")]
    TopLeft,

    #[strum(serialize = "Top center")]
    TopCenter,

    #[strum(serialize = "Top right")]
    TopRight,

    #[strum(serialize = "Bottom left")]
    BottomLeft,

    #[strum(serialize = "Bottom center")]
    BottomCenter,

    #[strum(serialize = "Bottom right")]
    BottomRight,

    Center,

    #[strum(serialize = "Left Center")]
    LeftCenter,

    #[strum(serialize = "Right Center")]
    RightCenter,
}

impl_static_variants!(Anchor);

impl Anchor {
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
