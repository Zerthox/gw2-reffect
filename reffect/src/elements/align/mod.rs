mod horizontal;

pub use self::horizontal::*;

use crate::{math::ComponentWise, render::Rect};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumIter, VariantArray};

/// 2 dimensional alignment.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    AsRefStr,
    EnumIter,
    VariantArray,
    Serialize,
    Deserialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum Align {
    Right,
    Left,
    Up,
    Down,
    Center,
}

impl Align {
    #[inline]
    pub const fn offset(&self, size: [f32; 2]) -> [f32; 2] {
        let [width, height] = size;
        match self {
            Self::Right => [0.0, -0.5 * height],
            Self::Left => [-width, -0.5 * height],
            Self::Up => [-0.5 * width, -height],
            Self::Down => [-0.5 * width, 0.0],
            Self::Center => [-0.5 * width, -0.5 * height],
        }
    }

    #[inline]
    pub fn bounds(&self, size: [f32; 2]) -> Rect {
        let start = self.offset(size);
        (start, start.add(size))
    }
}
