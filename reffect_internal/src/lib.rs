use reffect_core::{
    Texture,
    context::{ItemInfo, SkillInfo},
    error::{Error, Result},
};

pub use reffect_core::Interface;

/// Use dummy as internal API.
pub type Internal = Dummy;

/// Dummy type.
#[derive(Debug)]
pub struct Dummy;

impl Interface for Dummy {
    #[inline]
    fn init() {}

    #[inline]
    fn deinit() {}

    #[inline]
    fn get_item_info(_id: u32) -> Result<ItemInfo> {
        Err(Error::Disabled)
    }

    #[inline]
    fn get_skill_info(_id: u32) -> Result<SkillInfo> {
        Err(Error::Disabled)
    }

    #[inline]
    fn get_skill_icon(_id: u32) -> Option<Texture> {
        None
    }
}
