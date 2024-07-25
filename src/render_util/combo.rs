use crate::traits::{Colored, ShortName};
use enumflags2::{BitFlag, BitFlags};
use nexus::imgui::{ComboBoxFlags, Selectable, StyleColor, StyleVar, Ui};
use std::mem;
use strum::VariantArray;

pub trait EnumStaticVariants: Sized {
    fn with_variants<R>(action: impl FnOnce(&[Self]) -> R) -> R;
}

impl<T> EnumStaticVariants for T
where
    T: VariantArray,
{
    fn with_variants<R>(action: impl FnOnce(&[Self]) -> R) -> R {
        action(Self::VARIANTS)
    }
}

/// Helper to implement [`EnumStaticVariants`] for enums already implementing [`IntoEnumIterator`].
macro_rules! impl_static_variants {
    ($ty:ty) => {
        impl $crate::render_util::EnumStaticVariants for $ty {
            fn with_variants<R>(action: impl FnOnce(&[Self]) -> R) -> R {
                use ::std::thread_local;
                use ::strum::IntoEnumIterator;

                thread_local! { static VARIANTS: Vec<$ty> = <$ty as IntoEnumIterator>::iter().collect(); };
                VARIANTS.with(|variants| action(variants))
            }
        }
    };
}

pub(crate) use impl_static_variants;

pub fn enum_combo<T>(
    ui: &Ui,
    label: impl AsRef<str>,
    current: &mut T,
    flags: ComboBoxFlags,
) -> Option<T>
where
    T: Clone + AsRef<str> + EnumStaticVariants + 'static,
{
    let mut replaced = None;
    if let Some(_token) = ui.begin_combo_with_flags(label, current.as_ref(), flags) {
        T::with_variants(|variants| {
            for entry in variants {
                // distinguish only discriminants
                let selected = mem::discriminant(entry) == mem::discriminant(current);
                if Selectable::new(entry).selected(selected).build(ui) {
                    replaced = Some(mem::replace(current, entry.clone()));
                }

                // handle focus
                if selected {
                    ui.set_item_default_focus();
                }
            }
        });
    }
    replaced
}

pub fn enum_combo_bitflags<T>(
    ui: &Ui,
    label: impl AsRef<str>,
    current: &mut BitFlags<T>,
    flags: ComboBoxFlags,
) -> bool
where
    T: Copy + PartialEq + Ord + AsRef<str> + BitFlag + VariantArray + ShortName + Colored,
    &'static str: From<T>,
{
    let mut changed = false;

    let mut iter = current.iter();
    let preview = if let Some(first) = iter.next() {
        let string = iter
            .clone()
            .take(4)
            .map(|el| el.short_name())
            .fold(first.short_name().to_string(), |acc, el| acc + "," + el);
        if iter.nth(4).is_some() {
            string + ",..."
        } else {
            string
        }
    } else {
        "Any".into()
    };

    if let Some(_token) = ui.begin_combo_with_flags(&label, &preview, flags) {
        let _style = ui.push_style_var(StyleVar::FramePadding([0.0, 0.0]));
        let mut focus = false;

        for entry in T::VARIANTS.iter().copied() {
            let mut selected = current.contains(entry);
            let _color = entry
                .colored()
                .map(|color| ui.push_style_color(StyleColor::Text, color));

            if ui.checkbox(entry, &mut selected) {
                changed = true;
                if selected {
                    current.insert(entry);
                } else {
                    current.remove(entry);
                }
            }
            if !focus && selected {
                ui.set_item_default_focus();
                focus = true;
            }
        }
    }
    changed
}
