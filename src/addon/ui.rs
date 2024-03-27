use super::Addon;
use crate::{
    element::Render,
    get_buffs::get_buffs,
    state::{MapInfo, State},
};
use nexus::{
    data_link::get_mumble_link,
    imgui::{Ui, Window},
};

impl Addon {
    pub fn render(&mut self, ui: &Ui) {
        match unsafe { get_buffs() } {
            Ok(buffs) => {
                let mumble = unsafe { get_mumble_link().as_ref() };
                let map_info = mumble.map(|mumble| MapInfo::from_mumble(&mumble.context));
                let state = State::new(buffs, map_info);

                for element in &mut self.elements {
                    element.render(ui, &state);
                }
            }
            Err(err) => {
                Window::new("Reffect Error##reffect-getbuffs-error")
                    .always_auto_resize(true)
                    .build(ui, || {
                        ui.text_colored([1.0, 0.0, 0.0, 1.0], format!("{err}"));
                    });
            }
        }
    }

    pub fn render_options(&mut self, ui: &Ui) {
        let count = self.elements.len();
        ui.text(format!("{count} elements loaded"));

        if ui.button("Reload elements") {
            self.elements.clear();
            self.load_elements();
        }
    }
}
