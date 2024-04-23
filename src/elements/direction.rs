use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumIter, VariantArray};

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
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
    Horizontal,
    Vertical,
}

impl Direction {
    pub fn offset_for(&self, size: [f32; 2], pad: f32, element: usize, total: usize) -> [f32; 2] {
        let [width, height] = size;
        let i = element as f32;
        let offset_x = i * (width + pad);
        let offset_y = i * (height + pad);
        let additional = total.saturating_sub(1) as f32;
        let half = 0.5 * additional;
        let half_pad = 0.5 * additional * pad;
        match self {
            Self::Right => [offset_x, 0.0],
            Self::Left => [-offset_x, 0.0],
            Self::Up => [0.0, -offset_y],
            Self::Down => [0.0, offset_x],
            Self::Horizontal => [offset_x - half * width - half_pad, 0.0],
            Self::Vertical => [0.0, offset_y - half * height - half_pad],
        }
    }
}
