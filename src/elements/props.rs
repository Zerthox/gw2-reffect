use crate::{
    action::Action,
    colors,
    context::{Context, EditState},
    render_util::{collapsing_header_same_line_end, delete_confirm_modal},
    traits::RenderOptions,
    trigger::{ProgressActive, ProgressThreshold},
};
use fields::{AllFields, Fields};
use nexus::imgui::{
    CollapsingHeader, ComboBoxFlags, Direction, Selectable, StyleColor, TreeNodeFlags, Ui,
};
use serde::{Deserialize, Serialize};
use std::{fmt, mem};

// TODO: multiple conditions with 1 threshold? use props struct with all optional fields instead of enum?

pub type Condition<T> = (ProgressThreshold, <T as Fields>::Field);

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Props<T>
where
    T: Clone + Fields,
    <T as Fields>::Field: Clone + fmt::Debug + Serialize + for<'d> Deserialize<'d>,
{
    #[serde(flatten)]
    pub base: T,

    pub conditions: Vec<Condition<T>>,

    #[serde(skip)]
    pub current: T,
}

impl<T> Props<T>
where
    T: Clone + Fields,
    <T as Fields>::Field: Clone + fmt::Debug + Serialize + for<'de> Deserialize<'de>,
{
    pub fn update(&mut self, ctx: &Context, active: Option<&ProgressActive>) {
        self.current = self.base.clone();
        if let Some(active) = active {
            for (threshold, prop) in &self.conditions {
                if threshold.is_met(active, ctx) {
                    self.current.set(prop.clone());
                }
            }
        }
    }
}

impl<T> std::ops::Deref for Props<T>
where
    T: Clone + Fields,
    <T as Fields>::Field: Clone + fmt::Debug + Serialize + for<'de> Deserialize<'de>,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.current
    }
}

impl<T> RenderOptions for Props<T>
where
    T: Clone + Fields + AllFields + RenderOptions,
    <T as Fields>::Field: Default
        + Clone
        + fmt::Debug
        + AsRef<str>
        + RenderOptions
        + Serialize
        + for<'d> Deserialize<'d>,
{
    fn render_options(&mut self, ui: &Ui, state: &mut EditState) {
        self.base.render_options(ui, state)
    }

    fn render_tabs(&mut self, ui: &Ui, state: &mut EditState) {
        if let Some(_token) = ui.tab_item("Condition") {
            let mut action = Action::new();
            for (i, (threshold, prop)) in self.conditions.iter_mut().enumerate() {
                let _id = ui.push_id(i as i32);

                let mut remains = true;
                let name = prop.as_ref();

                let open = CollapsingHeader::new(format!("Condition {}: {name}###cond{i}", i + 1))
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
                if delete_confirm_modal(ui, &title, || ui.text(format!("Delete Condition {name}?")))
                {
                    action = Action::Delete(i);
                }

                if open {
                    threshold.render_options(ui, state);

                    if let Some(_token) =
                        ui.begin_combo_with_flags("Property", prop.as_ref(), ComboBoxFlags::empty())
                    {
                        for entry in self.base.all() {
                            let selected = mem::discriminant(&entry) == mem::discriminant(prop);
                            if Selectable::new(&entry).selected(selected).build(ui) {
                                *prop = entry;
                            }

                            // handle focus
                            if selected {
                                ui.set_item_default_focus();
                            }
                        }
                    }

                    prop.render_options(ui, state);

                    ui.spacing();
                }
            }
            action.perform(&mut self.conditions);

            if ui.button("Add Condition") {
                let new = (
                    ProgressThreshold::default(),
                    <T as Fields>::Field::default(),
                );
                self.conditions.push(new);
            }
        }
    }
}
