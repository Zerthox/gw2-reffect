use crate::{
    api::Weapon,
    render::colors::{Color, Colored},
    util::ShortName,
};

impl ShortName for Weapon {
    fn short_name(&self) -> &'static str {
        &<&str>::from(self)[0..3]
    }
}

impl Colored for Weapon {
    fn colored(&self) -> Option<Color> {
        None
    }
}
