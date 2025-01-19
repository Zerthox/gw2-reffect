mod action;
mod addon;
mod assets;
mod chat_code;
mod context;
mod elements;
mod fmt;
mod id;
mod internal;
mod interval;
mod lockbox;
mod render;
mod schema;
mod serde;
mod settings;
mod texture_manager;
mod tree;
mod trigger;
mod util;

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
