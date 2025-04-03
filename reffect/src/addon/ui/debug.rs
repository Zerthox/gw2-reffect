use super::Addon;
use crate::{
    context::Context,
    internal::Resources,
    internal::{BuffMap, Error, Interface, Internal},
    render::colors::{self, Colored},
};
use nexus::imgui::{StyleColor, Ui, Window};
use reffect_internal::{SkillInfo, Slot, State};
use std::fmt;
use strum::IntoEnumIterator;

impl Addon {
    pub fn render_debug(&mut self, ui: &Ui) {
        Window::new("Reffect Debug")
            .collapsible(false)
            .always_auto_resize(true)
            .opened(&mut self.debug)
            .build(ui, || {
                let ctx = &self.context;

                ui.text(format!("Show elements: {}", ctx.ui.should_show()));

                ui.text("Own weapons:");
                ui.same_line();
                debug_result_tooltip(ui, ctx.player.info.as_ref(), |info| {
                    for weapon in info.weapons.iter() {
                        ui.text(weapon);
                    }
                });

                ui.text("Own traits:");
                ui.same_line();
                debug_result_tooltip(ui, ctx.player.info.as_ref(), |info| {
                    let traits = &info.traits;
                    for [a, b, c] in [
                        [traits[0], traits[1], traits[2]],
                        [traits[3], traits[4], traits[5]],
                        [traits[6], traits[7], traits[8]],
                    ] {
                        ui.text(format!("{a: >4} {b: >4} {c: >4}"));
                    }
                });

                let State {
                    own_resources,
                    own_skillbar,
                    own_buffs,
                    target_buffs,
                    group_buffs,
                } = &ctx.state;

                ui.text("Own resources:");
                ui.same_line();
                debug_result_tooltip(ui, own_resources.as_ref(), |resources| {
                    let Resources {
                        health,
                        barrier,
                        endurance,
                        primary,
                        secondary,
                    } = resources;
                    ui.text(format!("Health: {}/{}", health.current, health.max));
                    ui.text(format!("Barrier: {}/{}", barrier.current, barrier.max));
                    ui.text(format!(
                        "Endurance: {}/{}",
                        endurance.current, endurance.max
                    ));
                    ui.text(format!("Primary: {}/{}", primary.current, primary.max));
                    ui.text(format!(
                        "Secondary: {}/{}",
                        secondary.current, secondary.max
                    ));
                });

                ui.text("Own skillbar:");
                ui.same_line();
                debug_result_tooltip(ui, own_skillbar.as_ref(), |skillbar| {
                    let now = ctx.now;
                    ui.text(format!("Bundle: {}", skillbar.has_bundle));
                    for slot in Slot::iter() {
                        if let Some(ability) = skillbar.slot(slot) {
                            ui.text(format!("{slot:<14} = {}x {:>5}", ability.ammo, ability.id));

                            let recharge = ability.recharge_remaining(now);
                            if recharge > 0 {
                                ui.same_line();
                                ui.text(format!(
                                    "{:.1}/{:.1}s {:.1}%",
                                    to_secs(recharge),
                                    to_secs(ability.recharge),
                                    100.0 * ability.recharge_progress(now)
                                ));
                            }

                            let ammo_recharge = ability.ammo_recharge_remaining(now);
                            if ammo_recharge > 0 {
                                ui.same_line();
                                ui.text(format!(
                                    "Ammo {:.1}/{:.1}s {:.1}%",
                                    to_secs(ammo_recharge),
                                    to_secs(ability.ammo_recharge),
                                    100.0 * ability.ammo_recharge_progress(now)
                                ));
                            }
                        }
                    }
                });

                ui.text("Own buffs:");
                ui.same_line();
                debug_result_tooltip(ui, own_buffs.as_ref(), |buffs| {
                    buffs_tooltip(ui, ctx, buffs)
                });

                ui.text("Last screen border:");
                ui.same_line();
                debug_result(ui, ctx.player.info.as_ref(), |info| {
                    ui.text(info.last_screen_border.to_string());
                });

                ui.text("Last squad highlight:");
                ui.same_line();
                debug_result(ui, ctx.player.info.as_ref(), |info| {
                    ui.text(info.last_squad_highlight.to_string());
                });

                ui.text("Target buffs:");
                ui.same_line();
                debug_result_tooltip(ui, target_buffs.as_ref(), |buffs| {
                    buffs_tooltip(ui, ctx, buffs)
                });

                for i in 0..4 {
                    ui.text(format!("Group Member {} buffs:", i + 1));
                    ui.same_line();
                    debug_result_tooltip(
                        ui,
                        group_buffs.as_ref().map(|group| &group[i]),
                        |buffs| {
                            if let Some(buffs) = buffs {
                                buffs_tooltip(ui, ctx, buffs)
                            }
                        },
                    );
                }

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

                ui.spacing();
                ui.separator();
                ui.spacing();

                self.context.edit.debug(ui);
            });
    }
}

fn debug_result<T>(ui: &Ui, result: Result<&T, &Error>, body: impl FnOnce(&T)) {
    match result {
        Ok(value) => body(value),
        Err(err) => {
            ui.text_colored(colors::RED, "unavailable");
            if ui.is_item_hovered() {
                ui.tooltip_text(err.to_string());
            }
        }
    }
}

fn debug_result_tooltip<T>(ui: &Ui, result: Result<&T, &Error>, tooltip: impl FnOnce(&T)) {
    debug_result(ui, result, |value| {
        ui.text_colored(colors::GREEN, "available");
        if ui.is_item_hovered() {
            ui.tooltip(|| tooltip(value));
        }
    })
}

fn buffs_tooltip(ui: &Ui, ctx: &Context, buffs: &BuffMap) {
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

fn to_secs(millisecs: u32) -> f32 {
    millisecs as f32 / 1000.0
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
