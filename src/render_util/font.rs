use nexus::imgui::{sys, ComboBoxFlags, Selectable, SelectableFlags, Ui};
use std::{
    borrow::Cow,
    ffi::{CStr, CString},
    ptr::NonNull,
    slice,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Font(pub NonNull<sys::ImFont>);

impl Font {
    pub unsafe fn get_all() -> impl Iterator<Item = Self> {
        let io = sys::igGetIO();
        let atlas = (*io).Fonts;
        let data = (*atlas).Fonts.Data;
        let len = (*atlas).Fonts.Size;

        slice::from_raw_parts(data, len as usize)
            .into_iter()
            .copied()
            .filter_map(NonNull::new)
            .map(Self)
    }

    pub fn try_from_name(name: impl AsRef<str>) -> Option<Self> {
        let name = CString::new(name.as_ref()).ok()?;
        for font in unsafe { Self::get_all() } {
            if unsafe { font.name_raw() } == name.as_c_str() {
                return Some(font);
            }
        }
        None
    }

    pub unsafe fn name_raw<'a>(&self) -> &'a CStr {
        unsafe {
            let config = (*self.as_ptr()).ConfigData;
            let name = (*config).Name.as_ptr();
            CStr::from_ptr(name)
        }
    }

    pub fn name_owned(&self) -> String {
        unsafe { self.name_raw() }.to_string_lossy().into_owned()
    }

    pub fn as_ptr(&self) -> *mut sys::ImFont {
        self.0.as_ptr()
    }

    pub fn push(&self) -> FontToken {
        unsafe { sys::igPushFont(self.as_ptr()) }
        FontToken
    }
}

pub struct FontToken;

impl Drop for FontToken {
    fn drop(&mut self) {
        unsafe { sys::igPopFont() }
    }
}

unsafe impl Send for Font {}

pub fn font_select(ui: &Ui, label: impl AsRef<str>, current: &mut Option<Font>) -> bool {
    const DEFAULT: &str = "Default";

    let mut changed = false;
    let preview = match *current {
        Some(font) => {
            let name = unsafe { font.name_raw() };
            name.to_string_lossy()
        }
        None => Cow::Borrowed(DEFAULT),
    };

    if let Some(_token) = ui.begin_combo_with_flags(label, preview, ComboBoxFlags::HEIGHT_LARGE) {
        if Selectable::new(DEFAULT).build(ui) {
            *current = None;
            changed = true;
        }

        for font in unsafe { Font::get_all() } {
            let _font = font.push();
            let is_selected = Some(font) == *current;
            if unsafe {
                sys::igSelectable_Bool(
                    font.name_raw().as_ptr(),
                    is_selected,
                    SelectableFlags::empty().bits() as i32,
                    [0.0, 0.0].into(),
                )
            } {
                *current = Some(font);
                changed = true;
            }
            if is_selected {
                ui.set_item_default_focus();
            }
        }
    }

    changed
}
