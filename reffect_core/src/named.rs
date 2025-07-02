/// Associated names.
pub trait Named {
    /// Returns the name.
    fn name(&self) -> &'static str;

    /// Returns the short name.
    fn short_name(&self) -> &'static str {
        &self.name()[0..3]
    }
}
