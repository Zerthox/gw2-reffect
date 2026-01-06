use crate::{
    action::Action,
    context::{Context, Update, Weapon},
    internal::{Interface, Internal},
    render::{Validation, enum_combo_bitflags, helper, input_item_id},
    serde::bitflags,
    trigger::{MemoizedTrigger, TriggerMode},
};
use const_default::ConstDefault;
use enumflags2::BitFlags;
use nexus::imgui::{ComboBoxFlags, InputTextFlags, Ui};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct GearTrigger {
    /// Equipped weapons.
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
    pub fn update_full(&mut self, ctx: &Context) {
        if ctx.map.is_valid() {
            for item in self.sigils.iter_mut().chain(&mut self.relics) {
                item.update();
            }
        }
        self.update(ctx);
    }

    pub fn weapons_active(&self, ctx: &Context) -> bool {
        let gear = ctx.player.gear.as_ref();
        self.weapon_mode
            .check_flags_optional(self.weapons, gear.map(|gear| gear.weapons).ok())
    }

    pub fn sigils_active(&self, ctx: &Context) -> bool {
        if let Ok(gear) = ctx.player.gear.as_ref() {
            self.sigil_mode
                .check_slice(&self.sigils, |sigil| gear.sigils.contains(&sigil.buff))
        } else {
            true
        }
    }

    pub fn relic_active(&self, ctx: &Context) -> bool {
        if let Ok(gear) = ctx.player.gear.as_ref() {
            TriggerMode::Any.check_slice(&self.relics, |relic| gear.relic == relic.buff)
        } else {
            true
        }
    }

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

impl MemoizedTrigger for GearTrigger {
    fn resolve_active(&mut self, ctx: &Context) -> bool {
        self.weapons_active(ctx) && self.sigils_active(ctx) && self.relic_active(ctx)
    }

    fn memoized_state(&mut self) -> &mut bool {
        &mut self.active
    }

    fn needs_update(&self, ctx: &Context) -> bool {
        ctx.has_update(Update::Gear)
    }
}

impl ConstDefault for GearTrigger {
    const DEFAULT: Self = Self {
        weapons: BitFlags::EMPTY,
        weapon_mode: TriggerMode::Any,
        sigils: Vec::new(),
        sigil_mode: TriggerMode::Any,
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
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(transparent)]
pub struct Item {
    /// Item id.
    pub item: u32,

    /// Hidden buff id.
    #[serde(skip)]
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

    pub fn validate(&self) -> Validation<String> {
        let Self { item, .. } = *self;
        if let Ok(info) = Internal::get_item_info(item)
            && let Some(buff) = info.buff()
        {
            Validation::Confirm(format!(
                "{} {item} corresponds to hidden effect {buff}",
                info.as_ref()
            ))
        } else {
            Validation::Error(format!("Item {item} is invalid"))
        }
    }
}
