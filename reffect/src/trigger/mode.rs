use crate::render::enum_combo;
use enumflags2::{BitFlag, BitFlags};
use nexus::imgui::{ComboBoxFlags, Ui};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumIter, VariantArray};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    AsRefStr,
    EnumIter,
    VariantArray,
    Serialize,
    Deserialize,
)]
pub enum Mode {
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

impl Mode {
    pub fn check_iter<T>(
        &self,
        iter: impl IntoIterator<Item = T>,
        predicate: impl FnMut(T) -> bool,
    ) -> bool {
        match self {
            Self::Any => iter.into_iter().any(predicate),
            Self::All => iter.into_iter().all(predicate),
            Self::NotAny => !Self::Any.check_iter(iter, predicate),
            Self::NotAll => !Self::All.check_iter(iter, predicate),
        }
    }

    pub fn check_flags<T>(&self, expected: BitFlags<T>, got: BitFlags<T>) -> bool
    where
        T: BitFlag,
    {
        match self {
            Self::Any => got.intersects(expected),
            Self::All => got.contains(expected),
            Self::NotAny => !Self::Any.check_flags(expected, got),
            Self::NotAll => !Self::All.check_flags(expected, got),
        }
    }

    pub fn render_options(&mut self, ui: &Ui, label: impl AsRef<str>) -> bool {
        enum_combo(ui, label, self, ComboBoxFlags::empty()).is_some()
    }
}
