mod element;
mod props;
mod source;

pub use self::{element::*, props::*, source::*};

use super::{align::AlignHorizontal, Props, RenderState};
use crate::{
    colors::{self, with_alpha, with_alpha_factor},
    component_wise::ComponentWise,
    context::{Context, EditState},
    render_util::{debug_optional, draw_spinner_bg, draw_text_bg, input_color_alpha, Rect},
    settings::icon::{DurationBarSettings, DurationTextSettings, StackTextSettings},
    traits::{RenderDebug, RenderOptions},
    trigger::ProgressActive,
};
use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Icon {
    #[serde(rename = "icon")]
    pub source: IconSource,

    #[serde(flatten)]
    pub props: Props<IconProps>,

    #[serde(alias = "duration")]
    pub duration_bar: bool,
    pub duration_text: bool,

    #[serde(alias = "stacks")]
    pub stacks_text: bool,
}

impl Icon {
    pub fn load(&mut self) {
        self.source.load();
    }

    fn texture_color(&self, ui: &Ui) -> [f32; 4] {
        let [r, g, b, a] = self.props.tint;
        [r, g, b, a * ui.clone_style().alpha]
    }

    pub fn rel_bounds(size: [f32; 2]) -> Rect {
        let [half_x, half_y] = size.mul_scalar(0.5);
        ([-half_x, -half_y], [half_x, half_y])
    }

    pub fn bounds(pos: [f32; 2], size: [f32; 2]) -> Rect {
        let half_size = size.mul_scalar(0.5);
        let start = pos.sub(half_size);
        let end = pos.add(half_size);
        (start, end)
    }

    pub fn render(
        &mut self,
        ui: &Ui,
        ctx: &Context,
        state: &RenderState,
        active: Option<&ProgressActive>,
        size: [f32; 2],
    ) {
        self.props.update(ctx, active);

        if let Some(active) = active {
            let texture = self.source.get_texture();
            if self.source.is_empty() || texture.is_some() {
                let (start, end) = Self::bounds(state.pos, size);
                let color @ [_, _, _, alpha] = self.texture_color(ui);

                // render icon
                if let Some(texture) = texture {
                    ui.get_background_draw_list()
                        .add_image(texture, start, end)
                        .col(color)
                        .build();
                }

                // render duration bar
                if self.duration_bar {
                    if let Some(progress) = active.progress(ctx.now) {
                        let DurationBarSettings { height, color } = ctx.icon_settings.duration_bar;

                        let [start_x, _] = start;
                        let [end_x, end_y] = end;

                        let x1 = start_x;
                        let x2 = end_x;
                        let x_mid = x1 + progress * (x2 - x1);
                        let y1 = end_y - height;
                        let y2 = end_y;

                        ui.get_background_draw_list()
                            .add_rect([x1, y1], [x_mid, y2], with_alpha_factor(color, alpha))
                            .filled(true)
                            .build();
                    }
                }

                // render stack count
                if self.stacks_text {
                    let StackTextSettings {
                        scale,
                        offset,
                        color: color @ [_, _, _, alpha],
                        decoration,
                    } = ctx.icon_settings.stack_text;

                    let stacks = active.intensity();
                    let text = if stacks > 99 {
                        "!"
                    } else {
                        &stacks.to_string()
                    };

                    let [width, height] = size;
                    let font_size = scale * width.min(height);
                    let font_scale = font_size / ui.current_font_size();
                    let [x_offset, _] = AlignHorizontal::Right.text_offset(ui, text, font_scale);
                    let line_height = font_scale * ui.text_line_height();
                    let text_pos = end.add([x_offset, -line_height]).sub(offset);

                    let decoration_color = with_alpha(colors::BLACK, alpha);
                    decoration.render(ui, text, text_pos, font_scale, decoration_color);
                    draw_text_bg(ui, text, text_pos, font_scale, color);
                }

                // render duration text
                if self.duration_text {
                    if let Some(remain) = active.current(ctx.now) {
                        let DurationTextSettings {
                            max_remain,
                            scale,
                            color: color @ [_, _, _, alpha],
                            decoration,
                        } = ctx.icon_settings.duration_text;

                        if remain < max_remain {
                            let text = active.current_text(ctx.now);

                            let [width, height] = size;
                            let font_size = scale * width.min(height);
                            let font_scale = font_size / ui.current_font_size();
                            let offset = AlignHorizontal::Center.text_offset(ui, &text, font_scale);
                            let text_pos = state.pos.add(offset);

                            let decoration_color = with_alpha(colors::BLACK, alpha);
                            decoration.render(ui, &text, text_pos, font_scale, decoration_color);
                            draw_text_bg(ui, &text, text_pos, font_scale, color);
                        }
                    }
                }
            } else {
                let [x, _] = size;
                draw_spinner_bg(
                    ui,
                    state.pos,
                    0.4 * x,
                    colors::WHITE,
                    with_alpha(colors::WHITE, 0.3),
                )
            }
        }
    }
}

impl RenderOptions for Icon {
    fn render_options(&mut self, ui: &Ui, _state: &mut EditState) {
        self.source.render_select(ui);

        input_color_alpha(ui, "Tint", &mut self.props.base.tint);

        // TODO: duration customizations
        ui.checkbox("Show Duration Bar", &mut self.duration_bar);
        ui.checkbox("Show Duration Text", &mut self.duration_text);

        // TODO: stacks customizations
        ui.checkbox("Show Stacks", &mut self.stacks_text);
    }

    fn render_tabs(&mut self, ui: &Ui, state: &mut EditState) {
        if let Some(_token) = ui.tab_item("Condition") {
            self.props.render_condition_options(ui, state);
        }
    }
}

impl RenderDebug for Icon {
    fn render_debug(&mut self, ui: &Ui) {
        debug_optional(
            ui,
            "Texture",
            self.source
                .get_texture()
                .map(|texture| texture.id() as *mut ()),
        );
    }
}

impl Default for Icon {
    fn default() -> Self {
        Self {
            source: IconSource::Unknown,
            props: Props::default(),
            duration_bar: false,
            duration_text: false,
            stacks_text: false,
        }
    }
}
