/// Information about an item.
#[derive(Debug, Clone)]
pub enum ItemInfo {
    /// Sigil upgrade.
    Sigil {
        /// Hidden buff applied.
        buff: u32,
    },

    /// Relic.
    Relic {
        /// Hidden buff applied.
        buff: u32,
    },
}

impl ItemInfo {
    #[inline]
    pub const fn buff(&self) -> Option<u32> {
        match *self {
            Self::Sigil { buff } | Self::Relic { buff } => Some(buff),
        }
    }
}
