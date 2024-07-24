use super::{Align, Direction, Progress, RenderState, Unit};
use crate::{
    action::Action,
    bounds::Bounds,
    colors::{self, with_alpha_factor},
    component_wise::ComponentWise,
    context::{Context, EditState},
    render_util::{
        enum_combo, helper, helper_slider, input_float_with_format, input_size, input_u32,
        slider_percent, Rect,
    },
    traits::{Render, RenderOptions},
    tree::TreeLeaf,
    trigger::ProgressTrigger,
};
use nexus::imgui::{ColorEdit, ColorPreview, ComboBoxFlags, InputTextFlags, Ui};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Bar {
    #[serde(alias = "buff")]
    pub progress_trigger: ProgressTrigger,

    #[serde(alias = "progress")]
    pub progress_kind: Progress,
    pub max: u32,
    pub cap: f32,

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
    fn cap(&self, value: f32) -> f32 {
        (value / self.cap).min(1.0)
    }
}

impl TreeLeaf for Bar {}

impl Render for Bar {
    fn render(&mut self, ui: &Ui, ctx: &Context, state: &RenderState) {
        if let Some(active) = &self.progress_trigger.active_or_edit(ctx, state) {
            let alpha = ui.clone_style().alpha;

            let (start, end) = self.bounding_box(ui, ctx, state.pos);
            let progress = self.cap(self.progress_kind.calc(ctx, active, self.max));
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
                let tick = self.cap(*tick);
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
    fn render_options(&mut self, ui: &Ui, state: &mut EditState) {
        self.progress_trigger.render_options(ui, state);

        ui.spacing();

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

        slider_percent(ui, "Cap", &mut self.cap);
        helper(ui, || ui.text("Bar cap in percent"));

        enum_combo(ui, "Direction", &mut self.direction, ComboBoxFlags::empty());
        helper(ui, || ui.text("Progress fill direction"));

        enum_combo(ui, "Align", &mut self.align, ComboBoxFlags::empty());

        let [x, y] = &mut self.size;
        input_size(x, y);

        ColorEdit::new("Fill", &mut self.fill)
            .preview(ColorPreview::Alpha)
            .alpha_bar(true)
            .build(ui);

        ColorEdit::new("Background", &mut self.background)
            .preview(ColorPreview::Alpha)
            .alpha_bar(true)
            .build(ui);

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

            ColorEdit::new("Border color", &mut self.border_color)
                .preview(ColorPreview::Alpha)
                .alpha_bar(true)
                .build(ui);
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

        ColorEdit::new("Tick color", &mut self.tick_color)
            .preview(ColorPreview::Alpha)
            .alpha_bar(true)
            .build(ui);

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
            progress_trigger: ProgressTrigger::default(),
            progress_kind: Progress::default(),
            max: 25,
            cap: 1.0,
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
