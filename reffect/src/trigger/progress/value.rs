#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ProgressValue {
    Primary,
    Secondary,
    PreferPrimary,
    PreferSecondary,
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
