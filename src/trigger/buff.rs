use super::{BuffThreshold, BuffTriggerId, Trigger};
use crate::{
    context::{Context, ContextUpdate},
    elements::RenderState,
    traits::RenderOptions,
};
use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};

// TODO: replace with generic progress trigger or similar?

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
        let active = self.id.always() || ctx.buffs_ok() && self.threshold.is_met(stacks);
        self.active_memo = active.then(|| {
            let (apply, runout) = self.id.time_range(ctx);
            ActiveBuff {
                stacks,
                apply,
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
            let apply = ctx.now - (ctx.now % 5000);
            Some(ActiveBuff {
                stacks: 1,
                runout: apply + 5000,
                apply,
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
    pub apply: u32,
    pub runout: u32,
}

impl ActiveBuff {
    pub fn full_duration(&self) -> u32 {
        self.runout.saturating_sub(self.apply)
    }
}
