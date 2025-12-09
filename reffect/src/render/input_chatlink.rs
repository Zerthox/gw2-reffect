use super::input_text_simple_menu;
use crate::chatlink::{decode_item, decode_skill, decode_trait};
use nexus::imgui::{InputTextFlags, Ui};

pub fn input_chatlink(
    ui: &Ui,
    label: impl AsRef<str>,
    id: &mut u32,
    flags: InputTextFlags,
    decode: impl FnOnce(&str) -> Option<u32>,
) -> bool {
    let label = label.as_ref();
    let mut text = id.to_string(); // TODO: switch to faster int/float to string conversion libraries
    let changed = ui
        .input_text(label, &mut text)
        .flags(flags | InputTextFlags::AUTO_SELECT_ALL | InputTextFlags::CALLBACK_RESIZE)
        .build();
    if changed {
        if let Ok(new) = text.parse() {
            *id = new;
        } else if let Some(new) = decode(text.trim()) {
            *id = new;
        }
    }
    input_text_simple_menu(ui, format!("##{label}ctx"), &mut text);
    changed
}

pub fn input_item_id(ui: &Ui, label: impl AsRef<str>, id: &mut u32, flags: InputTextFlags) -> bool {
    input_chatlink(ui, label, id, flags, decode_item)
}

pub fn input_skill_id(
    ui: &Ui,
    label: impl AsRef<str>,
    id: &mut u32,
    flags: InputTextFlags,
) -> bool {
    input_chatlink(ui, label, id, flags, decode_skill)
}

pub fn input_trait_id(
    ui: &Ui,
    label: impl AsRef<str>,
    id: &mut u32,
    flags: InputTextFlags,
) -> bool {
    input_chatlink(ui, label, id, flags, decode_trait)
}
