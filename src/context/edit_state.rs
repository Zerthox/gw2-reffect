use crate::elements::Element;
use nexus::imgui::Ui;
use uuid::Uuid;

// TODO: store parent chain and only display those during edit?

#[derive(Debug, Default, Clone)]
pub struct EditState {
    clipboard: Option<Element>,
    active: Uuid,
}

impl EditState {
    pub fn is_active(&self, id: Uuid) -> bool {
        self.active == id
    }

    pub fn select(&mut self, id: Uuid) {
        if self.active == id {
            self.active = Uuid::nil();
        } else {
            self.active = id;
        }
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
        match &self.clipboard {
            Some(element) => ui.text(format!("Clipboard: {}", element.kind.as_ref())),
            None => ui.text("Clipboard: empty"),
        }
        ui.text(format!("Selected Element: {}", self.active.simple()));
    }
}
