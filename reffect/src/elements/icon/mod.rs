mod element;
mod props;
mod source;

pub use self::{element::*, props::*, source::*};

use super::{align::AlignHorizontal, Props, RenderState};
use crate::{
    action::DynAction,
    context::Context,
    render::{
        colors::{self, with_alpha, with_alpha_factor},
        ComponentWise, RenderDebug, RenderOptions,
    },
    render_copy_field,
    render_util::{debug_optional, draw_spinner_bg, draw_text_bg, Rect},
    settings::icon::{DurationBarSettings, DurationTextSettings, StackTextSettings},
    trigger::{ProgressActive, ProgressValue, Skill},
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

    pub fn bounds(size: [f32; 2]) -> Rect {
        let start = size.mul_scalar(-0.5);
        let end = size.mul_scalar(0.5);
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
            let [width, height] = size;
            let small_size = width.min(height);
            let texture = self.source.get_texture(if ctx.settings.use_game_icons {
                active.skill()
            } else {
                Skill::Unknown
            });

            let (start, end) = Self::bounds(size);
            let start = state.pos.add(start);
            let end = state.pos.add(end);
            let round = self.props.round * small_size;
            let color @ [_, _, _, alpha] = self.texture_color(ui);

            // render icon
            if let Some(texture) = texture {
                let uv_change = 0.5 * (1.0 - self.props.zoom);
                let uv_min = [uv_change, uv_change];
                let uv_max = [1.0, 1.0].sub_scalar(uv_change);
                ui.get_background_draw_list()
                    .add_image_rounded(texture, start, end, round)
                    .uv_min(uv_min)
                    .uv_max(uv_max)
                    .col(color)
                    .build();
            } else if !self.source.is_empty() {
                draw_spinner_bg(
                    ui,
                    state.pos,
                    0.4 * small_size,
                    with_alpha(colors::WHITE, alpha),
                    with_alpha(colors::WHITE, 0.3 * alpha),
                )
            }

            if self.props.border_size > 0.0 {
                ui.get_background_draw_list()
                    .add_rect(
                        start,
                        end,
                        with_alpha_factor(self.props.border_color, alpha),
                    )
                    .thickness(self.props.border_size)
                    .rounding(round)
                    .build();
            }

            // render duration bar
            if self.duration_bar {
                if let Some(progress) = active.progress(ProgressValue::PreferSecondary, ctx.now) {
                    let DurationBarSettings { height, color } = ctx.settings.icon.duration_bar;

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
                    threshold,
                    scale,
                    offset,
                    color: color @ [_, _, _, alpha],
                    decoration,
                } = ctx.settings.icon.stack_text;

                let stacks = active.intensity();
                if stacks >= threshold {
                    let text = if stacks > 99 {
                        "!"
                    } else {
                        &stacks.to_string()
                    };

                    let font_size = scale * small_size;
                    let font_scale = font_size / ui.current_font_size();
                    let [x_offset, _] = AlignHorizontal::Right.text_offset(ui, text, font_scale);
                    let line_height = font_scale * ui.text_line_height();
                    let text_pos = end.add([x_offset, -line_height]).sub(offset);

                    let decoration_color = with_alpha(colors::BLACK, alpha);
                    decoration.render(ui, text, text_pos, font_scale, decoration_color);
                    draw_text_bg(ui, text, text_pos, font_scale, color);
                }
            }

            // render duration text
            if self.duration_text {
                if let Some(remain) = active.current(ProgressValue::Primary, ctx.now) {
                    let settings = &ctx.settings.icon.duration_text;
                    let DurationTextSettings {
                        threshold_buff: _,
                        threshold_ability: _,
                        scale,
                        color: _,
                        color_fast: _,
                        color_slow: _,
                        decoration,
                    } = *settings;

                    let threshold = settings.threshold(active);
                    if remain < threshold {
                        let text = active.current_text(
                            ProgressValue::Primary,
                            ctx.now,
                            false,
                            &ctx.settings.format,
                        );

                        let font_size = scale * small_size;
                        let font_scale = font_size / ui.current_font_size();
                        let offset = AlignHorizontal::Center.text_offset(ui, &text, font_scale);
                        let text_pos = state.pos.add(offset);

                        let color @ [_, _, _, alpha] = settings.color(active.progress_rate());
                        let decoration_color = with_alpha(colors::BLACK, alpha);
                        decoration.render(ui, &text, text_pos, font_scale, decoration_color);
                        draw_text_bg(ui, &text, text_pos, font_scale, color);
                    }
                }
            }
        }
    }

    pub fn render_options(&mut self, ui: &Ui, ctx: &Context) -> DynAction<Self> {
        let mut action = DynAction::<Self>::empty();

        let source_action = self.source.render_select(ui, ctx);
        action.or(source_action.map(|icon: &mut Self| &mut icon.source));

        ui.spacing();

        let props_action = self.props.base.render_options(ui, ctx);
        action.or(props_action.map(|icon: &mut Self| &mut icon.props.base));

        ui.checkbox("Show Duration Bar", &mut self.duration_bar);
        render_copy_field!(action, ui, self.duration_bar);

        ui.checkbox("Show Duration Text", &mut self.duration_text);
        render_copy_field!(action, ui, self.duration_text);

        ui.checkbox("Show Stacks", &mut self.stacks_text);
        render_copy_field!(action, ui, self.stacks_text);

        action
    }

    pub fn render_tabs(&mut self, ui: &Ui, ctx: &Context) -> DynAction<Self> {
        if let Some(_token) = ui.tab_item("Condition") {
            self.props
                .render_condition_options(ui, ctx)
                .map(|icon: &mut Self| &mut icon.props)
        } else {
            DynAction::empty()
        }
    }
}

impl RenderDebug for Icon {
    fn render_debug(&mut self, ui: &Ui, _ctx: &Context) {
        debug_optional(
            ui,
            "Texture",
            self.source
                .get_texture(Skill::Unknown)
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
