use num_enum::{FromPrimitive, IntoPrimitive};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIter, IntoStaticStr, VariantArray};

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
    IntoStaticStr,
    Display,
    EnumIter,
    VariantArray,
    FromPrimitive,
    IntoPrimitive,
    Serialize,
    Deserialize,
)]
#[repr(u8)]
pub enum Mount {
    #[default]
    None = 0,
    Jackal = 1,
    Griffon = 2,
    Springer = 3,
    Skimmer = 4,
    Raptor = 5,
    RollerBeetle = 6,
    Warclaw = 7,
    Skyscale = 8,
    Skiff = 9,
    SiegeTurtle = 10,
}
