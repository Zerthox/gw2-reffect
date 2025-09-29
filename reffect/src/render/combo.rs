use crate::{colors::Colored, enums::EnumStaticVariants, named::Named};
use enumflags2::{BitFlag, BitFlags};
use itertools::Itertools;
use nexus::imgui::{ComboBoxFlags, Selectable, StyleColor, StyleVar, Ui};
use std::mem;
use strum::VariantArray;

pub fn enum_combo<T>(
    ui: &Ui,
    label: impl AsRef<str>,
    current: &mut T,
    flags: ComboBoxFlags,
) -> Option<T>
where
    T: Clone + AsRef<str> + EnumStaticVariants + 'static,
{
    T::with_variants(|iter| enum_combo_iter(ui, label, current, flags, iter))
}

pub fn enum_combo_iter<'i, T>(
    ui: &Ui,
    label: impl AsRef<str>,
    current: &mut T,
    flags: ComboBoxFlags,
    iter: impl IntoIterator<Item = &'i T>,
) -> Option<T>
where
    T: Clone + AsRef<str> + EnumStaticVariants + 'static,
{
    let mut replaced = None;
    if let Some(_token) = ui.begin_combo_with_flags(label, current.as_ref(), flags) {
        for variant in iter {
            // distinguish only discriminants
            let selected = mem::discriminant(variant) == mem::discriminant(current);
            if Selectable::new(variant).selected(selected).build(ui) {
                replaced = Some(mem::replace(current, variant.clone()));
            }

            // handle focus
            if selected {
                ui.set_item_default_focus();
            }
        }
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
    T: Copy + PartialEq + Ord + AsRef<str> + BitFlag + VariantArray + Named + Colored,
    &'static str: From<T>,
{
    let mut changed = false;
    let preview = if !current.is_empty() {
        let mut iter = current.iter();
        let string = iter.clone().take(4).map(|el| el.short_name()).join(",");
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
