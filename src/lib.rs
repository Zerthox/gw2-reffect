mod addon;
mod assets;
mod colors;
mod component_wise;
mod context;
mod elements;
mod get_buffs;
mod interval;
mod render_util;
mod texture_manager;
mod traits;
mod trigger;

use addon::Addon;

nexus::export! {
    name: "Reffect",
    signature: -0xb359580,
    load: Addon::load,
    unload: Addon::unload,
}
