#![allow(unused)]

use crate::render::item_context_menu;
use nexus::imgui::{MenuItem, Ui};
use std::fmt;

pub type Action<T> = Box<dyn FnMut(&mut T)>;

pub struct DynAction<T>(Option<Action<T>>);

impl<T> DynAction<T> {
    pub const fn empty() -> Self {
        Self(None)
    }

    pub fn new(action: impl FnMut(&mut T) + 'static) -> Self {
        Self(Some(Box::new(action)))
    }

    pub fn try_new(action: Option<impl FnMut(&mut T) + 'static>) -> Self {
        match action {
            Some(action) => Self::new(action),
            None => Self::empty(),
        }
    }

    pub fn is_none(&self) -> bool {
        !self.is_some()
    }

    pub fn is_some(&self) -> bool {
        self.0.is_some()
    }

    pub fn apply(&mut self, value: &mut T) {
        if let Some(action) = self.0.as_mut() {
            action(value)
        }
    }

    pub fn apply_to_all<'a>(&mut self, iter: impl IntoIterator<Item = &'a mut T>)
    where
        T: 'static,
    {
        if let Some(action) = self.0.as_mut() {
            for item in iter.into_iter() {
                action(item);
            }
        }
    }

    pub fn set(&mut self, action: impl FnMut(&mut T) + 'static) {
        *self = Self::new(action);
    }

    pub fn map<O>(self, mut map: impl (FnMut(&mut O) -> &mut T) + 'static) -> DynAction<O>
    where
        T: 'static,
    {
        DynAction::try_new(
            self.0
                .map(|mut action| move |value: &mut O| action(map(value))),
        )
    }

    pub fn try_map<O>(
        self,
        mut try_map: impl FnMut(&mut O) -> Option<&mut T> + 'static,
    ) -> DynAction<O>
    where
        T: 'static,
    {
        DynAction::try_new(self.0.map(|mut action| {
            move |value: &mut O| {
                if let Some(inner) = try_map(value) {
                    action(inner)
                }
            }
        }))
    }

    pub fn or(&mut self, other: Self) {
        if self.is_none() {
            *self = other;
        }
    }

    pub fn render_copy_all(
        &mut self,
        ui: &Ui,
        id: impl Into<String>,
        action: impl FnMut(&mut T) + 'static,
    ) {
        item_context_menu(id, || {
            if MenuItem::new("Copy to all siblings").build(ui) {
                self.set(action);
            }
        });
    }

    pub fn render_copy_all_cloned(
        &mut self,
        ui: &Ui,
        id: impl Into<String>,
        value: &T,
        mut action: impl FnMut(&mut T, &T) + 'static,
    ) where
        T: Clone + 'static,
    {
        item_context_menu(id, || {
            if MenuItem::new("Copy to all siblings").build(ui) {
                let cloned = value.clone();
                self.set(move |target| action(target, &cloned));
            }
        });
    }
}

impl<T> fmt::Debug for DynAction<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("DynAction")
            .field(&format_args!(
                "{:?}",
                self.0.as_ref().map(|inner| inner.as_ref() as *const _)
            ))
            .finish()
    }
}

/// Render a copy all context menu for the last item.
///
/// Note: this uses the field name as popup id!
#[macro_export]
macro_rules! render_copy_field {
    ($action:expr, $ui:expr, $self:ident . $field:ident) => {{
        let value = $self.$field;
        $action.render_copy_all($ui, stringify!($field), move |other| {
            other.$field = value;
        });
    }};
    ($action:expr, $ui:expr, *$field:ident) => {{
        let value = *$field;
        $action.render_copy_all($ui, stringify!($field), move |other| {
            other.$field = value;
        });
    }};
    ($action:expr, $ui:expr, $field:ident) => {{
        $action.render_copy_all($ui, stringify!($field), move |other| {
            other.$field = $field;
        });
    }};
}

pub use render_copy_field;
