use crate::elements::{Element, ElementType};
use nexus::imgui::Ui;
use std::{cell::Cell, thread::LocalKey};

/// Clipboard state.
pub struct Clipboard {
    element: Cell<Option<Element>>,
}

impl Clipboard {
    const fn new() -> Self {
        Self {
            element: Cell::new(None),
        }
    }

    fn get() -> &'static LocalKey<Self> {
        thread_local! { static CLIPBOARD: Clipboard = const { Clipboard::new() }; }
        &CLIPBOARD
    }

    pub fn set(element: Element) {
        Self::get().with(|clipboard| clipboard.element.set(Some(element)))
    }

    pub fn take() -> Option<Element> {
        Self::get().with(|clipboard| clipboard.element.take())
    }

    pub fn has_some() -> bool {
        Self::get().with(|clipboard| unsafe { &*clipboard.element.as_ptr() }.is_some())
    }

    pub fn has_icon() -> bool {
        Self::get().with(|clipboard| {
            matches!(
                unsafe { &*clipboard.element.as_ptr() },
                Some(Element {
                    kind: ElementType::Icon(_),
                    ..
                }),
            )
        })
    }

    pub fn debug(ui: &Ui) {
        Self::get().with(|clipboard| {
            ui.text("Clipboard:");
            ui.same_line();
            match unsafe { &*clipboard.element.as_ptr() } {
                Some(element) => ui.text(&element.kind),
                None => ui.text_disabled("empty"),
            }
        })
    }
}
