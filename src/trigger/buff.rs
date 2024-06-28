use super::{BuffThreshold, BuffTriggerId, Trigger};
use crate::{
    context::{Context, ContextUpdate},
    elements::RenderState,
    traits::RenderOptions,
};
use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};

// TODO: update examples!

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct BuffTrigger {
    /// Buff id(s).
    pub id: BuffTriggerId,

    /// Buff threshold requirement.
    #[serde(alias = "stacks")]
    pub threshold: BuffThreshold,

    /// Memoized active stacks & duration.
    #[serde(skip)]
    active_memo: Option<ActiveBuff>,
}

impl Trigger for BuffTrigger {
    fn is_active(&mut self, ctx: &Context) -> bool {
        self.active(ctx).is_some()
    }
}

impl BuffTrigger {
    fn force_update(&mut self, ctx: &Context) {
        let stacks = self.id.count_stacks(ctx);
        self.active_memo = (self.id.always() || self.threshold.is_met(stacks)).then(|| {
            let (duration, runout) = self.id.times(ctx);
            ActiveBuff {
                stacks,
                duration,
                runout,
            }
        });
    }

    pub fn active(&mut self, ctx: &Context) -> Option<ActiveBuff> {
        if ctx.has_update_or_edit(ContextUpdate::Buffs) {
            self.force_update(ctx);
        }
        self.active_memo.clone()
    }

    pub fn active_or_edit(&mut self, ctx: &Context, state: &RenderState) -> Option<ActiveBuff> {
        if state.is_edit(ctx) {
            Some(ActiveBuff {
                stacks: 1,
                runout: ctx.now + 5000 - (ctx.now % 5000),
                duration: 5000,
            })
        } else {
            self.active(ctx)
        }
    }

    pub fn active_stacks_or_edit(&mut self, ctx: &Context, state: &RenderState) -> Option<u32> {
        self.active_or_edit(ctx, state).map(|active| active.stacks)
    }

    pub fn active_runout_or_edit(&mut self, ctx: &Context, state: &RenderState) -> Option<u32> {
        self.active_or_edit(ctx, state).map(|active| active.runout)
    }
}

impl RenderOptions for BuffTrigger {
    fn render_options(&mut self, ui: &Ui) {
        self.id.render_options(ui);
        if !self.id.always() {
            self.threshold.render_options(ui);
            // TODO: we rely on buffs interval refreshing the memo, render options might want context for updates
        }
    }
}

#[derive(Debug, Clone)]
pub struct ActiveBuff {
    pub stacks: u32,
    pub duration: u32,
    pub runout: u32,
}
