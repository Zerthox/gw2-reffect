use const_default::ConstDefault;

/// A progress value selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ProgressValue {
    /// Select primary progress.
    Primary,

    /// Select secondary progress.
    Secondary,

    /// Prefer primary but fall back to secondary progress.
    PreferPrimary,

    /// Prefer secondary but fall back to primary progress.
    PreferSecondary,
}

impl ConstDefault for ProgressValue {
    const DEFAULT: Self = Self::Primary;
}

impl ProgressValue {
    pub fn pick(&self, primary: u32, secondary: u32) -> u32 {
        match self {
            Self::Primary => primary,
            Self::Secondary => secondary,
            Self::PreferPrimary => {
                if primary > 0 {
                    primary
                } else {
                    secondary
                }
            }
            Self::PreferSecondary => {
                if secondary > 0 {
                    secondary
                } else {
                    primary
                }
            }
        }
    }
}
