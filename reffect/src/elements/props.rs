use crate::{
    action::{Action, DynAction},
    colors,
    context::Context,
    render::{collapsing_header_same_line_end, delete_confirm_modal, item_context_menu},
    trigger::{Condition, ProgressActive, ProgressSource},
};
use const_default::ConstDefault;
use nexus::imgui::{CollapsingHeader, Direction, MenuItem, StyleColor, TreeNodeFlags, Ui};
use partial::IntoPartial;
use serde::{Deserialize, Serialize};
use std::{fmt, ops};

#[derive(Debug, Default, ConstDefault, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Props<T>
where
    T: Clone + IntoPartial,
    T::Partial: Clone + fmt::Debug + Serialize + for<'d> Deserialize<'d>,
{
    #[serde(flatten)]
    pub base: T,

    pub conditions: Vec<Condition<T>>,

    #[serde(skip)]
    pub current: T,
}

impl<T> Props<T>
where
    T: Clone + IntoPartial,
    T::Partial: Clone + fmt::Debug + Serialize + for<'de> Deserialize<'de>,
{
    pub fn update(&mut self, ctx: &Context, active: Option<&ProgressActive>) {
        if ctx.has_any_update_or_edit() {
            self.current = self.base.clone();
            if let Some(active) = active {
                for condition in &mut self.conditions {
                    condition.process(&mut self.current, ctx, active);
                }
            }
        }
    }
}

impl<T> ops::Deref for Props<T>
where
    T: Clone + IntoPartial,
    T::Partial: Clone + fmt::Debug + Serialize + for<'de> Deserialize<'de>,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.current
    }
}

impl<T> Props<T>
where
    T: Clone + IntoPartial + 'static,
    T::Partial: fmt::Debug + Clone + Serialize + for<'d> Deserialize<'d> + PartialProps<T>,
{
    pub fn render_condition_options(
        &mut self,
        ui: &Ui,
        ctx: &Context,
        source: &ProgressSource,
    ) -> DynAction<Self> {
        let mut copy_action = DynAction::<Self>::empty();
        let mut action = Action::new();

        let len = self.conditions.len();
        for (i, condition) in self.conditions.iter_mut().enumerate() {
            let _id = ui.push_id(i as i32);

            let mut remains = true;

            let label = format!("{}###cond{i}", condition.trigger);
            let open = CollapsingHeader::new(&label)
                .flags(TreeNodeFlags::ALLOW_ITEM_OVERLAP)
                .begin_with_close_button(ui, &mut remains);

            let cloned = condition.clone();
            item_context_menu(label, || {
                if MenuItem::new("Duplicate").build(ui) {
                    action = Action::Duplicate(i)
                }

                if MenuItem::new("Copy to all siblings").build(ui) {
                    copy_action.set(move |props| {
                        if props.conditions.len() == len
                            && props.conditions[i].trigger.is_same_type(&cloned.trigger)
                        {
                            props.conditions[i] = cloned.clone();
                        } else {
                            props.conditions.push(cloned.clone());
                        }
                    });
                }
            });

            {
                let size_x = ui.frame_height();
                let [spacing_x, _] = ui.clone_style().item_spacing;
                collapsing_header_same_line_end(ui, 3.0 * size_x + 2.0 * spacing_x);

                let _style = ui.push_style_color(StyleColor::Button, colors::TRANSPARENT);
                if ui.arrow_button("up", Direction::Up) {
                    action = Action::Up(i);
                }

                ui.same_line();
                if ui.arrow_button("down", Direction::Down) {
                    action = Action::Down(i);
                }
            }

            let title = format!("Confirm Delete##reffectcond{i}");
            if !remains {
                ui.open_popup(&title);
            }
            if delete_confirm_modal(ui, &title, || {
                ui.text(format!("Delete Condition {}?", condition.trigger))
            }) {
                action = Action::Delete(i);
            }

            if open {
                condition.render_options(ui, ctx, source, &self.base);
            }
        }
        action.perform(&mut self.conditions);

        if ui.button("Add Condition") {
            self.conditions.push(Condition::default());
        }

        copy_action
    }
}

pub trait PartialProps<T>
where
    T: IntoPartial<Partial = Self>,
{
    fn render_options(&mut self, ui: &Ui, base: &T);
}
