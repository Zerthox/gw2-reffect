mod filter_update;
mod font_reload;
mod load;
mod resize;
mod visit;

pub use self::{filter_update::*, font_reload::*, load::*, resize::*, visit::*};

use crate::elements::Element;

/// [`Element`] tree node.
pub trait TreeNode {
    /// Returns the child [`Elements`].
    fn children(&mut self) -> Option<&mut Vec<Element>> {
        None
    }
}
