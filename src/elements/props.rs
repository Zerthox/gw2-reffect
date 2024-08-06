use crate::{
    action::Action,
    colors,
    context::{Context, EditState},
    render_util::{collapsing_header_same_line_end, delete_confirm_modal},
    traits::RenderOptions,
    trigger::{Condition, ProgressActive},
};
use nexus::imgui::{CollapsingHeader, Direction, StyleColor, TreeNodeFlags, Ui};
use partial::IntoPartial;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
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
        self.current = self.base.clone();
        if let Some(active) = active {
            for condition in &mut self.conditions {
                condition.process(&mut self.current, ctx, active);
            }
        }
    }
}

impl<T> std::ops::Deref for Props<T>
where
    T: Clone + IntoPartial,
    T::Partial: Clone + fmt::Debug + Serialize + for<'de> Deserialize<'de>,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.current
    }
}

impl<T> RenderOptions for Props<T>
where
    T: Clone + IntoPartial + RenderOptions,
    T::Partial: Clone + fmt::Debug + Serialize + for<'d> Deserialize<'d> + PartialOptions<T>,
{
    fn render_options(&mut self, ui: &Ui, state: &mut EditState) {
        self.base.render_options(ui, state)
    }

    fn render_tabs(&mut self, ui: &Ui, state: &mut EditState) {
        if let Some(_token) = ui.tab_item("Condition") {
            let mut action = Action::new();
            for (i, condition) in self.conditions.iter_mut().enumerate() {
                let _id = ui.push_id(i as i32);

                let mut remains = true;
                let cond_name = condition.trigger.as_ref();

                let open = CollapsingHeader::new(format!("{cond_name}###cond{i}"))
                    .flags(TreeNodeFlags::ALLOW_ITEM_OVERLAP)
                    .begin_with_close_button(ui, &mut remains);

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
                    ui.text(format!("Delete {cond_name} Condition?"))
                }) {
                    action = Action::Delete(i);
                }

                if open {
                    condition.trigger.render_options(ui, state);
                    ui.spacing();
                    condition.properties.render_options(ui, &self.base);
                    ui.spacing();
                }
            }
            action.perform(&mut self.conditions);

            if ui.button("Add Condition") {
                self.conditions.push(Condition::default());
            }
        }
    }
}

pub trait PartialOptions<T>
where
    T: IntoPartial<Partial = Self>,
{
    fn render_options(&mut self, ui: &Ui, base: &T);
}
