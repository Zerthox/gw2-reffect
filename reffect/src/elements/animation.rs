use crate::{
    elements::RenderCtx,
    enums::check_variant_array,
    render::{
        cycle_progress, enum_combo, helper, input_float_with_format, input_seconds,
        push_alpha_change,
    },
    util::non_zero_u32,
};
use const_default::ConstDefault;
use nexus::imgui::{ComboBoxFlags, InputTextFlags, StyleStackToken, Ui};
use serde::{Deserialize, Serialize};
use std::num::NonZero;
use strum::{AsRefStr, EnumCount, EnumIter, IntoStaticStr, VariantArray};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct Animation {
    /// Kind of animation.
    pub kind: AnimationKind,

    /// Animation period in milliseconds.
    pub period: NonZero<u32>,
}

impl Animation {
    pub fn animate<'ui>(&mut self, ui: &'ui Ui, ctx: &RenderCtx) -> impl Drop + 'ui {
        let progress = cycle_progress(ui, self.period);
        self.kind.animate(ui, ctx, progress)
    }
}

impl ConstDefault for Animation {
    const DEFAULT: Self = Self {
        kind: AnimationKind::Pulse,
        period: non_zero_u32!(1000),
    };
}

impl Default for Animation {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl Animation {
    pub fn render_options(&mut self, ui: &Ui) {
        enum_combo(ui, "Animation", &mut self.kind, ComboBoxFlags::empty());

        let mut period = self.period.get();
        if input_seconds(ui, "Period", &mut period) {
            self.period = NonZero::new(period).unwrap_or(non_zero_u32!(1));
        }
        helper(ui, || ui.text("Animation loop duration in seconds"));

        self.kind.render_options(ui);
    }
}

// TODO: tint animation changing color via state?
#[derive(Debug, Clone, AsRefStr, IntoStaticStr, EnumIter, EnumCount, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum AnimationKind {
    Pulse,
    Shake { intensity: f32 },
    Bounce { intensity: f32 },
}

impl VariantArray for AnimationKind {
    const VARIANTS: &'static [Self] = &[
        Self::Pulse,
        Self::Shake { intensity: 10.0 },
        Self::Bounce { intensity: 10.0 },
    ];
}

const _: () = check_variant_array::<AnimationKind>();

impl AnimationKind {
    fn back_and_forth(progress: f32) -> f32 {
        if progress < 0.5 {
            1.0 - 2.0 * progress
        } else {
            2.0 * progress - 1.0
        }
    }

    pub fn animate<'ui>(&mut self, ui: &'ui Ui, ctx: &RenderCtx, progress: f32) -> impl Drop + 'ui {
        enum Token<'ui> {
            None,
            Style(StyleStackToken<'ui>),
        }

        impl<'ui> Drop for Token<'ui> {
            fn drop(&mut self) {}
        }

        // we can directly change render state for the current element here
        match *self {
            Self::Pulse => {
                // TODO: dont disappear completely?
                let progress = Self::back_and_forth(progress);
                Token::Style(push_alpha_change(ui, progress))
            }
            Self::Shake { intensity } => {
                let progress = Self::back_and_forth(progress);
                ctx.add_offset([(progress - 0.5) * intensity, 0.0]);
                Token::None
            }
            Self::Bounce { intensity } => {
                let progress = Self::back_and_forth(progress);
                ctx.add_offset([0.0, (progress - 0.5) * intensity]);
                Token::None
            }
        }
    }

    pub fn render_options(&mut self, _ui: &Ui) {
        match self {
            AnimationKind::Pulse => {}
            AnimationKind::Shake { intensity } | AnimationKind::Bounce { intensity } => {
                input_float_with_format(
                    "Intensity",
                    intensity,
                    1.0,
                    10.0,
                    "%.1f",
                    InputTextFlags::empty(),
                );
            }
        }
    }
}
