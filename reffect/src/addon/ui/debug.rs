use super::Addon;
use crate::{
    clipboard::Clipboard,
    colors::{self, Colored},
    context::{
        AbilityState, BuffMap, Build, CombatantResources, Context, Defiance, Gear, PlayerResources,
        SkillInfo, Skillbar, Slot,
    },
    error::Error,
    internal::{Interface, Internal},
};
use nexus::imgui::{StyleColor, TreeNode, TreeNodeFlags, Ui, Window};
use std::{
    cmp::Ordering,
    fmt::{self, Write},
};
use strum::IntoEnumIterator;

impl Addon {
    pub fn render_debug(&mut self, ui: &Ui, ctx: &Context) {
        Window::new("Reffect Debug")
            .collapsible(false)
            .always_auto_resize(true)
            .opened(&mut self.debug)
            .build(ui, || {
                ui.text(format!("Show elements: {}", ctx.ui.should_show()));

                ui.text(format!("Combat: {}", ctx.ui.combat));

                ui.text("Profession:");
                ui.same_line();
                name_or_unknown_id_colored(ui, ctx.player.prof);

                ui.text("Specialization:");
                ui.same_line();
                name_or_unknown_id_colored(ui, ctx.player.spec);

                ui.text("Race:");
                ui.same_line();
                ui.text(match ctx.player.race {
                    Ok(value) => value.to_string(),
                    Err(id) => format!("Unknown ({id})"),
                });

                ui.text("Mount:");
                ui.same_line();
                name_or_unknown_id_colored(ui, ctx.player.mount);

                ui.text(format!("Map id: {}", ctx.map.id));
                ui.text(format!("Map category: {}", ctx.map.category));

                debug_result_tree(ui, "plgear", "Player gear", &ctx.player.gear, |gear| {
                    let Gear {
                        weapons,
                        sigils,
                        relic,
                    } = gear;

                    ui.text("Sigils:");
                    for sigil in sigils {
                        ui.same_line();
                        ui.text(format!("{sigil: >5}"));
                    }
                    ui.text(format!("Relic: {relic: >5}"));

                    ui.text("Weapons:");
                    ui.indent();
                    for weapon in weapons.iter() {
                        ui.text(weapon);
                    }
                    ui.unindent();
                });
                debug_result_tree(ui, "plbuild", "Player build", &ctx.player.build, |build| {
                    let Build {
                        specs,
                        traits,
                        skill_selections,
                        prof_selections,
                    } = build;

                    let [spec1, spec2, spec3] = specs;
                    ui.text(format!("Specs: {spec1: >2} {spec2: >2} {spec3: >2}"));

                    ui.text("Traits:");
                    ui.indent();
                    for [adept, master, grandmaster] in [
                        [traits[0], traits[1], traits[2]],
                        [traits[3], traits[4], traits[5]],
                        [traits[6], traits[7], traits[8]],
                    ] {
                        ui.text(format!("{adept: >4} {master: >4} {grandmaster: >4}"));
                    }
                    ui.unindent();

                    ui.text("Skill selections:");
                    if !skill_selections.is_empty() {
                        ui.indent();
                        let mut text = String::new();
                        for (i, skill) in skill_selections.iter().enumerate() {
                            if i != 0 && i.is_multiple_of(3) {
                                text.push('\n');
                            }
                            let _ = write!(&mut text, "{skill: >5} ");
                        }
                        ui.text(text);
                        ui.unindent();
                    }

                    ui.text("Profession selections:");
                    ui.indent();
                    for info in prof_selections.iter() {
                        let _color = info
                            .colored()
                            .map(|color| ui.push_style_color(StyleColor::Text, color));
                        ui.text(info);
                        ui.same_line();
                    }
                    ui.new_line();
                    ui.unindent();
                });
                debug_result_tree(
                    ui,
                    "plres",
                    "Player resources",
                    &ctx.player.resources,
                    |resources| debug_player_resources(ui, resources),
                );
                debug_result_tree(
                    ui,
                    "plbuffs",
                    "Player buffs",
                    &ctx.player.buff_info,
                    |buff_info| {
                        ui.text(format!(
                            "Last screen border: {}",
                            buff_info.last_screen_border
                        ));
                        ui.text(format!(
                            "Last squad highlight: {}",
                            buff_info.last_squad_highlight
                        ));

                        ui.spacing();
                        debug_buffs(ui, ctx, &buff_info.buffs)
                    },
                );
                debug_result_tree(
                    ui,
                    "plskills",
                    "Player skillbar",
                    &ctx.player.skillbar,
                    |skillbar| debug_skillbar(ui, ctx, skillbar),
                );
                debug_result_tree(
                    ui,
                    "ptres",
                    "Pet resources",
                    &ctx.player.resources,
                    |resources| {
                        if let Some(pet) = &resources.pet {
                            debug_combatant_resources(ui, pet, false);
                        } else {
                            ui.text("No pet");
                        }
                    },
                );

                debug_result_tree(
                    ui,
                    "tgres",
                    "Target resources",
                    &ctx.target.resources,
                    |resources| debug_combatant_resources(ui, resources, true),
                );
                debug_result_tree(ui, "tgbuff", "Target buffs", &ctx.target.buffs, |buffs| {
                    debug_buffs(ui, ctx, buffs)
                });

                debug_result_tree(ui, "grp", "Group", &ctx.group, |group| {
                    ui.text(format!("Group Type: {}", group.group_type));

                    for (i, member) in group.members.iter().enumerate() {
                        let label = format!(
                            "Member {}: {}",
                            i + 1,
                            if let Some(acc) = &member.account {
                                acc.strip_prefix(':').unwrap_or(acc)
                            } else {
                                "-"
                            }
                        );
                        TreeNode::new(i.to_string())
                            .label::<String, _>(label)
                            .flags(TreeNodeFlags::SPAN_AVAIL_WIDTH)
                            .build(ui, || {
                                debug_result_tree(
                                    ui,
                                    "res",
                                    "Resources",
                                    &member.resources,
                                    |resources| debug_combatant_resources(ui, resources, true),
                                );
                                debug_result_tree(ui, "buffs", "Buffs", &member.buffs, |buffs| {
                                    debug_buffs(ui, ctx, buffs)
                                });
                            });
                    }
                });

                ui.spacing();
                ui.separator();
                ui.spacing();

                ctx.edit.debug(ui);
                Clipboard::debug(ui);
            });
    }
}

fn debug_combatant_resources(ui: &Ui, resources: &CombatantResources, normalized: bool) {
    let CombatantResources {
        health,
        barrier,
        defiance,
    } = resources;

    let precision = if normalized { 1 } else { 0 };
    ui.text(format!("Health: {health:.*}", precision));
    ui.text(format!("Barrier: {barrier:.*}", precision));

    ui.text("Defiance:");
    ui.same_line();
    match defiance {
        Defiance::None => ui.text("-"),
        Defiance::Immune => ui.text("immune"),
        Defiance::Active(percent) => ui.text(format!("active {percent:.1}%")),
        Defiance::Recover(percent) => ui.text(format!("recover {percent:.1}%")),
    }
}

fn debug_player_resources(ui: &Ui, resources: &PlayerResources) {
    let PlayerResources {
        combatant,
        health_reduction,
        endurance,
        primary,
        secondary,
        pet: _,
    } = resources;

    debug_combatant_resources(ui, combatant, false);

    ui.text(format!("Health reduction: {health_reduction}"));
    ui.text(format!("Endurance: {endurance}",));
    ui.text(format!("Primary: {primary}"));
    ui.text(format!("Secondary: {secondary}"));
}

fn debug_buffs(ui: &Ui, ctx: &Context, buffs: &BuffMap) {
    for (id, buff) in buffs {
        let id = *id;
        ui.group(|| {
            let category = match Internal::get_skill_info(id) {
                Ok(SkillInfo::Buff { category, .. }) => category.to_string(),
                _ => "Invalid".into(),
            };

            ui.text(format!("{:>2}x {id:>5} {category}", buff.stacks));

            if !buff.is_infinite() {
                ui.same_line();
                ui.text(format!(
                    "{:.1}/{:.1}s {:.1}%",
                    to_secs(buff.remaining(ctx.now)),
                    to_secs(buff.duration()),
                    100.0 * buff.progress(ctx.now),
                ));
            }
        });

        if ui.is_item_hovered()
            && let Ok(SkillInfo::Buff {
                stacking,
                visibility,
                ..
            }) = Internal::get_skill_info(id)
        {
            ui.tooltip_text(format!("Stacking {stacking}\nVisible for {visibility}"));
        }
    }
}

fn debug_skillbar(ui: &Ui, ctx: &Context, skillbar: &Skillbar) {
    for slot in Slot::iter() {
        ui.text(format!("{slot:<14} ="));
        if let Some(ability) = skillbar.slot(slot) {
            let color = if ability.state.contains(AbilityState::Pressed) {
                Some(ui.push_style_color(StyleColor::Text, colors::BLUE))
            } else if ability.state.contains(AbilityState::Pending) {
                Some(ui.push_style_color(StyleColor::Text, colors::YELLOW))
            } else {
                None
            };

            ui.same_line();
            ui.text(format!("{}x {}", ability.ammo, ability.id));
            drop(color);

            let _color = match ability.recharge_rate.total_cmp(&1.0) {
                Ordering::Less => Some(ui.push_style_color(StyleColor::Text, colors::BLUE)),
                Ordering::Equal => None,
                Ordering::Greater => Some(ui.push_style_color(StyleColor::Text, colors::GREEN)),
            };
            let recharge = ability.recharge_remaining(ctx.now);
            if recharge > 0 {
                ui.same_line();
                ui.text(format!(
                    "{:.1}/{:.1}s {:.1}%",
                    to_secs(recharge),
                    to_secs(ability.recharge),
                    100.0 * ability.recharge_progress(ctx.now)
                ));
            }

            let ammo_recharge = ability.ammo_recharge_remaining(ctx.now);
            if ammo_recharge > 0 {
                ui.same_line();
                ui.text(format!(
                    "Ammo {:.1}/{:.1}s {:.1}%",
                    to_secs(ammo_recharge),
                    to_secs(ability.ammo_recharge),
                    100.0 * ability.ammo_recharge_progress(ctx.now)
                ));
            }
        }
    }
}

fn debug_result_tree<T>(
    ui: &Ui,
    id: impl AsRef<str>,
    label: impl AsRef<str>,
    value: &Result<T, Error>,
    body: impl FnOnce(&T),
) {
    TreeNode::new(id)
        .label::<&str, _>(label.as_ref())
        .flags(TreeNodeFlags::SPAN_AVAIL_WIDTH)
        .build(ui, || match value {
            Ok(value) => {
                body(value);
            }
            Err(err) => ui.text_colored(colors::RED, format!("Error: {err}")),
        });
}

fn name_or_unknown_id_colored<T, N>(ui: &Ui, value: Result<T, N>)
where
    T: AsRef<str> + Colored,
    N: fmt::Display,
{
    match value {
        Ok(value) => {
            let _color = value
                .colored()
                .map(|color| ui.push_style_color(StyleColor::Text, color));
            ui.text(value);
        }
        Err(id) => ui.text(format!("Unknown ({id})")),
    }
}

fn to_secs(millisecs: u32) -> f32 {
    millisecs as f32 / 1000.0
}
