pub use reffect_api::*;

/// Use dummy as internal API.
pub type Internal = Dummy;

/// Dummy type.
#[derive(Debug)]
pub struct Dummy;

impl Interface for Dummy {
    #[inline]
    fn update_state(state: &mut State) {
        *state = State::disabled();
    }

    #[inline]
    fn get_player_info() -> Result<PlayerInfo> {
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
