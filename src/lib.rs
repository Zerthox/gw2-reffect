mod action;
mod addon;
mod assets;
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
mod trigger;
mod util;
mod visit;

use addon::Addon;
use nexus::AddonFlags;

nexus::export! {
    name: "Reffect (alpha)",
    signature: -0xb359580,
    flags: AddonFlags::IsVolatile,
    load: Addon::load,
    unload: Addon::unload,
}
