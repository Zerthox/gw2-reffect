use crate::render::enum_combo;
use enumflags2::{BitFlag, BitFlags};
use nexus::imgui::{ComboBoxFlags, Ui};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumCount, EnumIter, IntoStaticStr, VariantArray};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Display,
    AsRefStr,
    IntoStaticStr,
    EnumIter,
    EnumCount,
    VariantArray,
    Serialize,
    Deserialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum TriggerMode {
    #[strum(serialize = "Any of")]
    Any,

    #[strum(serialize = "All of")]
    All,

    #[strum(serialize = "None of")]
    #[serde(alias = "None")]
    NotAny,

    #[strum(serialize = "Not all of")]
    NotAll,
}

impl TriggerMode {
    /// Checks the iterator with the given predicate while returning `false` for empty iterators.
    pub fn check_iter_non_empty<T>(
        &self,
        iter: impl IntoIterator<Item = T>,
        predicate: impl FnMut(T) -> bool,
    ) -> bool {
        match self {
            Self::Any => iter.into_iter().any(predicate),
            Self::All => iter.into_iter().all(predicate),
            Self::NotAny => !Self::Any.check_iter_non_empty(iter, predicate),
            Self::NotAll => !Self::All.check_iter_non_empty(iter, predicate),
        }
    }

    /// Checks the slice with the given predicate while returning `false` for empty slices.
    pub fn check_slice_non_empty<T>(&self, slice: &[T], predicate: impl FnMut(&T) -> bool) -> bool {
        self.check_iter_non_empty(slice, predicate)
    }

    /// Checks the slice with the given predicate while returning `true` for empty slices.
    pub fn check_slice<T>(&self, slice: &[T], predicate: impl FnMut(&T) -> bool) -> bool {
        slice.is_empty() || self.check_slice_non_empty(slice, predicate)
    }

    /// Checks the given flags while returning `false` for empty flags.
    pub fn check_flags_non_empty<T>(
        &self,
        expected: BitFlags<T>,
        got: impl Into<BitFlags<T>>,
    ) -> bool
    where
        T: BitFlag,
    {
        match self {
            Self::Any => got.into().intersects(expected),
            Self::All => got.into().contains(expected),
            Self::NotAny => !Self::Any.check_flags_non_empty(expected, got),
            Self::NotAll => !Self::All.check_flags_non_empty(expected, got),
        }
    }

    /// Checks the given flags while returning `true` for empty flags.
    pub fn check_flags<T>(&self, expected: BitFlags<T>, got: impl Into<BitFlags<T>>) -> bool
    where
        T: BitFlag,
    {
        expected.is_empty() || self.check_flags_non_empty(expected, got)
    }

    /// Checks the given flags while returning `true` for empty flags.
    pub fn check_flags_optional<T>(
        &self,
        expected: BitFlags<T>,
        got: Option<impl Into<BitFlags<T>>>,
    ) -> bool
    where
        T: BitFlag,
    {
        match got {
            Some(got) => self.check_flags(expected, got),
            None => true,
        }
    }

    pub fn render_options(&mut self, ui: &Ui, label: impl AsRef<str>) -> bool {
        enum_combo(ui, label, self, ComboBoxFlags::empty()).is_some()
    }
}
