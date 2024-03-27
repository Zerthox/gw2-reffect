use addon::Addon;

mod addon;
mod element;
mod get_buffs;
mod state;

nexus::export! {
    name: "Reffect",
    signature: -0xb359580,
    load: Addon::load,
}
