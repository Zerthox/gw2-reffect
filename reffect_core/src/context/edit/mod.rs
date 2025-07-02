mod settings;

use crate::{context::ui::UiInfo, id::Id};
use nexus::imgui::Ui;

pub use self::settings::*;

#[derive(Debug)]
pub struct EditState {
    /// Edit settings.
    pub settings: EditSettings,

    /// Whether edit mode is currently allowed.
    allowed: bool,

    /// Selected element id.
    selected: Id,

    /// Selected element parents.
    // TODO: keep parents sorted?
    parents: Vec<Id>,
}

impl EditState {
    #[inline]
    pub const fn new() -> Self {
        Self {
            settings: EditSettings::new(),
            allowed: true,
            selected: Id::NIL,
            parents: Vec::new(),
        }
    }

    #[inline]
    pub fn reset(&mut self) {
        *self = Self::new();
    }

    /// Whether edit mode is currently allowed.
    #[inline]
    pub fn is_allowed(&self) -> bool {
        self.allowed
    }

    /// Whether the id is selected.
    #[inline]
    pub fn is_selected(&self, id: Id) -> bool {
        self.selected == id
    }

    /// Whether the id itself or a child of it is selected.
    #[inline]
    pub fn is_selected_or_parent(&self, id: Id) -> bool {
        self.is_selected(id) || self.is_selected_parent(id)
    }

    /// Whether a child is selected.
    #[inline]
    pub fn is_selected_parent(&self, id: Id) -> bool {
        self.parents.contains(&id)
    }

    /// Whether the id is currently edited.
    #[inline]
    pub fn is_edited(&self, id: Id) -> bool {
        self.is_allowed() && self.is_selected(id)
    }

    /// Whether a child is currently edited.
    #[inline]
    pub fn is_edited_parent(&self, id: Id) -> bool {
        self.is_allowed() && self.is_selected_parent(id)
    }

    /// Whether the id itself or a child of it is currently edited.
    #[inline]
    pub fn is_edited_or_parent(&self, id: Id) -> bool {
        self.is_allowed() && self.is_selected_or_parent(id)
    }

    /// Whether any element is edited.
    #[inline]
    pub fn is_editing(&self) -> bool {
        self.is_allowed() && self.selected != Id::default()
    }

    pub fn select(&mut self, id: Id) -> bool {
        self.parents.clear();
        if id == self.selected {
            self.selected = Id::default();
            false
        } else {
            self.selected = id;
            true
        }
    }

    #[inline]
    pub fn push_parent(&mut self, id: Id) {
        if id != self.selected {
            self.parents.push(id);
        }
    }

    #[inline]
    pub fn update_allowed(&mut self, ui: &UiInfo) {
        self.allowed = self.settings.during_combat || !ui.combat;
    }

    #[inline]
    pub fn reset_allowed(&mut self) {
        self.allowed = false;
    }

    pub fn debug(&self, ui: &Ui) {
        ui.text("Edit allowed:");
        ui.same_line();
        ui.text(self.is_allowed().to_string());

        ui.text("Selected element:");
        ui.same_line();
        ui.text(self.selected.to_string());

        ui.text("Selected parents:");
        for id in &self.parents {
            ui.same_line();
            ui.text(id.to_string());
        }
    }
}

impl Default for EditState {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
