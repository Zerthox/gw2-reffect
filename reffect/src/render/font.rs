use super::Validation;
use nexus::imgui::{ComboBoxFlags, Selectable, SelectableFlags, Ui, sys};
use serde::{Deserialize, Serialize};
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
        unsafe {
            let io = sys::igGetIO();
            let atlas = (*io).Fonts;
            let data = (*atlas).Fonts.Data;
            let len = (*atlas).Fonts.Size;

            slice::from_raw_parts(data, len as usize)
                .iter()
                .copied()
                .filter_map(NonNull::new)
                .map(Self)
        }
    }

    pub fn try_from_name(name: impl AsRef<str>) -> Option<Self> {
        let name = CString::new(name.as_ref()).ok()?;
        unsafe { Self::get_all() }.find(|font| unsafe { font.name_raw() } == name.as_c_str())
    }

    pub fn from_name_or_warn(name: impl AsRef<str>) -> Option<Self> {
        let name = name.as_ref();
        let result = Self::try_from_name(name);
        if result.is_none() {
            log::warn!("Failed to find font \"{name}\"");
        }
        result
    }

    pub fn as_ptr(&self) -> *mut sys::ImFont {
        self.0.as_ptr()
    }

    #[allow(unused)]
    pub fn is_loaded(&self) -> bool {
        unsafe { sys::ImFont_IsLoaded(self.as_ptr()) }
    }

    pub fn is_valid(&self) -> bool {
        unsafe { Self::get_all() }.any(|font| font == *self)
    }

    pub unsafe fn name_raw<'a>(&self) -> &'a CStr {
        unsafe { CStr::from_ptr(sys::ImFont_GetDebugName(self.as_ptr())) }
    }

    pub fn name_owned(&self) -> String {
        unsafe { self.name_raw() }.to_string_lossy().into_owned()
    }

    pub fn push(&self) -> Option<FontToken> {
        self.is_valid().then(|| {
            unsafe { sys::igPushFont(self.as_ptr()) };
            FontToken
        })
    }
}

unsafe impl Send for Font {}

pub struct FontToken;

impl Drop for FontToken {
    fn drop(&mut self) {
        unsafe { sys::igPopFont() }
    }
}

#[allow(unused)]
pub fn font_select(ui: &Ui, label: impl AsRef<str>, current: &mut Option<Font>) -> bool {
    let preview = current
        .map(|current| unsafe { current.name_raw() }.to_string_lossy())
        .unwrap_or(Cow::Borrowed("Inherit"));
    font_select_with_preview(ui, label, preview, current)
}

pub fn font_select_with_preview(
    ui: &Ui,
    label: impl AsRef<str>,
    preview: impl AsRef<str>,
    current: &mut Option<Font>,
) -> bool {
    let mut changed = false;
    if let Some(_token) = ui.begin_combo_with_flags(label, preview, ComboBoxFlags::HEIGHT_LARGE) {
        if Selectable::new("Inherit").build(ui) {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(transparent)]
pub struct LoadedFont {
    name: Option<String>,

    #[serde(skip)]
    loaded: Option<Font>,
}

impl LoadedFont {
    pub const fn empty() -> Self {
        Self {
            name: None,
            loaded: None,
        }
    }

    pub fn loaded(name: Option<String>) -> Self {
        let mut font = Self::empty();
        font.load(name);
        font
    }

    pub fn name(&self) -> &Option<String> {
        &self.name
    }

    pub fn load(&mut self, name: Option<String>) {
        self.name = name;
        self.reload();
    }

    pub fn reload(&mut self) {
        self.loaded = self.name.as_ref().and_then(Font::from_name_or_warn);
    }

    pub fn push(&self) -> Option<FontToken> {
        self.loaded?.push()
    }

    pub fn as_font(&self) -> Option<Font> {
        self.loaded
    }

    pub fn render_select(&mut self, ui: &Ui, label: impl AsRef<str>) -> bool {
        let validation = if self.name.is_some() {
            match self.loaded {
                Some(font) if !font.is_valid() => Validation::Error("Font invalidated"),
                Some(_) => Validation::Ok,
                None => Validation::Error("Failed to find fond"),
            }
        } else {
            Validation::Ok
        };
        validation.for_item(ui, || {
            if font_select_with_preview(
                ui,
                label,
                self.name.as_deref().unwrap_or("Inherit"),
                &mut self.loaded,
            ) {
                self.name = self.loaded.map(|font| font.name_owned());
                true
            } else {
                false
            }
        })
    }
}

impl From<Option<String>> for LoadedFont {
    fn from(name: Option<String>) -> Self {
        Self::loaded(name)
    }
}

impl From<LoadedFont> for Option<String> {
    fn from(font: LoadedFont) -> Self {
        font.name
    }
}
