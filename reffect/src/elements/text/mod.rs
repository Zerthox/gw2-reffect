mod decoration;
mod props;

use super::{align::AlignHorizontal, Props, RenderState};
use crate::{
    context::{Context, ContextUpdate},
    fmt::Unit,
    render::{Bounds, ComponentWise, Render, RenderDebug, RenderOptions},
    render_util::{
        debug_optional, draw_text_bg, helper, input_text_multi_with_menu, LoadedFont, Rect,
    },
    tree::TreeNode,
    trigger::{ProgressActive, ProgressValue},
};
use nexus::imgui::{InputTextFlags, Ui};
use serde::{Deserialize, Serialize};
use std::{iter::Peekable, str::Chars};

pub use self::{decoration::*, props::*};

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Text {
    pub text: String,

    #[serde(flatten)]
    pub font: LoadedFont,

    pub align: AlignHorizontal,

    #[serde(flatten)]
    pub props: Props<TextProps>,

    #[serde(skip)]
    frequent: bool,

    #[serde(skip)]
    text_memo: Option<String>,
}

impl Text {
    pub fn update(&mut self, ctx: &Context, state: &RenderState) {
        let active = state.trigger_active();
        self.props.update(ctx, active);
        if self.frequent || ctx.has_update_or_edit(ContextUpdate::State) {
            self.frequent = false; // reset frequent, only enable while active
            self.text_memo = active.map(|active| self.process_text(active, ctx, state));
        }
    }

    pub fn load(&mut self) {
        self.font.reload();
    }

    fn process_text(
        &mut self,
        active: &ProgressActive,
        ctx: &Context,
        state: &RenderState,
    ) -> String {
        const PREFIX: char = '%';

        let mut result = String::with_capacity(self.text.len()); // always same or larger size

        let is_timed = active.is_timed();
        let mut iter = self.text.chars().peekable();
        while let Some(el) = iter.next() {
            if el == PREFIX {
                if let Some(next) = iter.peek() {
                    match next {
                        'n' => {
                            iter.next();
                            result.push_str(&state.common.name);
                        }
                        'i' | 's' => {
                            iter.next();
                            result.push_str(&active.intensity().to_string());
                        }
                        'I' => {
                            iter.next();
                            result.push_str(&Unit::format(active.intensity()));
                        }
                        'c' | 'r' => {
                            iter.next();
                            result.push_str(&active.current_text(
                                Self::parse_value(&mut iter),
                                ctx.now,
                                false,
                                &ctx.settings.format,
                            ));
                            self.frequent = is_timed;
                        }
                        'C' => {
                            iter.next();
                            result.push_str(&active.current_text(
                                Self::parse_value(&mut iter),
                                ctx.now,
                                true,
                                &ctx.settings.format,
                            ));
                            self.frequent = is_timed;
                        }
                        'f' => {
                            iter.next();
                            result.push_str(&active.max_text(
                                Self::parse_value(&mut iter),
                                false,
                                &ctx.settings.format,
                            ));
                        }
                        'F' => {
                            iter.next();
                            result.push_str(&active.max_text(
                                Self::parse_value(&mut iter),
                                true,
                                &ctx.settings.format,
                            ));
                        }
                        'p' | 'P' => {
                            iter.next();
                            let progress =
                                active.progress_or_default(Self::parse_value(&mut iter), ctx.now);
                            result.push_str(&format!("{:.1}", (100.0 * progress)));
                            self.frequent = is_timed;
                        }
                        &PREFIX => {
                            iter.next();
                            result.push(PREFIX);
                        }
                        _ => {
                            result.push(PREFIX);
                        }
                    }
                } else {
                    result.push(el);
                }
            } else {
                result.push(el);
            }
        }

        result
    }

    fn parse_value(iter: &mut Peekable<Chars>) -> ProgressValue {
        match iter.peek() {
            Some('1') => {
                iter.next();
                ProgressValue::Primary
            }
            Some('2') => {
                iter.next();
                ProgressValue::Secondary
            }
            _ => ProgressValue::Primary,
        }
    }

    fn helper(ui: &Ui) {
        helper(ui, || {
            ui.text("Uppercase for pretty format");
            ui.text("Suffix 1 or 2 for primary/secondary");
            ui.text("%n for name");
            ui.text("%i for intensity");
            ui.text("%c for current amount");
            ui.text("%f for full/max amount");
            ui.text("%p for progress percent");
            ui.text("%% for % sign");
        });
    }

    fn calc_offset(&self, ui: &Ui, text: &str) -> [f32; 2] {
        self.align.text_offset(ui, text, self.props.scale)
    }
}

impl TreeNode for Text {}

impl Render for Text {
    fn render(&mut self, ui: &Ui, ctx: &Context, state: &RenderState) {
        self.update(ctx, state);

        if let Some(text) = &self.text_memo {
            let _font = self.font.push();
            let font_scale = self.props.scale;
            let offset = self.calc_offset(ui, text);
            let pos = state.pos.add(offset);
            let [r, g, b, a] = self.props.color;
            let alpha = a * ui.clone_style().alpha;
            let color = [r, g, b, alpha];

            self.props
                .decoration
                .render(ui, text, pos, font_scale, [0.0, 0.0, 0.0, alpha]);
            draw_text_bg(ui, text, pos, font_scale, color);
        }
    }
}

impl Bounds for Text {
    fn bounds(&self, ui: &Ui, _ctx: &Context) -> Rect {
        self.text_memo
            .as_ref()
            .map(|text| {
                let _font = self.font.push();
                let offset = self.calc_offset(ui, text);
                let size = ui.calc_text_size(text).mul_scalar(self.props.scale);
                (offset, offset.add(size))
            })
            .unwrap_or_default()
    }
}

impl RenderOptions for Text {
    fn render_options(&mut self, ui: &Ui, ctx: &Context) {
        input_text_multi_with_menu(
            ui,
            "##text",
            &mut self.text,
            [0.0, 3.0 * ui.text_line_height()],
            InputTextFlags::ALLOW_TAB_INPUT,
        );

        ui.same_line();
        ui.text("Text"); // own label to fix helper position
        Self::helper(ui);

        self.align.render_combo(ui);

        self.font.render_select(ui, "Font");

        self.props.base.render_options(ui, ctx);
    }

    fn render_tabs(&mut self, ui: &Ui, ctx: &Context) {
        if let Some(_token) = ui.tab_item("Condition") {
            self.props.render_condition_options(ui, ctx);
        }
    }
}

impl RenderDebug for Text {
    fn render_debug(&mut self, ui: &Ui, _ctx: &Context) {
        debug_optional(ui, "Font", self.font.as_font());
        ui.text(format!("Frequent: {}", self.frequent));
    }
}

impl Default for Text {
    fn default() -> Self {
        Self {
            text: String::new(),
            align: AlignHorizontal::Center,
            props: Props::default(),
            font: LoadedFont::empty(),
            frequent: false,
            text_memo: None,
        }
    }
}

impl Clone for Text {
    fn clone(&self) -> Self {
        Self {
            text: self.text.clone(),
            align: self.align,
            props: self.props.clone(),
            font: self.font.clone(),
            frequent: self.frequent,
            text_memo: None, // dont clone the memo
        }
    }
}
