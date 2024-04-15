use crate::traits::Colored;
use nexus::imgui::{
    sys, ComboBoxFlags, InputTextFlags, Selectable, StyleColor, StyleVar, TreeNode, TreeNodeFlags,
    TreeNodeToken, Ui,
};
use std::{ffi::CString, mem};
use strum::VariantArray;

pub fn input_u32(ui: &Ui, label: impl AsRef<str>, value: &mut u32) {
    let mut int = *value as _;
    if ui.input_int(label, &mut int).step(0).step_fast(0).build() {
        if let Ok(new) = u32::try_from(int) {
            *value = new;
        }
    }
}

pub fn input_float_with_format(
    label: impl Into<String>,
    value: &mut f32,
    step: f32,
    step_fast: f32,
    format: impl Into<String>,
    flags: InputTextFlags,
) -> bool {
    if let (Ok(label), Ok(format)) = (CString::new(label.into()), CString::new(format.into())) {
        unsafe {
            sys::igInputFloat(
                label.as_ptr(),
                value as *mut f32,
                step,
                step_fast,
                format.as_ptr(),
                flags.bits() as i32,
            )
        }
    } else {
        false
    }
}

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

pub fn enum_combo_check<T>(
    ui: &Ui,
    label: impl AsRef<str>,
    current: &mut Vec<T>,
    flags: ComboBoxFlags,
) -> bool
where
    T: Clone + PartialEq + Ord + AsRef<str> + VariantArray + Colored,
{
    let mut changed = false;

    const ITEMS: usize = 4;
    const CHARS: usize = 4;

    let mut preview = current
        .first()
        .map(|el| &el.as_ref()[..CHARS])
        .unwrap_or("Any")
        .to_string();
    for el in current.iter().skip(1).take(ITEMS - 1) {
        preview += ", ";
        preview += &el.as_ref()[..CHARS];
    }
    if current.len() > ITEMS {
        preview += "...";
    }

    if let Some(_token) = ui.begin_combo_with_flags(&label, &preview, flags) {
        let _style = ui.push_style_var(StyleVar::FramePadding([0.0, 0.0]));
        let mut focus = false;

        for entry in T::VARIANTS {
            // we assume the vec is sorted
            let found = current.binary_search(entry);
            let mut selected = found.is_ok();
            let _color = entry
                .colored()
                .map(|color| ui.push_style_color(StyleColor::Text, color));

            if ui.checkbox(entry, &mut selected) {
                changed = true;
                match found {
                    Ok(index) => {
                        current.remove(index);
                    }
                    Err(index) => current.insert(index, entry.clone()),
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

pub fn tree_select_empty<'ui>(
    ui: &'ui Ui,
    id: impl AsRef<str>,
    selected: bool,
    leaf: bool,
) -> (Option<TreeNodeToken<'ui>>, bool) {
    let token = TreeNode::new(id)
        .label::<&str, _>("") // FIXME: unused type param in imgui-rs
        .flags(TreeNodeFlags::SPAN_AVAIL_WIDTH)
        .open_on_arrow(true)
        .selected(selected)
        .leaf(leaf)
        .tree_push_on_open(!leaf)
        .push(ui);
    (token, ui.is_item_clicked() && !ui.is_item_toggled_open())
}

pub fn item_context_menu(str_id: impl Into<String>, contents: impl FnOnce()) {
    if let Ok(str_id) = CString::new(str_id.into()) {
        if unsafe {
            sys::igBeginPopupContextItem(
                str_id.as_ptr(),
                sys::ImGuiPopupFlags_MouseButtonRight as _,
            )
        } {
            contents();
            unsafe { sys::igEndPopup() };
        }
    }
}

pub fn confirm_modal(ui: &Ui, title: impl AsRef<str>) -> bool {
    let mut confirm = false;
    ui.popup_modal(title)
        .always_auto_resize(true)
        .build(ui, || {
            if ui.button("Confirm") {
                confirm = true;
                ui.close_current_popup();
            }
            ui.set_item_default_focus();
            ui.same_line();
            if ui.button("Cancel") {
                ui.close_current_popup();
            }
        });
    confirm
}
