use crate::{
    action::Action,
    context::{Context, Update, Weapon},
    render::{enum_combo_bitflags, helper, input_item_id},
    serde::bitflags,
    trigger::{Mode, Trigger, check_bitflags_optional},
};
use const_default::ConstDefault;
use enumflags2::BitFlags;
use nexus::imgui::{ComboBoxFlags, InputTextFlags, Ui};
use reffect_core::Interface;
use reffect_internal::Internal;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct GearTrigger {
    #[serde(with = "bitflags")]
    pub weapons: BitFlags<Weapon>,

    pub sigils: Vec<Item>,
    pub sigil_mode: Mode,

    pub relics: Vec<Item>,

    #[serde(skip)]
    active: bool,
}

impl GearTrigger {
    pub fn needs_update(&self, ctx: &Context) -> bool {
        ctx.has_update(Update::Gear)
    }

    pub fn update(&mut self, ctx: &Context) {
        self.active = self.weapons_active(ctx) && self.sigils_active(ctx) && self.relic_active(ctx);
    }

    pub fn weapons_active(&self, ctx: &Context) -> bool {
        let gear = ctx.player.gear.as_ref();
        check_bitflags_optional(self.weapons, gear.map(|gear| gear.weapons).ok())
    }

    pub fn sigils_active(&self, ctx: &Context) -> bool {
        if let Ok(gear) = ctx.player.gear.as_ref() {
            self.sigil_mode
                .check_iter(&self.sigils, |sigil| gear.sigils.contains(&sigil.buff))
        } else {
            true
        }
    }

    pub fn relic_active(&self, ctx: &Context) -> bool {
        if let Ok(gear) = ctx.player.gear.as_ref() {
            self.relics.iter().any(|relic| gear.relic == relic.buff)
        } else {
            true
        }
    }

    pub fn render_options(&mut self, ui: &Ui, ctx: &Context) -> bool {
        let _id = ui.push_id("gear");
        let mut changed = false;

        changed |= enum_combo_bitflags(
            ui,
            "Weapons",
            &mut self.weapons,
            ComboBoxFlags::HEIGHT_LARGEST,
        );

        changed |= self.sigil_mode.render_options(ui, "Sigil Mode");
        changed |= Self::render_item_inputs(ui, "Sigil", &mut self.sigils);

        changed |= Self::render_item_inputs(ui, "Relic", &mut self.relics);

        if changed {
            // ensure fresh state after changed
            self.update(ctx);
        }

        changed
    }

    fn render_item_inputs(ui: &Ui, label: impl AsRef<str>, items: &mut Vec<Item>) -> bool {
        let label = label.as_ref();
        let mut changed = false;

        let mut action = Action::new();
        for (i, entry) in items.iter_mut().enumerate() {
            let _id = ui.push_id(i as i32);
            let entry_changed = action.input_with_buttons(ui, i, || {
                input_item_id(
                    ui,
                    format!("##{label}"),
                    &mut entry.item,
                    InputTextFlags::empty(),
                )
            });
            if entry_changed {
                entry.update();
                changed = true;
            }

            ui.same_line();
            ui.text(format!("Item Id {}", i + 1));

            if i == 0 {
                helper(ui, || {
                    ui.text("Can be found on the wiki, same as in GW2 API");
                    ui.text("Supports pasting item chat links");
                });
            }
        }
        changed |= action.perform(items);

        if ui.button(format!("Add {label}")) {
            items.push(Item::empty());
            changed = true;
        }

        changed
    }
}

impl Trigger for GearTrigger {
    fn is_active(&mut self, ctx: &Context) -> bool {
        if self.needs_update(ctx) {
            self.update(ctx);
        }
        self.active
    }
}

impl ConstDefault for GearTrigger {
    const DEFAULT: Self = Self {
        weapons: BitFlags::EMPTY,
        sigils: Vec::new(),
        sigil_mode: Mode::All,
        relics: Vec::new(),
        active: true,
    };
}

impl Default for GearTrigger {
    fn default() -> Self {
        Self::DEFAULT
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
    /// Item id.
    pub item: u32,

    /// Hidden buff id.
    pub buff: u32,
}

impl Item {
    pub const fn empty() -> Self {
        Self { item: 0, buff: 0 }
    }

    pub fn update(&mut self) {
        self.buff = Internal::get_item_info(self.item)
            .ok()
            .and_then(|info| info.buff())
            .unwrap_or(0);
    }
}
