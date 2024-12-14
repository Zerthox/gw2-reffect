mod buff;
mod error;
mod player;
mod resource;
mod skillbar;
mod state;

pub use self::{buff::*, error::*, player::*, resource::*, skillbar::*, state::*};

/// Interface for API.
pub trait Interface {
    /// Initializes the API.
    fn init();

    /// Updates and returns the current state.
    fn update_state(state: &mut State);

    /// Retrieves player information.
    fn get_player_info() -> Result<PlayerInfo>;

    /// Retrives buff information.
    fn get_buff_infos() -> &'static Result<BuffInfoMap>;
}
