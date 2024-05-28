use super::UiContext;
use crate::{elements::Element, id::Id};
use nexus::imgui::Ui;

// TODO: store parent chain and only display those during edit?

#[derive(Debug, Clone)]
pub struct EditState {
    /// Whether edit mode is allowed in combat.
    pub during_combat: bool,

    /// Whether edit mode is currently allowed.
    allowed: bool,

    /// Selected element id.
    active: Id,

    /// Current clipboard contents.
    clipboard: Option<Element>,
}

impl EditState {
    /// Whether the id is selected for editing.
    pub fn is_selected(&self, id: Id) -> bool {
        self.active == id
    }

    /// Whether edit mode is currently allowed.
    pub fn is_allowed(&self) -> bool {
        self.allowed
    }

    /// Whether the id currently edited.
    pub fn is_edited(&self, id: Id) -> bool {
        self.is_allowed() && self.is_selected(id)
    }

    pub fn is_editing(&self) -> bool {
        self.is_allowed() && self.active != Id::default()
    }

    pub fn select(&mut self, id: Id) {
        if self.active == id {
            self.active = Id::default();
        } else {
            self.active = id;
        }
    }

    pub fn update_allowed(&mut self, ui: &UiContext) {
        self.allowed = self.during_combat || !ui.combat;
    }

    pub fn has_clipboard(&mut self) -> bool {
        self.clipboard.is_some()
    }

    pub fn take_clipboard(&mut self) -> Option<Element> {
        self.clipboard.take()
    }

    pub fn set_clipboard(&mut self, element: Element) {
        self.clipboard = Some(element);
    }

    pub fn debug(&self, ui: &Ui) {
        ui.text("Clipboard:");
        ui.same_line();
        match &self.clipboard {
            Some(element) => ui.text(&element.kind),
            None => ui.text_disabled("empty"),
        }
        ui.text("Selected Element:");
        ui.same_line();
        ui.text(self.active.to_string());
    }
}

impl Default for EditState {
    fn default() -> Self {
        Self {
            during_combat: false,
            allowed: true,
            clipboard: None,
            active: Id::default(),
        }
    }
}
