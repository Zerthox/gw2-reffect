use super::{Align, Direction, Progress, RenderState};
use crate::{
    bounds::Bounds,
    colors::{self, with_alpha_factor},
    component_wise::ComponentWise,
    context::Context,
    render_util::{enum_combo, helper, input_float_with_format, input_size, input_u32, Rect},
    traits::{Render, RenderOptions},
    tree::TreeLeaf,
    trigger::ProgressTrigger,
};
use nexus::imgui::{ColorEdit, ColorPreview, ComboBoxFlags, InputTextFlags, Ui};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Bar {
    pub buff: ProgressTrigger,
    pub progress: Progress,
    pub max: u32,

    pub size: [f32; 2],
    pub align: Align,
    pub direction: Direction,
    pub fill: [f32; 4],
    pub background: [f32; 4],

    pub border: bool,
    pub border_size: f32,
    pub border_color: [f32; 4],
}

impl TreeLeaf for Bar {}

impl Render for Bar {
    fn render(&mut self, ui: &Ui, ctx: &Context, state: &RenderState) {
        if let Some(active) = &self.buff.active_or_edit(ctx, state) {
            let alpha = ui.clone_style().alpha;

            let (start, end) = self.bounding_box(ui, ctx, state.pos);
            let progress = self.progress.calc(ctx, active, self.max);
            let (fill_start, fill_end) = self.direction.progress_pos(start, self.size, progress);

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
    fn render_options(&mut self, ui: &Ui) {
        self.buff.render_options(ui);

        ui.spacing();

        enum_combo(ui, "Progress", &mut self.progress, ComboBoxFlags::empty());

        if let Progress::Intensity = self.progress {
            input_u32(ui, "Max", &mut self.max, 1, 10);
            helper(ui, || ui.text("Maximum progress value"));
        }

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
                .alpha_bar(true)
                // .preview(ColorPreview::Alpha)
                .build(ui);
        }
    }
}

impl Default for Bar {
    fn default() -> Self {
        Self {
            buff: ProgressTrigger::default(),
            progress: Progress::default(),
            max: 25,
            align: Align::Center,
            size: [128.0, 12.0],
            direction: Direction::Right,
            fill: colors::GREEN,
            background: colors::TRANSPARENT,
            border: true,
            border_size: 1.0,
            border_color: colors::BLACK,
        }
    }
}
