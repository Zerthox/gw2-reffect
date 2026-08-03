use microseh::ExceptionCode;
use std::{fmt, mem, result};
use thiserror::Error;
use windows::Win32::System::Memory::{MEMORY_BASIC_INFORMATION, VirtualQuery};

pub type Result<T> = result::Result<T, Error>;

/// Error information.
#[derive(Debug, Clone, Default, Error)]
pub enum Error {
    #[default]
    #[error("Disabled")]
    Disabled,

    #[error("Not initialized")]
    NotInitialized,

    #[error("No handler")]
    Handler,

    #[error("No Mumble link")]
    Mumble,

    #[error("Unavailable in competitive mode")]
    CompetitiveMode,

    #[error("Game thread not found")]
    GameThread,

    #[error("Context not found")]
    Context,

    #[error("World not found")]
    World,

    #[error("Content not found")]
    Content,

    #[error("Skill not found")]
    Skill,

    #[error("Item not found")]
    Item,

    #[error("User not found")]
    User,

    #[error("Character not found")]
    Character,

    #[error("Unavailable for character state")]
    CharacterState,

    #[error("Unknown affinity")]
    Affinity,

    #[error("Buffs not found")]
    Buffs,

    #[error("Skillbar not found")]
    Skillbar,

    #[error("Health not found")]
    Health,

    #[error("Endurance not found")]
    Endurance,

    #[error("Inventory not found")]
    Inventory,

    #[error("Profession not found")]
    Profession,

    #[error("Specialization not found")]
    Specialization,

    #[error("Build not found")]
    Build,

    #[error("Kennel not found")]
    Kennel,

    #[error("Pet not found")]
    Pet,

    #[error("Owner not found")]
    Owner,

    #[error("Target not found")]
    Target,

    #[error("Unavailable for target state")]
    TargetState,

    #[error("Group not found")]
    Group,

    #[error("{0}")]
    Exception(Box<Exception>),
}

unsafe impl Send for Error {}

#[derive(Debug, Clone, Error)]
pub struct Exception {
    code: ExceptionCode,
    address: usize,
    base: usize,
}

impl fmt::Display for Exception {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Self {
            code,
            address,
            base,
        } = *self;
        write!(f, "Exception at ")?;
        if base != usize::MAX {
            let rva = address.saturating_sub(base);
            write!(f, "0x{base:x}+{rva:x}")?
        } else {
            write!(f, "0x{address:x}")?
        }
        write!(f, ": {code}")
    }
}

impl From<microseh::Exception> for Error {
    #[inline]
    fn from(exception: microseh::Exception) -> Self {
        let address = exception.address();
        let mut info = MEMORY_BASIC_INFORMATION::default();
        let size = mem::size_of_val(&info);
        let written = unsafe { VirtualQuery(Some(address), &mut info, size) };
        let base = if size == written {
            info.BaseAddress as usize
        } else {
            usize::MAX
        };

        Self::Exception(Box::new(Exception {
            code: exception.code(),
            address: address as _,
            base,
        }))
    }
}
