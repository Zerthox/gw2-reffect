mod font_reload;
mod load;
mod visit;

pub use self::{font_reload::*, load::*, visit::*};

use crate::elements::Element;

/// [`Element`] tree node.
pub trait TreeNode {
    /// Returns the child [`Elements`].
    fn children(&mut self) -> Option<&mut Vec<Element>> {
        None
    }
}
