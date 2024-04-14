use crate::component_wise::ComponentWise;

/// Current render state.
///
/// This is used as immutable structure to avoid accidentally keeping child state changes.
#[derive(Debug, Clone)]
pub struct RenderState<'a> {
    pub edit: bool,
    pub pos: [f32; 2],
    pub name: &'a str,
}

impl RenderState<'_> {
    pub const fn new(edit: bool, pos: [f32; 2]) -> Self {
        Self {
            edit,
            pos,
            name: "Unnamed",
        }
    }

    pub fn with_offset(&self, offset: [f32; 2]) -> Self {
        Self {
            edit: self.edit,
            pos: self.pos.add(offset),
            name: self.name,
        }
    }

    // TODO: use for displaying only parents during edit?
    #[allow(unused)]
    pub fn and_edit(&self, edit: bool) -> Self {
        Self {
            edit: self.edit || edit,
            pos: self.pos,
            name: self.name,
        }
    }
}
