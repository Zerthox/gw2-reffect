use super::Common;
use crate::{
    context::Context, elements::ElementAnchor, render::ComponentWise, settings::GeneralSettings,
};
use nexus::imgui::Ui;
use std::{cell::Cell, ops::Deref};

// TODO: add tint + opacity as color, scale, use instead of imgui globals?
// TODO: add screen anchor, pos only relative, apply scale before final render?

/// Render context.
///
/// Associated functions create new instances to avoid accidentally keeping child state changes.
#[derive(Debug)]
pub struct RenderCtx<'a> {
    /// Main context.
    context: &'a Context,

    /// Settings.
    pub settings: &'a GeneralSettings,

    /// Whether the current element is edited.
    edit: Cell<bool>,

    /// Current screen cursor position.
    pos: Cell<[f32; 2]>,
}

impl<'a> RenderCtx<'a> {
    /// Creates a new initial render context.
    pub fn create(ui: &Ui, game: &'a Context, settings: &'a GeneralSettings) -> Self {
        Self {
            context: game,
            settings,
            edit: Cell::new(false),
            pos: Cell::new(ElementAnchor::root(ui)),
        }
    }

    /// Pushes a new state to the render context.
    ///
    /// Dropping the returned token reverts to the previous state.
    pub fn push(&self) -> Token {
        Token::capture(self)
    }

    /// Pushes a new child to the render context.
    ///
    /// Dropping the returned token reverts to the previous state.
    pub fn push_child(&self, ui: &Ui, common: &Common) -> Token {
        let token = self.push();
        let edited = if self.context.edit.settings.show_all {
            self.context.edit.is_edited_or_parent(common.id)
        } else {
            self.context.edit.is_edited(common.id)
        };
        self.edit.set(self.is_edited() || edited);
        self.pos.set(common.pos(ui, self.pos()));
        token
    }

    /// Pushes a new offset to the new render context
    ///
    /// Dropping the returned token reverts to the previous state.
    pub fn push_offset(&self, offset: [f32; 2]) -> Token {
        let token = self.push();
        self.add_offset(offset);
        token
    }

    /// Adds an offset to the current position.
    pub fn add_offset(&self, offset: [f32; 2]) {
        self.pos.set(self.pos().add(offset))
    }

    /// Checks whether the current element is visible in edit mode.
    pub fn is_edited(&self) -> bool {
        self.edit.get()
    }

    /// Returns the current cursor position.
    pub fn pos(&self) -> [f32; 2] {
        self.pos.get()
    }
}

impl Deref for RenderCtx<'_> {
    type Target = Context;

    fn deref(&self) -> &Self::Target {
        self.context
    }
}

#[derive(Debug)]
pub struct Token<'r, 'a> {
    ctx: &'r RenderCtx<'a>,
    edited: bool,
    pos: [f32; 2],
}

impl<'a, 'b> Token<'a, 'b> {
    fn capture(ctx: &'a RenderCtx<'b>) -> Self {
        Self {
            ctx,
            edited: ctx.is_edited(),
            pos: ctx.pos(),
        }
    }
}

impl Drop for Token<'_, '_> {
    fn drop(&mut self) {
        self.ctx.edit.set(self.edited);
        self.ctx.pos.set(self.pos);
    }
}
