use super::{Ability, AbilityInfo, SkillId, Slot};
use enumflags2::BitFlags;
use strum::EnumCount;

pub type SkillSlots = [Option<Ability>; Slot::COUNT];

/// Character skillbar.
#[derive(Debug, Default, Clone)]
pub struct Skillbar {
    /// Skill entries.
    pub skills: SkillSlots,
}

impl Skillbar {
    /// Creates an empty skillbar.
    #[inline]
    pub const fn empty() -> Self {
        Self {
            skills: [const { None }; Slot::COUNT],
        }
    }

    /// Returns the ability in the given slot.
    #[inline]
    pub const fn slot(&self, slot: Slot) -> Option<&Ability> {
        self.skills[slot as usize].as_ref()
    }

    /// Returns the ability in the given slot.
    #[inline]
    pub const fn slot_mut(&mut self, slot: Slot) -> Option<&mut Ability> {
        self.skills[slot as usize].as_mut()
    }

    /// Returns the ability with the given identifier.
    #[inline]
    pub fn ability(&self, id: impl Into<SkillId>) -> Option<&Ability> {
        let id = id.into();
        self.skills.iter().flatten().find(|ablity| ablity.id == id)
    }

    /// Sets the ability in the given slot.
    #[inline]
    pub const fn set_slot(&mut self, slot: Slot, ability: Option<Ability>) {
        self.skills[slot as usize] = ability;
    }

    /// Sets the ability info for the ability in the given slot.
    #[inline]
    pub fn set_slot_info(&mut self, slot: Slot, state: impl Into<BitFlags<AbilityInfo>>) {
        if let Some(ability) = self.slot_mut(slot) {
            ability.info.insert(state);
        }
    }
}
