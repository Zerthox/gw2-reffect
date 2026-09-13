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

    /// Returns the thread-local clipboard state.
    fn local() -> &'static LocalKey<Self> {
        thread_local! { static CLIPBOARD: Clipboard = const { Clipboard::new() }; }
        &CLIPBOARD
    }

    /// Returns an unchecked reference to the clipboard element.
    unsafe fn get_inner(&self) -> &Option<Element> {
        unsafe { self.element.as_ptr().as_ref_unchecked() }
    }

    /// Resets the clipboard state.
    pub fn reset() {
        Self::try_set(None);
    }

    /// Sets or removes the clipboard element.
    pub fn try_set(value: Option<Element>) {
        Self::local().with(|clipboard| clipboard.element.set(value))
    }

    /// Sets the clipboard element.
    pub fn set(element: Element) {
        Self::try_set(Some(element))
    }

    /// Returns the cloned clipboard element.
    pub fn cloned() -> Option<Element> {
        Self::local().with(|clipboard| unsafe { clipboard.get_inner() }.clone())
    }

    /// Checks whether the clipboard has an element.
    pub fn has_some() -> bool {
        Self::local().with(|clipboard| unsafe { clipboard.get_inner() }.is_some())
    }

    /// Checks whether the clipboard has an icon element.
    pub fn has_icon() -> bool {
        Self::local().with(|clipboard| {
            matches!(
                unsafe { clipboard.get_inner() },
                Some(Element {
                    kind: ElementType::Icon(_),
                    ..
                }),
            )
        })
    }

    /// Renders debug info.
    pub fn debug(ui: &Ui) {
        Self::local().with(|clipboard| {
            ui.text("Clipboard:");
            ui.same_line();
            match unsafe { clipboard.get_inner() } {
                Some(element) => ui.text(&element.kind),
                None => ui.text_disabled("empty"),
            }
        })
    }
}
