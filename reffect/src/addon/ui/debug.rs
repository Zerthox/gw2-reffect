use super::Addon;
use crate::{
    clipboard::Clipboard,
    colors::{self, Colored},
    context::{BuffMap, Context, SkillInfo, Slot},
    error::Error,
    internal::{Interface, Internal},
};
use nexus::imgui::{StyleColor, TreeNode, TreeNodeFlags, Ui, Window};
use reffect_core::context::{PlayerResources, Skillbar};
use std::{cmp::Ordering, fmt};
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

                debug_result_tree(
                    ui,
                    "plweap",
                    "Player weapons",
                    &ctx.player.weapons,
                    |weapons| {
                        for weapon in weapons.iter() {
                            ui.text(weapon);
                        }
                    },
                );
                debug_result_tree(
                    ui,
                    "pltrait",
                    "Player traits",
                    &ctx.player.traits,
                    |traits| {
                        for [adept, master, grandmaster] in [
                            [traits[0], traits[1], traits[2]],
                            [traits[3], traits[4], traits[5]],
                            [traits[6], traits[7], traits[8]],
                        ] {
                            ui.text(format!("{adept: >4} {master: >4} {grandmaster: >4}"));
                        }
                    },
                );
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
                    "tgres",
                    "Target resources",
                    &ctx.target.resources,
                    |resources| {
                        ui.text(format!("Health: {:.2}", 100.0 * resources.health));
                        ui.text(format!("Barrier: {:.2}", 100.0 * resources.barrier));

                        ui.text("Defiance:");
                        ui.same_line();
                        match &resources.defiance {
                            Some(defiance) => ui.text(format!("{:.2}", 100.0 * defiance)),
                            None => ui.text("-"),
                        }
                    },
                );
                debug_result_tree(ui, "tgbuff", "Target buffs", &ctx.target.buffs, |buffs| {
                    debug_buffs(ui, ctx, buffs)
                });

                debug_result_tree(ui, "grp", "Group", &ctx.group, |group| {
                    ui.text(format!("Group Type: {}", group.group_type));

                    for (i, member) in group.members.iter().enumerate() {
                        ui.text(format!("Group Member {}:", i + 1));
                        ui.same_line();
                        if let Some(acc) = &member.account {
                            ui.text(acc.strip_prefix(':').unwrap_or(acc));
                        } else {
                            ui.text("-");
                        }
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

fn debug_player_resources(ui: &Ui, resources: &PlayerResources) {
    let PlayerResources {
        health,
        barrier,
        defiance,
        endurance,
        primary,
        secondary,
    } = resources;

    ui.text(format!("Health: {}/{}", health.current, health.max));
    ui.text(format!("Barrier: {}/{}", barrier.current, barrier.max));

    ui.text("Defiance:");
    ui.same_line();
    match defiance {
        Some(defiance) => ui.text(format!("{defiance:.2}")),
        None => ui.text("-"),
    }

    ui.text(format!(
        "Endurance: {}/{}",
        endurance.current, endurance.max
    ));

    ui.text(format!("Primary: {}/{}", primary.current, primary.max));
    ui.text(format!(
        "Secondary: {}/{}",
        secondary.current, secondary.max
    ));
}

fn debug_buffs(ui: &Ui, ctx: &Context, buffs: &BuffMap) {
    for (id, buff) in buffs {
        ui.text(format!("{:>2}x {id:>5}", buff.stacks));
        if let Ok(SkillInfo::Buff { category, stacking }) = Internal::get_skill_info(*id) {
            ui.same_line();
            ui.text(format!("{category} {stacking}"));
        }
        if !buff.is_infinite() {
            ui.same_line();
            ui.text(format!(
                "{:.1}/{:.1}s {:.1}%",
                to_secs(buff.remaining(ctx.now)),
                to_secs(buff.duration()),
                100.0 * buff.progress(ctx.now),
            ));
        }
    }
}

fn debug_skillbar(ui: &Ui, ctx: &Context, skillbar: &Skillbar) {
    ui.text(format!("Bundle: {}", skillbar.has_bundle));

    for slot in Slot::iter() {
        ui.text(format!("{slot:<14} ="));
        if let Some(ability) = skillbar.slot(slot) {
            let color = if !ability.is_available {
                Some(ui.push_style_color(StyleColor::Text, colors::RED))
            } else if ability.is_pressed {
                Some(ui.push_style_color(StyleColor::Text, colors::BLUE))
            } else if ability.is_pending {
                Some(ui.push_style_color(StyleColor::Text, colors::YELLOW))
            } else {
                None
            };

            ui.same_line();
            ui.text(format!("{}x {:>5}", ability.ammo, ability.id));
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
