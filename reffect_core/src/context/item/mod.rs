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
