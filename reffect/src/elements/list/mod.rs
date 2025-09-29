mod action;
mod icon;
mod layout;

pub use self::{action::*, icon::*, layout::*};

use super::{Direction, RenderCtx};
use crate::{
    action::DynAction,
    clipboard::Clipboard,
    colors,
    context::Context,
    elements::Common,
    render::{
        Bounds, Rect, collapsing_header_same_line_end, delete_confirm_modal, enum_combo,
        input_float_with_format, input_size, item_context_menu, style_disabled_if,
    },
    tree::TreeNode,
};
use const_default::ConstDefault;
use nexus::imgui::{
    self as ig, CollapsingHeader, ComboBoxFlags, InputTextFlags, MenuItem, StyleColor,
    TreeNodeFlags, Ui,
};
use serde::{Deserialize, Serialize};

// TODO: wrapping, sorting options

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct IconList {
    pub layout: Layout,
    pub direction: Direction,
    pub size: [f32; 2],
    pub pad: f32,
    pub icons: Vec<ListIcon>,
}

// technically there is children, but they are not full elements
impl TreeNode for IconList {}

impl IconList {
    pub fn render(&mut self, ui: &Ui, ctx: &RenderCtx, common: &Common) {
        let render_icon = |icon: &mut ListIcon, i, len| {
            let offset = self.direction.list_item_offset(self.size, self.pad, i, len);
            let _token = ctx.push_offset(offset);
            icon.render(ui, ctx, self.size);
        };

        let parent_active = common.trigger.active();

        match self.layout {
            Layout::Dynamic => {
                let filtered = self
                    .icons
                    .iter_mut()
                    .filter_map(|icon| {
                        let visible = icon.is_visible(ctx) && icon.update(ctx, parent_active);
                        visible.then_some(icon)
                    })
                    .collect::<Vec<_>>();
                let len = filtered.len();

                for (i, icon) in filtered.into_iter().enumerate() {
                    render_icon(icon, i, len);
                }
            }
            Layout::Static => {
                let len = self.icons.len();
                for (i, icon) in self.icons.iter_mut().enumerate() {
                    if icon.is_visible(ctx) && icon.update(ctx, parent_active) {
                        render_icon(icon, i, len);
                    }
                }
            }
        };
    }

    pub fn render_options(&mut self, ui: &Ui, ctx: &RenderCtx) {
        enum_combo(ui, "Layout", &mut self.layout, ComboBoxFlags::empty());

        enum_combo(ui, "Direction", &mut self.direction, ComboBoxFlags::empty());

        input_size(&mut self.size);

        input_float_with_format(
            "Spacing",
            &mut self.pad,
            1.0,
            10.0,
            "%.2f",
            InputTextFlags::empty(),
        );

        ui.spacing();
        ui.text_disabled("Icons");

        let mut action = IconAction::new();
        let mut copy_action = DynAction::<ListIcon>::empty();
        for (i, list_icon) in self.icons.iter_mut().enumerate() {
            let _id = ui.push_id(i as i32);

            let mut remains = true;
            let style = style_disabled_if(ui, !list_icon.enabled);
            let open = CollapsingHeader::new(format!("{}###icon{i}", list_icon.name))
                .flags(TreeNodeFlags::ALLOW_ITEM_OVERLAP)
                .begin_with_close_button(ui, &mut remains);

            let size_x = ui.frame_height();
            let [spacing_x, _] = ui.clone_style().item_spacing;
            collapsing_header_same_line_end(ui, 3.0 * size_x + 2.0 * spacing_x);

            item_context_menu("##listiconctx", || {
                if MenuItem::new("Paste")
                    .enabled(Clipboard::has_icon())
                    .build(ui)
                {
                    action = IconAction::Paste(i)
                }
                if MenuItem::new("Cut").build(ui) {
                    action = IconAction::Cut(i);
                }
                if MenuItem::new("Copy").build(ui) {
                    Clipboard::set(list_icon.clone().into_element(self.size))
                }
                if MenuItem::new("Duplicate").build(ui) {
                    action = IconAction::Duplicate(i);
                }
                if MenuItem::new("Move Up").build(ui) {
                    action = IconAction::Up(i);
                }
                if MenuItem::new("Move Down").build(ui) {
                    action = IconAction::Down(i);
                }
                let _color = ui.push_style_color(StyleColor::HeaderHovered, colors::DELETE_HOVER);
                if MenuItem::new("Delete").build(ui) {
                    remains = false;
                }
            });

            {
                let _style = ui.push_style_color(StyleColor::Button, colors::TRANSPARENT);
                if ui.arrow_button("up", ig::Direction::Up) {
                    action = IconAction::Up(i);
                }

                ui.same_line();
                if ui.arrow_button("down", ig::Direction::Down) {
                    action = IconAction::Down(i);
                }
            }

            let title = format!("Confirm Delete##reffectlisticon{i}");
            if !remains {
                ui.open_popup(&title);
            }
            if delete_confirm_modal(ui, &title, || {
                ui.text(format!("Delete Icon {}?", list_icon.name))
            }) {
                action = IconAction::Delete(i);
            }

            drop(style);
            if open {
                copy_action.or(list_icon.render_options(ui, ctx));
                ui.spacing();
            }
        }
        if ui.button("Add Icon") {
            self.icons.push(ListIcon::default());
        }
        item_context_menu("##addiconctx", || {
            if MenuItem::new("Paste")
                .enabled(Clipboard::has_icon())
                .build(ui)
            {
                action = IconAction::Paste(self.icons.len());
            }
        });

        action.perform(&mut self.icons, self.size);
        copy_action.apply_to_all(&mut self.icons);
    }

    pub fn render_tabs(&mut self, ui: &Ui, ctx: &Context, common: &Common) {
        if let Some(_token) = ui.tab_item("Condition") {
            const INDENT: f32 = 10.0;
            let mut action = DynAction::empty();

            for (i, list_icon) in self.icons.iter_mut().enumerate() {
                let _id = ui.push_id(i as i32);
                let open = CollapsingHeader::new(format!("{}###icon{i}", list_icon.name))
                    .flags(TreeNodeFlags::ALLOW_ITEM_OVERLAP)
                    .begin(ui);
                if open {
                    ui.indent_by(INDENT);
                    action.or(list_icon.icon.props.render_condition_options(
                        ui,
                        ctx,
                        &common.trigger.source,
                    ));
                    ui.unindent_by(INDENT);
                }
            }

            action.apply_to_all(
                self.icons
                    .iter_mut()
                    .map(|list_icon| &mut list_icon.icon.props),
            );
        }
    }

    pub fn render_filters(&mut self, ui: &Ui, ctx: &Context) {
        ui.spacing();
        for (i, icon) in self.icons.iter_mut().enumerate() {
            let _id = ui.push_id(i as i32);
            let open = CollapsingHeader::new(format!("{}###icon{i}", icon.name))
                .flags(TreeNodeFlags::ALLOW_ITEM_OVERLAP)
                .begin(ui);
            if open {
                icon.filter.render_options(ui, ctx);
            }
        }
    }

    pub fn render_debug(&mut self, ui: &Ui, ctx: &RenderCtx) {
        ui.text(format!("Icons: {}", self.icons.len()));
        for (i, icon) in self.icons.iter_mut().enumerate() {
            if CollapsingHeader::new(format!("{}###icon{i}", icon.name))
                .flags(TreeNodeFlags::ALLOW_ITEM_OVERLAP)
                .begin(ui)
            {
                icon.render_debug(ui, ctx);
            }
        }
    }
}

impl Bounds for IconList {
    fn bounds(&self, _ui: &Ui, _ctx: &Context) -> Rect {
        // calculate with all visible and at least 1 dummy
        let len = match self.layout {
            Layout::Dynamic => self
                .icons
                .iter()
                .filter(|icon| icon.enabled && icon.trigger.active().is_some())
                .count(),
            Layout::Static => self.icons.len(),
        };
        self.direction
            .icon_list_bounds(self.size, self.pad, len.max(1))
    }
}

impl ConstDefault for IconList {
    const DEFAULT: Self = Self {
        layout: Layout::Dynamic,
        direction: Direction::Right,
        size: [32.0, 32.0],
        pad: 2.0,
        icons: Vec::new(),
    };
}

impl Default for IconList {
    fn default() -> Self {
        Self::DEFAULT
    }
}
