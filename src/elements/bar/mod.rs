mod progress;
mod props;

pub use self::{progress::*, props::*};

use super::{align::Align, Direction, Props, RenderState, Unit};
use crate::{
    action::Action,
    context::{Context, EditState},
    render::{
        colors::with_alpha_factor, Bounds, ComponentWise, Render, RenderDebug, RenderOptions,
    },
    render_util::{
        enum_combo, helper, helper_slider, input_color_alpha, input_float_with_format,
        input_percent, input_percent_inverse, input_positive_with_format, input_size, input_u32,
        slider_percent, Rect,
    },
    tree::TreeNode,
};
use nexus::imgui::{ComboBoxFlags, InputTextFlags, Ui};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Bar {
    #[serde(alias = "progress")]
    pub progress_kind: Progress,
    pub max: u32,

    #[serde(flatten)]
    pub props: Props<BarProps>,

    pub size: [f32; 2],
    pub align: Align,
    pub direction: Direction,

    pub tick_unit: Unit,
    pub ticks: Vec<f32>,
}

impl Bar {
    fn process_value(&self, value: f32) -> f32 {
        ((value * self.props.progress_factor) - self.props.lower_bound).clamp(0.0, 1.0)
    }
}

impl TreeNode for Bar {}

impl Render for Bar {
    fn render(&mut self, ui: &Ui, ctx: &Context, state: &RenderState) {
        let active = state.trigger_active();
        self.props.update(ctx, active);

        if let Some(active) = active {
            let alpha = ui.clone_style().alpha;

            let (start, end) = self.bounds_with_offset(ui, ctx, state.pos);
            let progress =
                self.process_value(self.progress_kind.calc_progress(ctx, active, self.max));
            let (offset_start, offset_end) =
                self.direction.progress_rect_offset(self.size, progress);
            let fill_start = start.add(offset_start);
            let fill_end = start.add(offset_end);

            let draw_list = ui.get_background_draw_list();
            draw_list
                .add_rect(start, end, with_alpha_factor(self.props.background, alpha))
                .filled(true)
                .build();
            draw_list
                .add_rect(
                    fill_start,
                    fill_end,
                    with_alpha_factor(self.props.fill, alpha),
                )
                .filled(true)
                .build();

            if self.props.border_size > 0.0 {
                draw_list
                    .add_rect(
                        start,
                        end,
                        with_alpha_factor(self.props.border_color, alpha),
                    )
                    .thickness(self.props.border_size)
                    .build();
            }

            if self.props.tick_size > 0.0 {
                let end_offset = self.direction.tick_end_offset(self.size);
                let max = self.progress_kind.progress_max(active, self.max);
                for tick in &self.ticks {
                    if let Some(progress) = self.tick_unit.calc_progress(*tick, max) {
                        let progress = self.process_value(progress);
                        let offset = self.direction.progress_value_offset(self.size, progress);
                        let start = start.add(offset);
                        let end = start.add(end_offset);
                        draw_list
                            .add_line(start, end, with_alpha_factor(self.props.tick_color, alpha))
                            .thickness(self.props.tick_size)
                            .build();
                    }
                }
            }
        }
    }
}

impl Bounds for Bar {
    fn bounds(&self, _ui: &Ui, _ctx: &Context) -> Rect {
        let start = self.align.offset(self.size);
        (start, start.add(self.size))
    }
}

impl RenderOptions for Bar {
    fn render_options(&mut self, ui: &Ui, _state: &mut EditState) {
        enum_combo(
            ui,
            "Progress",
            &mut self.progress_kind,
            ComboBoxFlags::empty(),
        );

        if let Progress::Intensity = self.progress_kind {
            input_u32(ui, "Max", &mut self.max, 1, 10);
            helper(ui, || ui.text("Maximum progress value"));
        }

        input_percent("Lower bound", &mut self.props.base.lower_bound);
        helper(ui, || ui.text("Lower bound for progress in percent"));

        input_percent_inverse("Upper bound", &mut self.props.base.progress_factor);
        helper(ui, || ui.text("Upper bound for progress in percent"));

        enum_combo(ui, "Direction", &mut self.direction, ComboBoxFlags::empty());
        helper(ui, || ui.text("Progress fill direction"));

        ui.spacing();

        enum_combo(ui, "Align", &mut self.align, ComboBoxFlags::empty());

        input_size(&mut self.size);

        input_color_alpha(ui, "Fill", &mut self.props.base.fill);

        input_color_alpha(ui, "Background", &mut self.props.base.background);

        input_positive_with_format(
            "Border size",
            &mut self.props.base.border_size,
            1.0,
            10.0,
            "%.1f",
            InputTextFlags::empty(),
        );
        input_color_alpha(ui, "Border color", &mut self.props.base.border_color);

        ui.spacing();

        input_positive_with_format(
            "Tick size",
            &mut self.props.base.tick_size,
            1.0,
            10.0,
            "%.1f",
            InputTextFlags::empty(),
        );

        input_color_alpha(ui, "Tick color", &mut self.props.base.tick_color);

        if let Some(prev) = enum_combo(ui, "Tick unit", &mut self.tick_unit, ComboBoxFlags::empty())
        {
            let new = self.tick_unit;
            for tick in &mut self.ticks {
                match (prev, new) {
                    (Unit::Percent, Unit::Absolute) => *tick *= 100.0,
                    (Unit::Absolute, Unit::Percent) => *tick /= 100.0,
                    _ => {}
                }
            }
        }

        let mut action = Action::new();
        for (i, tick) in self.ticks.iter_mut().enumerate() {
            let _id = ui.push_id(i as i32);
            action.input_with_buttons(ui, i, || match self.tick_unit {
                Unit::Absolute => input_float_with_format(
                    "##tick",
                    tick,
                    0.0,
                    0.0,
                    "%.1f",
                    InputTextFlags::empty(),
                ),

                Unit::Percent => slider_percent(ui, "##tick", tick),
            });
            ui.same_line();
            ui.text(format!("Tick {}", i + 1));
            if self.tick_unit == Unit::Percent && i == 0 {
                helper_slider(ui);
            }
        }
        action.perform(&mut self.ticks);
        if ui.button("Add Tick") {
            self.ticks.push(match self.tick_unit {
                Unit::Percent => 0.5,
                Unit::Absolute => 1.0,
            });
        }
    }

    fn render_tabs(&mut self, ui: &Ui, state: &mut EditState) {
        if let Some(_token) = ui.tab_item("Condition") {
            self.props.render_condition_options(ui, state);
        }
    }
}

impl RenderDebug for Bar {
    fn render_debug(&mut self, ui: &Ui) {
        ui.text(format!(
            "Progress factor: {}",
            self.props.base.progress_factor
        ));
    }
}

impl Default for Bar {
    fn default() -> Self {
        Self {
            progress_kind: Progress::default(),
            max: 25,
            props: Props::default(),
            align: Align::Center,
            size: [128.0, 12.0],
            direction: Direction::Right,
            tick_unit: Unit::default(),
            ticks: Vec::new(),
        }
    }
}
