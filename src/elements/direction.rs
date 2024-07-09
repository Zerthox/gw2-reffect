use super::Align;
use crate::component_wise::ComponentWise;
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
    pub fn align(&self) -> Align {
        match self {
            Self::Right => Align::Right,
            Self::Left => Align::Left,
            Self::Up => Align::Up,
            Self::Down => Align::Down,
            Self::Horizontal | Self::Vertical => Align::Center,
        }
    }

    pub fn progress_pos(
        &self,
        start: [f32; 2],
        size: [f32; 2],
        progress: f32,
    ) -> ([f32; 2], [f32; 2]) {
        let [width, height] = size;
        match self {
            Self::Right => (start, start.add([progress * width, height])),
            Self::Left => (start.add([(1.0 - progress) * width, 0.0]), start.add(size)),
            Self::Up => (start.add([0.0, (1.0 - progress) * height]), start.add(size)),
            Self::Down => (start, start.add([width, progress * height])),
            Self::Horizontal => (
                start.add([(0.5 - 0.5 * progress) * width, 0.0]),
                start.add([(0.5 + 0.5 * progress) * width, height]),
            ),
            Self::Vertical => (
                start.add([0.0, (0.5 - 0.5 * progress) * height]),
                start.add([width, (0.5 + 0.5 * progress) * height]),
            ),
        }
    }

    pub fn list_item_offset(
        &self,
        size: [f32; 2],
        pad: f32,
        element: usize,
        total: usize,
    ) -> [f32; 2] {
        // TODO: adjust center point of first element?
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
            Self::Down => [0.0, offset_y],
            Self::Horizontal => [offset_x - half * width - half_pad, 0.0],
            Self::Vertical => [0.0, offset_y - half * height - half_pad],
        }
    }
}
