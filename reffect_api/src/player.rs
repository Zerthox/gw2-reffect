/// Player information.
pub struct PlayerInfo {
    /// Current equipped weapons.
    pub weapons: Weapons,

    /// Current selected traits.
    pub traits: Traits,
}

/// Player weapon sets.
pub type Weapons = [u32; 4];

/// Player traits.
pub type Traits = [u32; 9];
