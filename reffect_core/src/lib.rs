#![allow(clippy::missing_safety_doc)]

pub mod chatlink;
pub mod colors;
pub mod context;
pub mod enums;
pub mod error;
pub mod file;
pub mod fmt;
pub mod id;
pub mod interval;
pub mod links;
pub mod lockbox;
pub mod named;
pub mod profiling;
pub mod serde;
pub mod util;
pub mod worker;

use crate::{
    context::{ItemInfo, SkillInfo},
    error::Result,
};
use nexus::imgui;
use windows::Win32::Graphics::Direct3D11::ID3D11ShaderResourceView;

pub type Texture = ID3D11ShaderResourceView;

pub type Ui<'ui> = imgui::Ui<'ui>;

/// Interface for internal API.
pub trait Interface {
    /// Initializes the internal API.
    fn init();

    /// Deinitializes the internal API.
    fn deinit();

    /// Retrieves item information.
    fn get_item_info(id: u32) -> Result<ItemInfo>;

    /// Retrieves item id from buff id.
    fn get_item_from_buff(id: u32) -> Result<u32>;

    /// Retrieves skill information.
    fn get_skill_info(id: u32) -> Result<SkillInfo>;

    /// Retrieves skill icon.
    fn get_skill_icon(ui: &Ui, id: u32) -> Option<Texture>;
}
