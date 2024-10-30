use super::UiContext;
use crate::{
    elements::{Element, ElementType},
    id::Id,
};
use nexus::imgui::Ui;

#[derive(Debug, Clone)]
pub struct EditState {
    /// Whether edit mode is allowed in combat.
    pub during_combat: bool,

    /// Whether to show all elements of a pack in edit mode.
    pub show_all: bool,

    /// Whether edit mode is currently allowed.
    allowed: bool,

    /// Selected element id.
    selected: Id,

    /// Selected element parents.
    // TODO: keep parents sorted?
    parents: Vec<Id>,

    /// Current clipboard contents.
    clipboard: Option<Element>,
}

impl EditState {
    /// Whether edit mode is currently allowed.
    pub fn is_allowed(&self) -> bool {
        self.allowed
    }

    /// Whether the id is selected.
    pub fn is_selected(&self, id: Id) -> bool {
        self.selected == id
    }

    /// Whether the id itself or a child of it is selected.
    pub fn is_selected_or_parent(&self, id: Id) -> bool {
        self.is_selected(id) || self.is_selected_parent(id)
    }

    /// Whether a child is selected.
    pub fn is_selected_parent(&self, id: Id) -> bool {
        self.parents.contains(&id)
    }

    /// Whether the id is currently edited.
    pub fn is_edited(&self, id: Id) -> bool {
        self.is_allowed() && self.is_selected(id)
    }

    /// Whether a child is currently edited.
    pub fn is_edited_parent(&self, id: Id) -> bool {
        self.is_allowed() && self.is_selected_parent(id)
    }

    /// Whether the id itself or a child of it is currently edited.
    pub fn is_edited_or_parent(&self, id: Id) -> bool {
        self.is_allowed() && self.is_selected_or_parent(id)
    }

    /// Whether any element is edited.
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

    pub fn push_parent(&mut self, id: Id) {
        if id != self.selected {
            self.parents.push(id);
        }
    }

    pub fn update_allowed(&mut self, ui: &UiContext) {
        self.allowed = self.during_combat || !ui.combat;
    }

    pub fn reset_allowed(&mut self) {
        self.allowed = false;
    }

    pub fn has_clipboard(&mut self) -> bool {
        self.clipboard.is_some()
    }

    pub fn has_icon_clipboard(&mut self) -> bool {
        matches!(
            self.clipboard,
            Some(Element {
                kind: ElementType::Icon(_),
                ..
            }),
        )
    }

    pub fn take_clipboard(&mut self) -> Option<Element> {
        self.clipboard.take()
    }

    pub fn set_clipboard(&mut self, element: Element) {
        self.clipboard = Some(element);
    }

    pub fn debug(&self, ui: &Ui) {
        ui.text("Edit allowed:");
        ui.same_line();
        ui.text(self.is_allowed().to_string());

        ui.text("Clipboard:");
        ui.same_line();
        match &self.clipboard {
            Some(element) => ui.text(&element.kind),
            None => ui.text_disabled("empty"),
        }

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
    fn default() -> Self {
        Self {
            during_combat: false,
            show_all: false,
            allowed: true,
            selected: Id::default(),
            parents: Vec::new(),
            clipboard: None,
        }
    }
}
