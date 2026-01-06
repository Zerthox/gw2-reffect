mod decoration;
mod props;

use super::{Props, RenderCtx, align::AlignHorizontal};
use crate::{
    context::{Context, Update},
    elements::Common,
    fmt::Unit,
    render::{
        Bounds, ComponentWise, LoadedFont, Rect, debug_optional, draw_text_bg, helper,
        input_text_multi_with_menu,
    },
    tree::TreeNode,
    trigger::{ProgressActive, ProgressValue},
};
use const_default::ConstDefault;
use nexus::imgui::{InputTextFlags, Ui};
use serde::{Deserialize, Serialize};
use std::{iter::Peekable, str::Chars};

pub use self::{decoration::*, props::*};

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct Text {
    /// Text contents.
    pub text: String,

    /// Text font.
    pub font: LoadedFont,

    /// Text alignment.
    pub align: AlignHorizontal,

    #[serde(flatten)]
    pub props: Props<TextProps>,

    #[serde(skip)]
    frequent: bool,

    #[serde(skip)]
    text_memo: Option<String>,
}

impl Text {
    pub fn update(&mut self, ctx: &RenderCtx, common: &Common) {
        let active = common.trigger.active();
        self.props.update(ctx, active);
        if self.frequent || ctx.has_update_or_edit(Update::Game) {
            self.frequent = false; // reset frequent, only enable while active
            self.text_memo = active.map(|active| self.process_text(active, ctx, common));
        }
    }

    pub fn load(&mut self) {
        self.font.reload();
    }

    fn process_text(
        &mut self,
        active: &ProgressActive,
        ctx: &RenderCtx,
        common: &Common,
    ) -> String {
        const PREFIX: char = '%';

        let mut result = String::with_capacity(self.text.len()); // always same or larger size

        let is_timed = active.is_timed();
        let mut iter = self.text.chars().peekable();
        while let Some(el) = iter.next() {
            if el == PREFIX {
                if let Some(next) = iter.peek().copied() {
                    match next {
                        'n' => {
                            iter.next();
                            result.push_str(&common.name);
                        }
                        'i' | 's' => {
                            // backwards compat for %s stacks
                            iter.next();
                            result.push_str(&active.intensity().to_string());
                        }
                        'I' => {
                            iter.next();
                            result.push_str(&Unit::format(active.intensity()));
                        }
                        'c' | 'r' | 'C' => {
                            // backwards compat for %r remaining
                            iter.next();
                            let value = Self::parse_value(&mut iter);
                            let pretty = next.is_ascii_uppercase();
                            result.push_str(&active.current_text(
                                value,
                                ctx.now,
                                pretty,
                                &ctx.settings.format,
                            ));
                            self.frequent = is_timed;
                        }
                        'f' | 'F' => {
                            iter.next();
                            let value = Self::parse_value(&mut iter);
                            let pretty = next.is_ascii_uppercase();
                            result.push_str(&active.max_text(value, pretty, &ctx.settings.format));
                        }
                        'p' | 'P' => {
                            iter.next();
                            let value = Self::parse_value(&mut iter);
                            let pretty = next.is_ascii_uppercase();
                            let progress = 100.0 * active.progress_or_default(value, ctx.now);
                            let precision = if pretty { 1 } else { 0 };
                            result.push_str(&format!("{progress:.precision$}"));
                            self.frequent = is_timed;
                        }
                        PREFIX => {
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

impl Text {
    pub fn render(&mut self, ui: &Ui, ctx: &RenderCtx, common: &Common) {
        self.update(ctx, common);

        if let Some(text) = &self.text_memo {
            let _font = self.font.push();
            let font_scale = self.props.scale;
            let offset = self.calc_offset(ui, text);
            let pos = ctx.pos().add(offset);
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

impl Text {
    pub fn render_options(&mut self, ui: &Ui, ctx: &RenderCtx) {
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

    pub fn render_tabs(&mut self, ui: &Ui, ctx: &RenderCtx, common: &Common) {
        if let Some(_token) = ui.tab_item("Condition") {
            self.props
                .render_condition_options(ui, ctx, &common.trigger.source);
        }
    }
}

impl Text {
    pub fn render_debug(&mut self, ui: &Ui, _ctx: &RenderCtx) {
        debug_optional(ui, "Font", self.font.as_font());
        ui.text(format!("Frequent: {}", self.frequent));
    }
}

impl ConstDefault for Text {
    const DEFAULT: Self = Self {
        text: String::new(),
        align: AlignHorizontal::Center,
        props: Props::DEFAULT,
        font: LoadedFont::empty(),
        frequent: false,
        text_memo: None,
    };
}

impl Default for Text {
    fn default() -> Self {
        Self::DEFAULT
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
