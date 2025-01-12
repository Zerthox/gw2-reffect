use super::Common;
use crate::{context::Context, render::ComponentWise, trigger::ProgressActive};
use nexus::imgui::Ui;

// TODO: as visitor?
// TODO: add tint + opacity as color, scale, use instead of imgui globals?
// TODO: add screen anchor, pos only relative, apply scale before final render?

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
    /// Creates a new initial render state.
    pub fn initial(edit: bool, pos: [f32; 2], common: &'a Common) -> Self {
        Self { edit, pos, common }
    }

    /// Creates a new render state for a child.
    pub fn for_child<'b>(&'a self, ui: &Ui, ctx: &Context, common: &'b Common) -> RenderState<'b>
    where
        'a: 'b,
    {
        RenderState {
            edit: self.edit || ctx.edit.is_edited(common.id),
            pos: common.pos(ui, self.pos),
            common,
        }
    }

    /// Creates a new the render state with a poisition offset.
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

    pub fn trigger_active(&self) -> Option<&ProgressActive> {
        self.common.trigger.active()
    }
}
