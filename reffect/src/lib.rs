pub mod element;
mod get_buffs;

pub use get_buffs::get_buffs;

pub(crate) fn add_pos(a: [f32; 2], b: [f32; 2]) -> [f32; 2] {
    let [ax, ay] = a;
    let [bx, by] = b;
    [ax + bx, ay + by]
}
