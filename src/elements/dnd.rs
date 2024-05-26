use super::Element;
use crate::action::ElementAction;
use nexus::imgui::{DragDropFlags, DragDropSource, DragDropTarget, Ui};

// imgui is single threaded
static mut DRAGGED: Option<Element> = None;

// TODO: drag between elements

pub struct Dnd;

impl Dnd {
    pub const ID: &'static str = "reffect-el";

    pub fn set_dragging(element: Element) {
        unsafe { DRAGGED = Some(element.clone()) }
    }

    pub fn render_drag(ui: &Ui, label: impl FnOnce()) -> ElementAction {
        if let Some(_tooltip) = DragDropSource::new(Self::ID)
            .flags(DragDropFlags::SOURCE_NO_DISABLE_HOVER)
            .begin(ui)
        {
            label();
            unsafe {
                if DRAGGED.is_none() {
                    return ElementAction::Drag;
                }
            }
        }
        ElementAction::None
    }

    pub fn render_drop(ui: &Ui) -> Option<Element> {
        if let Some(target) = DragDropTarget::new(ui) {
            if target
                .accept_payload_empty(Self::ID, DragDropFlags::ACCEPT_NO_DRAW_DEFAULT_RECT)
                .is_some()
            {
                return unsafe { DRAGGED.take() };
            }
        }
        None
    }

    pub fn render_drop_children(ui: &Ui, children: &mut Vec<Element>) -> bool {
        if let Some(new) = Self::render_drop(ui) {
            children.push(new);
            true
        } else {
            false
        }
    }
}
