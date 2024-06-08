use crate::{component_wise::ComponentWise, context::Context};

use super::Common;

// TODO: as visitor?
// TODO: avoid duplicating name?

/// Current render state.
///
/// This is used as immutable structure to avoid accidentally keeping child state changes.
#[derive(Debug, Clone)]
pub struct RenderState<'a> {
    /// Whether a parent is selected.
    edit: bool,

    /// Current screen cursor position.
    pub pos: [f32; 2],

    /// Common for element.
    pub common: &'a Common,
}

impl<'a> RenderState<'a> {
    pub const fn new(pos: [f32; 2], common: &'a Common) -> Self {
        Self {
            edit: false,
            pos,
            common,
        }
    }

    pub fn for_element(self, common: &'a Common, ctx: &Context) -> Self {
        Self {
            edit: self.edit || ctx.edit.is_selected(common.id), // TODO: setting to show entire pack?
            pos: self.pos,
            common,
        }
    }

    pub fn with_offset(&self, offset: [f32; 2]) -> Self {
        Self {
            edit: self.edit,
            common: self.common,
            pos: self.pos.add(offset),
        }
    }

    pub fn is_edit(&self, ctx: &Context) -> bool {
        self.edit || ctx.edit.is_parent(self.common.id)
    }
}
