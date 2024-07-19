use super::{Align, Icon};
use crate::render_util::Rect;
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

    pub fn progress_rect_offset(&self, size: [f32; 2], progress: f32) -> ([f32; 2], [f32; 2]) {
        let [width, height] = size;
        match self {
            Self::Right => ([0.0, 0.0], [progress * width, height]),
            Self::Left => ([(1.0 - progress) * width, 0.0], size),
            Self::Up => ([0.0, (1.0 - progress) * height], size),
            Self::Down => ([0.0, 0.0], [width, progress * height]),
            Self::Horizontal => (
                [(0.5 - 0.5 * progress) * width, 0.0],
                [(0.5 + 0.5 * progress) * width, height],
            ),
            Self::Vertical => (
                [0.0, (0.5 - 0.5 * progress) * height],
                [width, (0.5 + 0.5 * progress) * height],
            ),
        }
    }

    pub fn progress_value_offset(&self, size: [f32; 2], progress: f32) -> [f32; 2] {
        let [width, height] = size;
        match self {
            Self::Right => [progress * width, 0.0],
            Self::Left => [(1.0 - progress) * width, 0.0],
            Self::Up => [0.0, (1.0 - progress) * height],
            Self::Down => [0.0, progress * height],
            Self::Horizontal => [(0.5 + 0.5 * progress) * width, 0.0],
            Self::Vertical => [0.0, (0.5 + 0.5 * progress) * height],
        }
    }

    pub fn tick_end_offset(&self, size: [f32; 2]) -> [f32; 2] {
        let [width, height] = size;
        match self {
            Self::Right | Self::Left | Self::Horizontal => [0.0, height],
            Self::Up | Self::Down | Self::Vertical => [width, 0.0],
        }
    }

    pub fn list_start_offset(&self, size: [f32; 2], pad: f32, total: usize) -> [f32; 2] {
        let [width, height] = size;
        let last = total.saturating_sub(1) as f32;
        match self {
            Self::Right | Self::Left | Self::Up | Self::Down => [0.0, 0.0],
            Self::Horizontal => [-0.5 * last * (width + pad), 0.0],
            Self::Vertical => [0.0, -0.5 * last * (height + pad)],
        }
    }

    pub fn list_item_offset(
        &self,
        size: [f32; 2],
        pad: f32,
        element: usize,
        total: usize,
    ) -> [f32; 2] {
        let [width, height] = size;
        let i = element as f32;
        let offset_x = i * (width + pad);
        let offset_y = i * (height + pad);
        let last = total.saturating_sub(1) as f32;
        match self {
            Self::Right => [offset_x, 0.0],
            Self::Left => [-offset_x, 0.0],
            Self::Up => [0.0, -offset_y],
            Self::Down => [0.0, offset_y],
            Self::Horizontal => [offset_x - 0.5 * last * (width + pad), 0.0],
            Self::Vertical => [0.0, offset_y - 0.5 * last * (height + pad)],
        }
    }

    pub fn icon_list_bounds(&self, size: [f32; 2], pad: f32, total: usize) -> Rect {
        let [width, height] = size;
        let last = total.saturating_sub(1) as f32;
        let offset_x = last * (width + pad);
        let offset_y = last * (height + pad);
        let (min, max) = match self {
            Self::Right => ([0.0, 0.0], [offset_x, 0.0]),
            Self::Left => ([-offset_x, 0.0], [0.0, 0.0]),
            Self::Up => ([0.0, -offset_y], [0.0, 0.0]),
            Self::Down => ([0.0, 0.0], [0.0, offset_y]),
            Self::Horizontal => {
                let start = -0.5 * last * (width + pad);
                ([start, 0.0], [start + offset_x, 0.0])
            }
            Self::Vertical => {
                let start = -0.5 * last * (height + pad);
                ([0.0, start], [0.0, start + offset_y])
            }
        };
        let (min, _) = Icon::bounds(min, size);
        let (_, max) = Icon::bounds(max, size);
        (min, max)
    }
}
