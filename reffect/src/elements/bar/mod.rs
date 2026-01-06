mod progress;
mod props;

pub use self::{progress::*, props::*};

use super::{Direction, Props, RenderCtx, Unit, align::Align};
use crate::{
    action::Action,
    colors::with_alpha_factor,
    context::Context,
    elements::Common,
    render::{
        Bounds, ComponentWise, Rect, enum_combo, helper, helper_slider, input_color_alpha,
        input_float_with_format, input_percent, input_positive_with_format, input_size,
        slider_percent,
    },
    tree::TreeNode,
    trigger::ProgressActive,
};
use const_default::ConstDefault;
use nexus::imgui::{ComboBoxFlags, InputTextFlags, Ui};
use serde::{Deserialize, Serialize};

// TODO: rounding

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct Bar {
    /// Progress used by bar.
    #[serde(alias = "progress")]
    pub progress_kind: Progress,

    /// Maximum progress.
    pub max: f32,

    #[serde(flatten)]
    pub props: Props<BarProps>,

    /// Bar size.
    pub size: [f32; 2],

    /// Bar alignment.
    pub align: Align,

    /// Fill direction.
    pub direction: Direction,

    /// Unit for ticks.
    pub tick_unit: Unit,

    /// Tick placements.
    pub ticks: Vec<f32>,
}

impl Bar {
    fn calc_progress(&self, ctx: &Context, active: &ProgressActive) -> f32 {
        let progress = self.progress_kind.calc_progress(ctx, active, self.max);
        self.process_value(progress).clamp(0.0, 1.0)
    }

    fn process_value(&self, value: f32) -> f32 {
        let scale = self.props.upper_bound - self.props.lower_bound;
        (value - self.props.lower_bound) / scale
    }

    pub fn render(&mut self, ui: &Ui, ctx: &RenderCtx, common: &Common) {
        let active = common.trigger.active();
        self.props.update(ctx, active);

        if let Some(active) = active {
            let progress = self.calc_progress(ctx, active);

            let alpha = ui.clone_style().alpha;
            let (start, end) = self.bounds_with_offset(ui, ctx, ctx.pos());
            let offset_2d = self.direction.offset_2d(self.size);

            let (offset_start, offset_end) =
                self.direction.progress_rect_offset(self.size, progress);
            let fill_start = start.add(offset_start);
            let fill_end = start.add(offset_end);

            let bg_color = with_alpha_factor(self.props.background, alpha);
            let fill_color = with_alpha_factor(self.props.fill, alpha);

            let draw_list = ui.get_background_draw_list();
            if start != fill_start {
                draw_list
                    .add_rect(start, fill_start.add(offset_2d), bg_color)
                    .filled(true)
                    .build();
            }
            if end != fill_end.add(offset_2d) {
                draw_list
                    .add_rect(fill_end, end, bg_color)
                    .filled(true)
                    .build();
            }
            draw_list
                .add_rect(fill_start, fill_end.add(offset_2d), fill_color)
                .filled(true)
                .build();

            if self.props.border_size > 0.0 {
                let border_color = with_alpha_factor(self.props.border_color, alpha);
                draw_list
                    .add_rect(start, end, border_color)
                    .thickness(self.props.border_size)
                    .build();
            }

            if self.props.tick_size > 0.0 {
                let max = self.progress_kind.progress_max(active, self.max);
                for tick in &self.ticks {
                    if let Some(progress) = self
                        .tick_unit
                        .calc_progress(*tick, max)
                        .map(|value| self.process_value(value))
                        .filter(|value| *value > 0.0 && *value < 1.0)
                    {
                        // TODO: multiple ticks for horizontal/vertical?
                        let offset = self.direction.progress_value_offset(self.size, progress);
                        let start = start.add(offset);
                        let end = start.add(offset_2d);
                        let color = with_alpha_factor(self.props.tick_color, alpha);
                        draw_list
                            .add_line(start, end, color)
                            .thickness(self.props.tick_size)
                            .build();
                    }
                }
            }
        }
    }

    pub fn render_options(&mut self, ui: &Ui, _ctx: &Context) {
        enum_combo(
            ui,
            "Progress",
            &mut self.progress_kind,
            ComboBoxFlags::empty(),
        );

        if let Progress::Intensity = self.progress_kind {
            input_positive_with_format(
                "Max",
                &mut self.max,
                1.0,
                10.0,
                "%.1f",
                InputTextFlags::empty(),
            );
            helper(ui, || ui.text("Maximum progress value"));
        }

        input_percent("Lower bound", &mut self.props.base.lower_bound);
        helper(ui, || ui.text("Lower bound for progress in percent"));

        input_percent("Upper bound", &mut self.props.base.upper_bound);
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

    pub fn render_tabs(&mut self, ui: &Ui, ctx: &Context, common: &Common) {
        if let Some(_token) = ui.tab_item("Condition") {
            self.props
                .render_condition_options(ui, ctx, &common.trigger.source);
        }
    }

    pub fn render_debug(&mut self, ui: &Ui, _ctx: &Context) {
        ui.text(format!(
            "Progress scale: {}",
            self.props.upper_bound - self.props.lower_bound
        ));
    }
}

impl TreeNode for Bar {}

impl Bounds for Bar {
    fn bounds(&self, _ui: &Ui, _ctx: &Context) -> Rect {
        let start = self.align.offset(self.size);
        (start, start.add(self.size))
    }
}

impl ConstDefault for Bar {
    const DEFAULT: Self = Self {
        progress_kind: Progress::DEFAULT,
        max: 25.0,
        props: Props::DEFAULT,
        align: Align::Center,
        size: [128.0, 12.0],
        direction: Direction::Right,
        tick_unit: Unit::DEFAULT,
        ticks: Vec::new(),
    };
}

impl Default for Bar {
    fn default() -> Self {
        Self::DEFAULT
    }
}
