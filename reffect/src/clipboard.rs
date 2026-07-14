use crate::elements::{Element, ElementType};
use nexus::imgui::Ui;
use std::{cell::Cell, thread::LocalKey};

/// Clipboard state.
pub struct Clipboard {
    element: Cell<Option<Element>>,
}

impl Clipboard {
    /// Creates a new clipboard state.
    const fn new() -> Self {
        Self {
            element: Cell::new(None),
        }
    }

    /// Returns the clipboard state.
    fn get() -> &'static LocalKey<Self> {
        thread_local! { static CLIPBOARD: Clipboard = const { Clipboard::new() }; }
        &CLIPBOARD
    }

    /// Sets the clipboard element.
    pub fn set(element: Element) {
        Self::get().with(|clipboard| clipboard.element.set(Some(element)))
    }

    /// Attempts to take the clipboard element.
    pub fn take() -> Option<Element> {
        Self::get().with(|clipboard| clipboard.element.take())
    }

    /// Checks whether the clipboard has an element.
    pub fn has_some() -> bool {
        Self::get()
            .with(|clipboard| unsafe { clipboard.element.as_ptr().as_ref_unchecked() }.is_some())
    }

    /// Checks whether the clipboard has an icon element.
    pub fn has_icon() -> bool {
        Self::get().with(|clipboard| {
            matches!(
                unsafe { clipboard.element.as_ptr().as_ref_unchecked() },
                Some(Element {
                    kind: ElementType::Icon(_),
                    ..
                }),
            )
        })
    }

    /// Renders debug info.
    pub fn debug(ui: &Ui) {
        Self::get().with(|clipboard| {
            ui.text("Clipboard:");
            ui.same_line();
            match unsafe { clipboard.element.as_ptr().as_ref_unchecked() } {
                Some(element) => ui.text(&element.kind),
                None => ui.text_disabled("empty"),
            }
        })
    }
}
