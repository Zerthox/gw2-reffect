use super::list::ListIcon;
use crate::{math::ComponentWise, render::Rect};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumIter, VariantArray};

/// Direction.
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
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
    Horizontal,
    Vertical,
}

impl Direction {
    /// Whether the direction is bi-directional.
    pub const fn is_bidirectional(&self) -> bool {
        matches!(self, Self::Horizontal | Self::Vertical)
    }

    /// Returns the offset in the orthogonal 2-dimensional direction.
    pub const fn offset_2d(&self, size: [f32; 2]) -> [f32; 2] {
        let [width, height] = size;
        match self {
            Self::Right | Self::Left | Self::Horizontal => [0.0, height],
            Self::Up | Self::Down | Self::Vertical => [width, 0.0],
        }
    }

    /// Calculates the offset of the given progress value.
    pub const fn progress_value_offset(&self, size: [f32; 2], progress: f32) -> [f32; 2] {
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

    /// Calculates the alternative offset of the given progress value.
    pub const fn progress_value_offset_alt(
        &self,
        size: [f32; 2],
        progress: f32,
    ) -> Option<[f32; 2]> {
        let [width, height] = size;
        match self {
            Self::Right | Self::Left | Self::Up | Self::Down => None,
            Self::Horizontal => Some([(0.5 - 0.5 * progress) * width, 0.0]),
            Self::Vertical => Some([0.0, (0.5 - 0.5 * progress) * height]),
        }
    }

    /// Calculates the rect dimensions of the given progress.
    pub const fn progress_rect_offset(&self, size: [f32; 2], progress: f32) -> Rect {
        let [width, height] = size;
        match self {
            Self::Right => ([0.0, 0.0], [progress * width, 0.0]),
            Self::Left => ([(1.0 - progress) * width, 0.0], [width, 0.0]),
            Self::Up => ([0.0, (1.0 - progress) * height], [0.0, height]),
            Self::Down => ([0.0, 0.0], [0.0, progress * height]),
            Self::Horizontal => (
                [(0.5 - 0.5 * progress) * width, 0.0],
                [(0.5 + 0.5 * progress) * width, 0.0],
            ),
            Self::Vertical => (
                [0.0, (0.5 - 0.5 * progress) * height],
                [0.0, (0.5 + 0.5 * progress) * height],
            ),
        }
    }

    /// Calculates the start offset of a list with the given parameters.
    pub const fn list_start_offset(&self, size: [f32; 2], pad: f32, total: usize) -> [f32; 2] {
        let [width, height] = size;
        let last = total.saturating_sub(1) as f32;
        match self {
            Self::Right | Self::Left | Self::Up | Self::Down => [0.0, 0.0],
            Self::Horizontal => [-0.5 * last * (width + pad), 0.0],
            Self::Vertical => [0.0, -0.5 * last * (height + pad)],
        }
    }

    /// Calculates the offset of an item in the list with the given parameters.
    pub const fn list_item_offset(
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

    /// Calcualtes the bounds for a list icon.
    pub fn icon_list_bounds(&self, size: [f32; 2], pad: f32, total: usize) -> Rect {
        let [width, height] = size;
        let last = total.saturating_sub(1) as f32;
        let offset_x = last * (width + pad);
        let offset_y = last * (height + pad);
        let (first, last) = match self {
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
        let (bounds_min, bounds_max) = ListIcon::ALIGN.bounds(size);
        (first.add(bounds_min), last.add(bounds_max))
    }
}
