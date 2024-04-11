mod addon;
mod assets;
mod component_wise;
mod context;
mod elements;
mod get_buffs;
mod interval;
mod state;
mod texture_manager;
mod trigger;
mod util;

use addon::Addon;

nexus::export! {
    name: "Reffect",
    signature: -0xb359580,
    load: Addon::load,
    unload: Addon::unload,
}
