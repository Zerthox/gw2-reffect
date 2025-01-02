use crate::elements::{Element, ElementType};
use nexus::imgui::Ui;
use std::{cell::UnsafeCell, fmt};

/// Clipboard state.
#[repr(transparent)]
pub struct Clipboard<T> {
    contents: UnsafeCell<Option<T>>, // never give out references to this!
}

impl<T> Clipboard<T> {
    unsafe fn get(&self) -> &Option<T> {
        self.contents.get().as_ref().unwrap_unchecked()
    }

    #[allow(clippy::mut_from_ref)]
    unsafe fn get_mut(&self) -> &mut Option<T> {
        self.contents.get().as_mut().unwrap_unchecked()
    }

    pub fn has_some(&self) -> bool {
        unsafe { self.get() }.is_some()
    }

    pub fn take(&self) -> Option<T> {
        unsafe { self.get_mut() }.take()
    }

    pub fn set(&self, element: T) {
        *unsafe { self.get_mut() } = Some(element);
    }
}

impl Clipboard<Element> {
    pub fn has_icon(&self) -> bool {
        matches!(
            unsafe { self.get() },
            Some(Element {
                kind: ElementType::Icon(_),
                ..
            }),
        )
    }

    pub fn debug(&self, ui: &Ui) {
        match unsafe { self.get() } {
            Some(element) => ui.text(&element.kind),
            None => ui.text_disabled("empty"),
        }
    }
}

impl<T> Default for Clipboard<T> {
    fn default() -> Self {
        Self {
            contents: UnsafeCell::new(None),
        }
    }
}

impl<T> fmt::Debug for Clipboard<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Clipboard")
            .field("element", unsafe { self.get() })
            .finish()
    }
}
