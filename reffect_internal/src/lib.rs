use std::sync::{Mutex, MutexGuard};

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
    fn update_state() -> MutexGuard<'static, State> {
        static STATE: Mutex<State> = Mutex::new(State::new());

        STATE.lock().unwrap()
    }

    #[inline]
    fn get_player_info() -> Result<PlayerInfo, Error> {
        Err(Error::Disabled)
    }
}
