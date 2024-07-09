mod action;
mod addon;
mod assets;
mod bounds;
mod colors;
mod component_wise;
mod context;
mod elements;
mod id;
mod internal;
mod interval;
mod lockbox;
mod render_util;
mod schema;
mod serde_bitflags;
mod texture_manager;
mod traits;
mod tree;
mod trigger;
mod util;

use addon::Addon;
use nexus::AddonFlags;

nexus::export! {
    name: "Reffect (alpha)",
    signature: -0xb359580,
    flags: AddonFlags::IsVolatile,
    load: Addon::load,
    unload: Addon::unload,
}
