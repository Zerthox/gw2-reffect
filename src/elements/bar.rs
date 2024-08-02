use super::{Align, Direction, Progress, RenderState, Unit};
use crate::{
    action::Action,
    bounds::Bounds,
    colors::{self, with_alpha_factor},
    component_wise::ComponentWise,
    context::{Context, EditState},
    render_util::{
        enum_combo, helper, helper_slider, input_color_alpha, input_float_with_format,
        input_percent, input_size, input_u32, slider_percent, Rect,
    },
    traits::{Render, RenderOptions},
    tree::TreeLeaf,
};
use nexus::imgui::{ComboBoxFlags, InputTextFlags, Ui};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Bar {
    #[serde(alias = "progress")]
    pub progress_kind: Progress,
    pub max: u32,

    pub lower_bound: f32,
    pub progress_factor: f32,

    pub size: [f32; 2],
    pub align: Align,
    pub direction: Direction,
    pub fill: [f32; 4],
    pub background: [f32; 4],

    pub border: bool,
    pub border_size: f32,
    pub border_color: [f32; 4],

    pub tick_size: f32,
    pub tick_color: [f32; 4],
    pub tick_unit: Unit,
    pub ticks: Vec<f32>,
}

impl Bar {
    fn process_value(&self, value: f32) -> f32 {
        ((value * self.progress_factor) - self.lower_bound).clamp(0.0, 1.0)
    }
}

impl TreeLeaf for Bar {}

impl Render for Bar {
    fn render(&mut self, ui: &Ui, ctx: &Context, state: &RenderState) {
        if let Some(active) = state.trigger_active() {
            let alpha = ui.clone_style().alpha;

            let (start, end) = self.bounding_box(ui, ctx, state.pos);
            let progress = self.process_value(self.progress_kind.calc(ctx, active, self.max));
            let (offset_start, offset_end) =
                self.direction.progress_rect_offset(self.size, progress);
            let fill_start = start.add(offset_start);
            let fill_end = start.add(offset_end);

            let draw_list = ui.get_background_draw_list();
            draw_list
                .add_rect(start, end, with_alpha_factor(self.background, alpha))
                .filled(true)
                .build();
            draw_list
                .add_rect(fill_start, fill_end, with_alpha_factor(self.fill, alpha))
                .filled(true)
                .build();
            if self.border {
                draw_list
                    .add_rect(start, end, with_alpha_factor(self.border_color, alpha))
                    .thickness(self.border_size)
                    .build();
            }

            let end_offset = self.direction.tick_end_offset(self.size);
            let max = active.max();
            for tick in &self.ticks {
                let tick = self.process_value(*tick);
                if let Some(tick_progress) = self.tick_unit.calc_progress(tick, max) {
                    let offset = self
                        .direction
                        .progress_value_offset(self.size, tick_progress);
                    let start = start.add(offset);
                    let end = start.add(end_offset);
                    draw_list
                        .add_line(start, end, with_alpha_factor(self.tick_color, alpha))
                        .thickness(self.tick_size)
                        .build();
                }
            }
        }
    }
}

impl Bounds for Bar {
    fn bounding_box(&self, _ui: &Ui, _ctx: &Context, pos: [f32; 2]) -> Rect {
        let offset = self.align.offset(self.size);
        let start = pos.add(offset);
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

        input_percent("Lower bound", &mut self.lower_bound);
        helper(ui, || ui.text("Lower bound for progress in percent"));

        let mut upper_bound = 1.0 / self.progress_factor; // keep as factor to avoid divisions outside of edit
        if input_percent("Upper bound", &mut upper_bound) {
            self.progress_factor = 1.0 / upper_bound.max(0.0);
        }
        helper(ui, || ui.text("Upper bound for progress in percent"));

        enum_combo(ui, "Direction", &mut self.direction, ComboBoxFlags::empty());
        helper(ui, || ui.text("Progress fill direction"));

        ui.spacing();

        enum_combo(ui, "Align", &mut self.align, ComboBoxFlags::empty());

        input_size(&mut self.size);

        input_color_alpha(ui, "Fill", &mut self.fill);

        input_color_alpha(ui, "Background", &mut self.background);

        ui.checkbox("Border", &mut self.border);
        if self.border {
            if input_float_with_format(
                "Border size",
                &mut self.border_size,
                1.0,
                10.0,
                "%.1f",
                InputTextFlags::empty(),
            ) {
                self.border_size = self.border_size.max(0.0);
            }

            input_color_alpha(ui, "Border color", &mut self.border_color);
        }

        ui.spacing();

        if input_float_with_format(
            "Tick size",
            &mut self.tick_size,
            1.0,
            10.0,
            "%.1f",
            InputTextFlags::empty(),
        ) {
            self.tick_size = self.tick_size.max(0.0);
        }

        input_color_alpha(ui, "Tick color", &mut self.tick_color);

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
}

impl Default for Bar {
    fn default() -> Self {
        Self {
            progress_kind: Progress::default(),
            max: 25,
            lower_bound: 0.0,
            progress_factor: 1.0,
            align: Align::Center,
            size: [128.0, 12.0],
            direction: Direction::Right,
            fill: colors::GREEN,
            background: colors::TRANSPARENT,
            border: true,
            border_size: 1.0,
            border_color: colors::BLACK,
            tick_size: 1.0,
            tick_color: colors::BLACK,
            tick_unit: Unit::default(),
            ticks: Vec::new(),
        }
    }
}
