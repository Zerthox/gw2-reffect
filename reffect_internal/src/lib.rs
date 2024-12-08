pub use reffect_api::*;

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
    fn get_player_info() -> Result<PlayerInfo, Error> {
        Err(Error::Disabled)
    }

    #[inline]
    fn get_buff_info() -> &'static Result<BuffInfoMap, Error> {
        static BUFF_INFOS: OnceLock<Result<BuffInfoMap, Error>> = OnceLock::new();

        BUFF_INFOS.get_or_insert(|| Err(Error::Disabled))
    }
}
