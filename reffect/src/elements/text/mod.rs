mod decoration;
mod fragment;
mod process;
mod props;

use super::{Props, RenderCtx, align::AlignHorizontal};
use crate::{
    context::Context,
    elements::Common,
    math::ComponentWise,
    render::{
        Bounds, LoadedFont, Rect, debug_optional, draw_text_bg, helper, input_text_multi_with_menu,
    },
    settings::FormatSettings,
    tree::TreeNode,
};
use const_default::ConstDefault;
use nexus::imgui::{InputTextFlags, Ui};
use serde::{Deserialize, Serialize};
use std::fmt::Write;

pub use self::{decoration::*, fragment::*, process::*, props::*};

/// Text element.
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
    processing: Processing,

    #[serde(skip)]
    processed_text: Option<String>,
}

impl Text {
    /// Loads the text element.
    pub fn load(&mut self) {
        self.reprocess_next_frame();
    }

    /// Forces the text to be reprocessed next time it renders.
    pub fn reprocess_next_frame(&mut self) {
        self.processing = Processing::Frame;
    }

    /// Checks whether the text neeeds reprocessing before rendering.
    pub fn needs_reprocess(&mut self, ctx: &Context, common: &Common) -> bool {
        ctx.edit.is_edited(common.id) || self.processing.needs_reprocess(ctx, &common.trigger)
    }

    /// Reprocesses the text if needed.
    pub fn reprocess_if_need(&mut self, ctx: &Context, settings: &FormatSettings, common: &Common) {
        if self.needs_reprocess(ctx, common) {
            self.reprocess(ctx, settings, common);
        }
    }

    /// Force reprocesses the text.
    pub fn reprocess(&mut self, ctx: &Context, settings: &FormatSettings, common: &Common) {
        self.processing = Processing::MIN;
        self.processed_text = common.trigger.active().map(|active| {
            let mut text = String::with_capacity(self.text.len()); // expecting same size or larger
            for fragment in TextFragment::parse(&self.text) {
                self.processing.or(Processing::resolve(&fragment, active));
                let _ = write!(
                    &mut text,
                    "{}",
                    fragment.display(active, ctx, settings, &common.name)
                );
            }
            text
        });
    }

    /// Returns the processed text, if present.
    pub fn processed_text(&self) -> Option<&str> {
        self.processed_text.as_deref()
    }

    /// Renders the text.
    pub fn render(&mut self, ui: &Ui, ctx: &RenderCtx, common: &Common) {
        self.reprocess_if_need(ctx, &ctx.settings.format, common);

        if let Some(text) = &self.processed_text {
            let _font = self.font.push(ui);
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

    /// Calculates the text offset.
    fn calc_offset(&self, ui: &Ui, text: &str) -> [f32; 2] {
        self.align.text_offset(ui, text, self.props.scale)
    }

    /// Renders text element options.
    pub fn render_options(&mut self, ui: &Ui, ctx: &RenderCtx) {
        let changed = input_text_multi_with_menu(
            ui,
            "##text",
            &mut self.text,
            [0.0, 3.0 * ui.text_line_height()],
            InputTextFlags::ALLOW_TAB_INPUT,
        );
        if changed {
            self.reprocess_next_frame();
        }

        ui.same_line();
        ui.text("Text"); // own label to fix helper position
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

        self.align.render_select(ui);

        self.font.render_select(ui, "Font");

        self.props.base.render_options(ui, ctx);
    }

    /// Renders text element tabs.
    pub fn render_tabs(&mut self, ui: &Ui, ctx: &RenderCtx, common: &Common) {
        if let Some(_token) = ui.tab_item("Condition") {
            self.props
                .render_condition_options(ui, ctx, &common.trigger.source);
        }
    }

    /// Renders text element debug information.
    pub fn render_debug(&mut self, ui: &Ui, _ctx: &RenderCtx) {
        debug_optional(ui, "Font", self.font.as_font());
        ui.text(format!("Processing: {}", self.processing));
    }
}

impl Bounds for Text {
    fn bounds(&self, ui: &Ui, _ctx: &Context) -> Rect {
        self.processed_text
            .as_ref()
            .map(|text| {
                let _font = self.font.push(ui);
                let offset = self.calc_offset(ui, text);
                let size = ui.calc_text_size(text).mul_scalar(self.props.scale);
                (offset, offset.add(size))
            })
            .unwrap_or_default()
    }
}

impl ConstDefault for Text {
    const DEFAULT: Self = Self {
        text: String::new(),
        align: AlignHorizontal::Center,
        props: Props::DEFAULT,
        font: LoadedFont::empty(),
        processing: Processing::Frame,
        processed_text: None,
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
            ..Self::DEFAULT // dont clone internal state
        }
    }
}

impl TreeNode for Text {}
