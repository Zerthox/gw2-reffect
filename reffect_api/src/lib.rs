mod ability;
mod buff;
mod error;
mod player;
mod resource;
mod skill;
mod state;

use windows::Win32::Graphics::Direct3D11::ID3D11ShaderResourceView;

pub use self::{ability::*, buff::*, error::*, player::*, resource::*, skill::*, state::*};

pub type Texture = ID3D11ShaderResourceView;

/// Interface for API.
pub trait Interface {
    /// Initializes the API.
    #[inline]
    fn init() {}

    /// Deinitializes the API.
    #[inline]
    fn deinit() {}

    /// Updates and returns the current state.
    fn update_state(state: &mut State);

    /// Retrieves player information.
    fn get_player_info() -> Result<PlayerInfo>;

    /// Retrieves skill information.
    fn get_skill_info(id: u32) -> Result<SkillInfo>;

    /// Retrieves skill icon.
    fn get_skill_icon(id: u32) -> Option<Texture>;
}
