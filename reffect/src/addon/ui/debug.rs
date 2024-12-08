use super::Addon;
use crate::{
    context::Context,
    internal::Resources,
    internal::{BuffInfoMap, BuffMap, Error, Interface, Internal},
    render::colors::{Colored, GREEN, RED},
};
use nexus::imgui::{StyleColor, Ui, Window};
use std::fmt;

impl Addon {
    pub fn render_debug(&mut self, ui: &Ui) {
        Window::new("Reffect Debug")
            .collapsible(false)
            .always_auto_resize(true)
            .opened(&mut self.debug)
            .build(ui, || {
                let ctx = &self.context;
                let infos = Internal::get_buff_infos().as_ref().ok();

                ui.text(format!("Show elements: {}", ctx.ui.should_show()));

                ui.text("Own weapons:");
                ui.same_line();
                debug_result(ui, ctx.player.info.as_ref(), |info| {
                    for weapon in info.weapons.iter() {
                        ui.text(weapon);
                    }
                });

                ui.text("Own traits:");
                ui.same_line();
                debug_result(ui, ctx.player.info.as_ref(), |info| {
                    let traits = &info.traits;
                    for [a, b, c] in [
                        [traits[0], traits[1], traits[2]],
                        [traits[3], traits[4], traits[5]],
                        [traits[6], traits[7], traits[8]],
                    ] {
                        ui.text(format!("{a: >4} {b: >4} {c: >4}"));
                    }
                });

                ui.text("Own resources:");
                ui.same_line();
                debug_result(ui, ctx.state.own_resources.as_ref(), |resources| {
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

                ui.text("Own buffs:");
                ui.same_line();
                debug_result(ui, ctx.state.own_buffs.as_ref(), |buffs| {
                    buffs_tooltip(ui, ctx, infos, buffs)
                });

                ui.text("Target buffs:");
                ui.same_line();
                debug_result(ui, ctx.state.target_buffs.as_ref(), |buffs| {
                    buffs_tooltip(ui, ctx, infos, buffs)
                });

                for i in 0..4 {
                    ui.text(format!("Group Member {} buffs:", i + 1));
                    ui.same_line();
                    debug_result(
                        ui,
                        ctx.state.group_buffs.as_ref().map(|group| &group[i]),
                        |buffs| buffs_tooltip(ui, ctx, infos, buffs),
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

fn debug_result<T>(ui: &Ui, result: Result<&T, &Error>, tooltip: impl FnOnce(&T)) {
    match result {
        Ok(value) => {
            ui.text_colored(GREEN, "available");
            if ui.is_item_hovered() {
                ui.tooltip(|| tooltip(value));
            }
        }
        Err(err) => {
            ui.text_colored(RED, "unavailable");
            if ui.is_item_hovered() {
                ui.tooltip_text(err.to_string());
            }
        }
    }
}

fn buffs_tooltip(ui: &Ui, ctx: &Context, infos: Option<&BuffInfoMap>, buffs: &BuffMap) {
    for (id, buff) in buffs {
        ui.text(format!("{}x {id}", buff.stacks));
        if let Some(info) = infos.and_then(|infos| infos.get(id)) {
            ui.same_line();
            ui.text(format!("{:?} {:?}", info.category, info.stacking));
        }
        if let Some(remain) = ctx.time_until(buff.runout_time) {
            let full = buff.runout_time - buff.apply_time;
            let progress = remain as f32 / full as f32;
            ui.same_line();
            ui.text(format!(
                "{:.1}/{:.1}s {:.1}%",
                remain as f32 / 1000.0,
                full as f32 / 1000.0,
                progress * 100.0,
            ));
        }
    }
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
