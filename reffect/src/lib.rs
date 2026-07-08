mod action;
mod addon;
mod assets;
mod clipboard;
mod render;
mod texture;

pub mod elements;
pub mod schema;
pub mod settings;
pub mod tree;
pub mod trigger;

pub use reffect_core::*;
pub use reffect_internal as internal;

use addon::Addon;
use nexus::{AddonFlags, UpdateProvider};

nexus::export! {
    name: "Reffect (alpha)",
    signature: -0xb359580,
    flags: AddonFlags::IsVolatile,
    load: Addon::load,
    unload: Addon::unload,
    provider: UpdateProvider::GitHub,
    update_link: env!("CARGO_PKG_REPOSITORY"),
}
