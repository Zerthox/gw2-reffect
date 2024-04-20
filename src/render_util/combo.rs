use crate::traits::Colored;
use enumflags2::{BitFlag, BitFlags};
use nexus::imgui::{ComboBoxFlags, Selectable, StyleColor, StyleVar, Ui};
use std::mem;
use strum::VariantArray;

pub trait EnumStaticVariants: Sized {
    fn static_variants() -> &'static [Self];
}

impl<T> EnumStaticVariants for T
where
    T: VariantArray,
{
    fn static_variants() -> &'static [Self] {
        Self::VARIANTS
    }
}

/// Helper to implement [`EnumStaticVariants`] for enums already implementing [`IntoEnumIterator`].
macro_rules! impl_static_variants {
    ($ty:ty) => {
        impl $crate::render_util::EnumStaticVariants for $ty {
            fn static_variants() -> &'static [Self] {
                use ::std::sync::OnceLock;
                use ::strum::IntoEnumIterator;

                static VARIANTS: OnceLock<Vec<$ty>> = OnceLock::new();
                VARIANTS.get_or_init(|| <Self as IntoEnumIterator>::iter().collect())
            }
        }
    };
}

pub(crate) use impl_static_variants;

pub fn enum_combo<T>(ui: &Ui, label: impl AsRef<str>, current: &mut T, flags: ComboBoxFlags) -> bool
where
    T: Clone + AsRef<str> + EnumStaticVariants + 'static,
{
    let mut changed = false;
    if let Some(_token) = ui.begin_combo_with_flags(label, &current, flags) {
        for entry in T::static_variants() {
            // distinguish only discriminants
            let selected = mem::discriminant(entry) == mem::discriminant(current);
            if Selectable::new(entry).selected(selected).build(ui) {
                changed = true;
                *current = entry.clone();
            }

            // handle focus
            if selected {
                ui.set_item_default_focus();
            }
        }
    }
    changed
}

pub fn enum_combo_bitflags<T>(
    ui: &Ui,
    label: impl AsRef<str>,
    current: &mut BitFlags<T>,
    flags: ComboBoxFlags,
) -> bool
where
    T: Copy + PartialEq + Ord + AsRef<str> + BitFlag + VariantArray + Colored,
    &'static str: From<T>,
{
    let mut changed = false;

    let preview = current
        .iter()
        .take(5)
        .map(|el| &<&'static str>::from(el)[..4])
        .fold(String::new(), |acc, el| acc + el + " ");

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
