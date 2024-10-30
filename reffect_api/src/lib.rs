mod buff;
mod error;
mod player;
mod resource;
mod state;

use std::sync::MutexGuard;

pub use self::{buff::*, error::*, player::*, resource::*, state::*};

/// Interface for API.
pub trait Interface {
    /// Initializes the API.
    fn init();

    /// Updates and returns the current state.
    fn update_state() -> MutexGuard<'static, State>;

    /// Retrieves player information.
    fn get_player_info() -> Result<PlayerInfo, Error>;
}
