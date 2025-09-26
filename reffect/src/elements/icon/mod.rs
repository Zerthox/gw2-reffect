mod element;
mod props;
mod source;

pub use self::{element::*, props::*, source::*};

use super::{Common, Props, RenderCtx};
use crate::{
    action::DynAction,
    colors::{self, with_alpha, with_alpha_factor},
    context::{Context, SkillId},
    render::{ComponentWise, Rect, debug_optional, draw_spinner_bg},
    render_copy_field,
    settings::icon::DurationBarSettings,
    trigger::{ProgressActive, ProgressValue},
};
use const_default::ConstDefault;
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
        ctx: &RenderCtx,
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
                SkillId::Unknown
            });

            let (start, end) = Self::bounds(size);
            let start = ctx.pos().add(start);
            let end = ctx.pos().add(end);
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
                    ctx.pos(),
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
                let settings = &ctx.settings.icon.stack_text;

                let stacks = active.intensity();
                if stacks >= settings.threshold {
                    let text = if stacks > 99 {
                        "!"
                    } else {
                        &stacks.to_string()
                    };

                    let color = settings.text.color;
                    settings
                        .text
                        .render(ui, start, size, color, small_size, text);
                }
            }

            // render duration text
            if self.duration_text {
                if let Some(remain) = active.current(ProgressValue::Primary, ctx.now) {
                    let settings = &ctx.settings.icon.duration_text;
                    let threshold = settings.threshold(active);

                    if remain < threshold as f32 {
                        let text = active.current_text(
                            ProgressValue::Primary,
                            ctx.now,
                            false,
                            &ctx.settings.format,
                        );

                        let color = settings.color(active.progress_rate());
                        settings
                            .text
                            .render(ui, start, size, color, small_size, text);
                    }
                }
            }
        }
    }

    pub fn render_options(&mut self, ui: &Ui, ctx: &RenderCtx) -> DynAction<Self> {
        let mut action = DynAction::<Self>::empty();

        let source_action = self.source.render_select(ui, ctx);
        action.or(source_action.map(|icon: &mut Self| &mut icon.source));

        ui.spacing();

        let props_action = self.props.base.render_options(ui);
        action.or(props_action.map(|icon: &mut Self| &mut icon.props.base));

        ui.checkbox("Show Duration Bar", &mut self.duration_bar);
        render_copy_field!(action, ui, self.duration_bar);

        ui.checkbox("Show Duration Text", &mut self.duration_text);
        render_copy_field!(action, ui, self.duration_text);

        ui.checkbox("Show Stacks", &mut self.stacks_text);
        render_copy_field!(action, ui, self.stacks_text);

        action
    }

    pub fn render_tabs(&mut self, ui: &Ui, ctx: &Context, common: &Common) -> DynAction<Self> {
        if let Some(_token) = ui.tab_item("Condition") {
            self.props
                .render_condition_options(ui, ctx, &common.trigger.source)
                .map(|icon: &mut Self| &mut icon.props)
        } else {
            DynAction::empty()
        }
    }

    pub fn render_debug(&mut self, ui: &Ui, _ctx: &RenderCtx) {
        const SIZE: [f32; 2] = [64.0, 64.0];

        let texture = self.source.get_texture(SkillId::Unknown);
        debug_optional(
            ui,
            "Texture",
            texture.map(|texture| texture.id() as *mut ()),
        );
        if let Some(texture) = texture {
            if ui.is_item_hovered() {
                ui.tooltip(|| {
                    let pos = ui.cursor_screen_pos();
                    ui.dummy(SIZE);
                    ui.get_foreground_draw_list()
                        .add_image(texture, pos, pos.add(SIZE))
                        .build();
                });
            }
        }
    }
}

impl ConstDefault for Icon {
    const DEFAULT: Self = Self {
        source: IconSource::Unknown,
        props: Props::DEFAULT,
        duration_bar: false,
        duration_text: false,
        stacks_text: false,
    };
}

impl Default for Icon {
    fn default() -> Self {
        Self::DEFAULT
    }
}
