use arcdps::imgui::{Ui, Window};

arcdps::export! {
    name: "Reffect",
    sig: 0xb359580e,
    options_windows,
    imgui,
}

static mut VISIBLE: bool = true;

fn options_windows(ui: &Ui, option_name: Option<&str>) -> bool {
    if option_name.is_none() {
        ui.checkbox("Reffect buffs debug", unsafe { &mut VISIBLE });
    }
    false
}

fn imgui(ui: &Ui, _not_loading: bool) {
    if unsafe { VISIBLE } {
        Window::new("Reffect buffs debug")
            .scrollable(true)
            .collapsible(false)
            .focus_on_appearing(false)
            .opened(unsafe { &mut VISIBLE })
            .build(ui, || {
                let res = unsafe { reffect::get_buffs() };
                match res {
                    Ok(buffs) => {
                        for buff in buffs {
                            ui.text(format!("{}x {}", buff.count, buff.id));
                        }
                    }
                    Err(err) => ui.text(format!("Error: {}", err)),
                }
            });
    }
}
