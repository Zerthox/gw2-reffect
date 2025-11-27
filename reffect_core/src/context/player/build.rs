#[derive(Debug, Clone)]
pub struct Build {
    /// Selected traits.
    pub traits: Traits,
}

impl Build {
    #[inline]
    pub const fn empty() -> Self {
        Self { traits: [0; 9] }
    }
}

/// Player traits.
pub type Traits = [u32; 9];
