mod addon;
mod assets;
mod component_wise;
mod context;
mod element;
mod get_buffs;
mod interval;
mod player;
mod texture_manager;
mod trigger;

use addon::Addon;

nexus::export! {
    name: "Reffect",
    signature: -0xb359580,
    load: Addon::load,
    unload: Addon::unload,
}
