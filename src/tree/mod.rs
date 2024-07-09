mod load;
mod visit;

use crate::elements::Element;

pub use self::{load::*, visit::*};

/// [`Element`] tree node.
pub trait TreeNode {
    /// Returns the child [`Elements`].
    fn children(&mut self) -> Option<&mut Vec<Element>>;
}

/// [`Element`] tree node that is a leaf.
pub trait TreeLeaf {}

impl<T> TreeNode for T
where
    T: TreeLeaf,
{
    fn children(&mut self) -> Option<&mut Vec<Element>> {
        None
    }
}
