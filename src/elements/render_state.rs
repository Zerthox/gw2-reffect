use crate::component_wise::ComponentWise;

/// Current render state.
///
/// This is used as immutable structure to avoid accidentally keeping child state changes.
#[derive(Debug, Clone)]
pub struct RenderState {
    pub edit: bool,
    pub pos: [f32; 2],
}

impl RenderState {
    pub const fn new(pos: [f32; 2]) -> Self {
        Self { edit: false, pos }
    }

    pub fn with_offset(&self, offset: [f32; 2]) -> Self {
        Self {
            edit: self.edit,
            pos: self.pos.add(offset),
        }
    }

    pub fn with_edit(&self, edit: bool) -> Self {
        Self {
            edit,
            pos: self.pos,
        }
    }
}
