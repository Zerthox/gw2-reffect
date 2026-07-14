use super::Item;
use crate::{
    action::Action,
    context::{Context, Update, Updateable, Weapon},
    render::{enum_combo_bitflags, helper, input_item_id},
    serde::bitflags,
    trigger::TriggerMode,
};
use const_default::ConstDefault;
use enumflags2::BitFlags;
use nexus::imgui::{ComboBoxFlags, InputTextFlags, Ui};
use serde::{Deserialize, Serialize};

/// Player gear trigger.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct GearTrigger {
    /// Equipped weapon types.
    #[serde(with = "bitflags")]
    #[cfg_attr(feature = "schema", schemars(with = "bitflags::Schema<Weapon>"))]
    pub weapons: BitFlags<Weapon>,

    /// Trigger logic mode for weapons.
    pub weapon_mode: TriggerMode,

    /// Equipped sigils.
    pub sigils: Vec<Item>,

    /// Trigger logic mode for sigils.
    pub sigil_mode: TriggerMode,

    /// Equipped relic.
    pub relics: Vec<Item>,

    #[serde(skip)]
    active: bool,
}

impl GearTrigger {
    /// Returns whether the gear trigger is active.
    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn needs_item_update(&self, ctx: &Context) -> bool {
        ctx.has_update(Update::Map) && ctx.map.is_valid()
    }

    fn update_items(&mut self) {
        for item in self.sigils.iter_mut().chain(&mut self.relics) {
            item.update();
        }
    }

    /// Resolves whether weapon types match.
    fn weapons_active(&self, ctx: &Context) -> bool {
        let gear = ctx.player.gear.as_ref();
        self.weapon_mode
            .check_flags_optional(self.weapons, gear.map(|gear| gear.weapons).ok())
    }

    /// Resolves whether sigils match.
    fn sigils_active(&self, ctx: &Context) -> bool {
        if let Ok(gear) = ctx.player.gear.as_ref() {
            self.sigil_mode
                .check_slice(&self.sigils, |sigil| gear.sigils.contains(&sigil.buff))
        } else {
            true
        }
    }

    /// Resolves whether the relic matches.
    fn relic_active(&self, ctx: &Context) -> bool {
        if let Ok(gear) = ctx.player.gear.as_ref() {
            TriggerMode::Any.check_slice(&self.relics, |relic| gear.relic == relic.buff)
        } else {
            true
        }
    }

    /// Renders gear trigger options.
    pub fn render_options(&mut self, ui: &Ui, ctx: &Context) -> bool {
        let _id = ui.push_id("gear");
        let mut changed = false;

        changed |= self.weapon_mode.render_options(ui, "Weapon Mode");
        changed |= enum_combo_bitflags(
            ui,
            "Weapons",
            &mut self.weapons,
            ComboBoxFlags::HEIGHT_LARGEST,
        );
        helper(ui, || ui.text("Equipped weapons (active or inactive)"));

        changed |= self.sigil_mode.render_options(ui, "Sigil Mode");
        changed |= Self::render_item_inputs(ui, "Sigil", &mut self.sigils);
        helper(ui, || ui.text("Equipped sigils (active or inactive"));

        changed |= Self::render_item_inputs(ui, "Relic", &mut self.relics);
        helper(ui, || ui.text("Equipped relic"));

        if changed {
            // ensure fresh state after changed
            self.force_update(ctx);
        }

        changed
    }

    /// Renders item inputs.
    fn render_item_inputs(ui: &Ui, label: impl AsRef<str>, items: &mut Vec<Item>) -> bool {
        let label = label.as_ref();
        let mut changed = false;

        let mut action = Action::new();
        for (i, entry) in items.iter_mut().enumerate() {
            let _id = ui.push_id(i as i32);
            action.input_with_buttons(ui, i, || {
                entry.validate().for_item(ui, || {
                    if input_item_id(
                        ui,
                        format!("##{label}"),
                        &mut entry.item,
                        InputTextFlags::empty(),
                    ) {
                        entry.update();
                        changed = true;
                    }
                })
            });

            ui.same_line();
            ui.text(format!("{label} Id {}", i + 1));

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

impl Updateable for GearTrigger {
    fn needs_update(&self, ctx: &Context) -> bool {
        ctx.has_update_or_edit(Update::PlayerGear) || self.needs_item_update(ctx)
    }

    fn force_update(&mut self, ctx: &Context) {
        self.active = self.weapons_active(ctx) && self.sigils_active(ctx) && self.relic_active(ctx);
    }

    fn update_if_need(&mut self, ctx: &Context) {
        if self.needs_item_update(ctx) {
            self.update_items();
        }
        if self.needs_update(ctx) {
            self.force_update(ctx);
        }
    }
}

impl ConstDefault for GearTrigger {
    const DEFAULT: Self = Self {
        weapons: BitFlags::EMPTY,
        weapon_mode: TriggerMode::Any,
        sigils: Vec::new(),
        sigil_mode: TriggerMode::Any,
        relics: Vec::new(),
        active: false,
    };
}

impl Default for GearTrigger {
    fn default() -> Self {
        Self::DEFAULT
    }
}
