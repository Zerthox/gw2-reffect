use super::Validation;
use nexus::imgui::{ComboBoxFlags, Selectable, SelectableFlags, Ui, sys};
use serde::{Deserialize, Serialize};
use std::{
    ffi::{CStr, CString},
    ptr::NonNull,
    slice,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Io(pub NonNull<sys::ImGuiIO>);

impl Io {
    /// Forcefully gain access to the IO context.
    pub unsafe fn force() -> Self {
        Self(unsafe { NonNull::new_unchecked(sys::igGetIO()) })
    }

    /// Get access to the IO context via a frame's [`Ui`].
    pub fn get(_ui: &Ui) -> Self {
        unsafe { Self::force() }
    }

    /// Return a reference to the underlying [`sys::ImGuiIO`] struct.
    pub unsafe fn as_ref<'a>(&self) -> &'a sys::ImGuiIO {
        unsafe { self.0.as_ref() }
    }

    /// Returns the fonts.
    pub unsafe fn fonts<'a>(&self) -> impl Iterator<Item = Font> + 'a {
        unsafe {
            let io = self.as_ref();
            let atlas = io.Fonts;
            let data = (*atlas).Fonts.Data;
            let len = (*atlas).Fonts.Size;

            slice::from_raw_parts(data, len as usize)
                .iter()
                .copied()
                .filter_map(Font::from_ptr)
        }
    }

    pub fn default_font(&self) -> Option<Font> {
        Font::from_ptr(unsafe { self.as_ref().FontDefault })
    }
}

impl From<&Ui<'_>> for Io {
    fn from(ui: &Ui) -> Self {
        Self::get(ui)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Font(NonNull<sys::ImFont>);

impl Font {
    pub fn from_ptr(ptr: *mut sys::ImFont) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    pub fn try_from_name(io: Io, name: impl AsRef<str>) -> Option<Self> {
        let name = CString::new(name.as_ref()).ok()?;
        unsafe { io.fonts() }.find(|font| unsafe { font.name_raw() } == name.as_c_str())
    }

    pub fn from_name_or_warn(io: Io, name: impl AsRef<str>) -> Option<Self> {
        let name = name.as_ref();
        let result = Self::try_from_name(io, name);
        if result.is_none() {
            log::warn!("Failed to find font \"{name}\"");
        }
        result
    }

    pub fn as_ptr(&self) -> *mut sys::ImFont {
        self.0.as_ptr()
    }

    pub unsafe fn as_ref<'a>(&self) -> &'a sys::ImFont {
        unsafe { self.0.as_ref() }
    }

    pub fn size(&self) -> f32 {
        unsafe { self.as_ref() }.FontSize
    }

    pub fn is_loaded(&self) -> bool {
        unsafe { sys::ImFont_IsLoaded(self.as_ptr()) }
    }

    pub fn is_valid(&self, io: Io) -> bool {
        unsafe { io.fonts() }.any(|font| font == *self)
    }

    pub unsafe fn name_raw<'a>(&self) -> &'a CStr {
        unsafe { CStr::from_ptr(sys::ImFont_GetDebugName(self.as_ptr())) }
    }

    pub fn name_owned(&self) -> String {
        unsafe { self.name_raw() }.to_string_lossy().into_owned()
    }

    pub fn push(&self, ui: &Ui) -> Option<FontToken> {
        // TODO: skip validation?
        self.is_valid(ui.into()).then(|| {
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

        for font in unsafe { Io::get(ui).fonts() } {
            let _font = font.push(ui);
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

    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            loaded: None,
        }
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn load(&mut self, io: Io) {
        self.loaded = self
            .name
            .as_ref()
            .and_then(|name| Font::from_name_or_warn(io, name));
    }

    pub fn push(&self, ui: &Ui) -> Option<FontToken> {
        self.loaded?.push(ui)
    }

    pub fn as_font(&self) -> Option<Font> {
        self.loaded
    }

    pub fn render_select(&mut self, ui: &Ui, label: impl AsRef<str>) -> bool {
        let validation = if self.name.is_some() {
            match self.loaded {
                Some(font) if !font.is_valid(ui.into()) => Validation::Error("Font invalidated"),
                Some(_) => Validation::Ok,
                None => Validation::Error("Failed to find font"),
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

impl From<LoadedFont> for Option<String> {
    fn from(font: LoadedFont) -> Self {
        font.name
    }
}
