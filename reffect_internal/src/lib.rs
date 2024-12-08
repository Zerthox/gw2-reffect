pub use reffect_api::*;
use std::sync::OnceLock;

/// Use dummy as internal API.
pub type Internal = Dummy;

/// Dummy type.
#[derive(Debug)]
pub struct Dummy;

impl Interface for Dummy {
    #[inline]
    fn init() {}

    #[inline]
    fn update_state(state: &mut State) {
        *state = State::disabled();
    }

    #[inline]
    fn get_player_info() -> Result<PlayerInfo> {
        Err(Error::Disabled)
    }

    #[inline]
    fn get_buff_infos() -> &'static Result<BuffInfoMap> {
        static BUFF_INFOS: Result<BuffInfoMap> = Err(Error::Disabled);
        &BUFF_INFOS
    }
}
