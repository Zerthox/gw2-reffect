use crate::{
    elements::Element,
    id::{Id, IdGen},
};
use nexus::imgui::Ui;

// TODO: store parent chain and only display those during edit?

#[derive(Debug, Default, Clone)]
pub struct EditState {
    clipboard: Option<Element>,
    active: Id,
}

impl EditState {
    pub fn is_active(&self, id: Id) -> bool {
        self.active == id
    }

    pub fn select(&mut self, id: Id) {
        if self.active == id {
            self.active = IdGen::nil();
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
        ui.text("Clipboard:");
        ui.same_line();
        match &self.clipboard {
            Some(element) => ui.text(&element.kind),
            None => ui.text_disabled("empty"),
        }
        ui.text("Selected Element:");
        ui.same_line();
        ui.text(IdGen::display(self.active).to_string());
    }
}
