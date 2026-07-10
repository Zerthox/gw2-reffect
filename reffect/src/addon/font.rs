use super::Addon;
use crate::render::Io;

impl Addon {
    pub fn load_fonts(&mut self, io: Io) {
        log::debug!("Reloading fonts");
        self.settings.font.load(io);
        for pack in &mut self.packs {
            pack.load_fonts(io);
        }
    }
}
