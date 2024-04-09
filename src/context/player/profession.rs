use num_enum::{FromPrimitive, IntoPrimitive};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIter};

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    AsRefStr,
    Display,
    EnumIter,
    FromPrimitive,
    IntoPrimitive,
    Serialize,
    Deserialize,
)]
#[repr(u8)]
pub enum Profession {
    #[default]
    Unknown = 0,
    Guardian = 1,
    Warrior = 2,
    Engineer = 3,
    Ranger = 4,
    Thief = 5,
    Elementalist = 6,
    Mesmer = 7,
    Necromancer = 8,
    Revenant = 9,
}
