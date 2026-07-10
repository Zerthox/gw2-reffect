mod font_load;
mod load;
mod resize;
mod update;
mod visit;

pub use self::{font_load::*, load::*, resize::*, update::*, visit::*};

use crate::elements::Element;

/// [`Element`] tree node.
pub trait TreeNode {
    /// Returns the child [`Elements`].
    fn children(&mut self) -> Option<&mut Vec<Element>> {
        None
    }
}
