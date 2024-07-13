use super::Common;
use crate::{component_wise::ComponentWise, context::Context};

// TODO: as visitor?
// TODO: add tint + opacity as color, scale, use instead of imgui globals?

/// Current render state.
///
/// Associated functions create new instances to avoid accidentally keeping child state changes.
#[derive(Debug)]
pub struct RenderState<'a> {
    /// Whether a parent is selected.
    edit: bool,

    /// Current screen cursor position.
    pub pos: [f32; 2],

    /// Common for element.
    pub common: &'a Common,
}

impl<'a> RenderState<'a> {
    pub fn new(edit: bool, pos: [f32; 2], common: &'a Common) -> Self {
        Self { edit, pos, common }
    }

    pub fn for_element<'b>(&'a self, common: &'b Common, ctx: &Context) -> RenderState<'b> {
        RenderState {
            edit: self.edit || ctx.edit.is_edited(common.id),
            pos: common.pos(self),
            common,
        }
    }

    pub fn with_offset(&'a self, offset: [f32; 2]) -> Self {
        // TODO: positive offset always towards center of screen? need direction/anchor here probably
        Self {
            edit: self.edit,
            common: self.common,
            pos: self.pos.add(offset),
        }
    }

    pub fn is_edit(&self, ctx: &Context) -> bool {
        self.edit || ctx.edit.is_edited_parent(self.common.id)
    }
}
