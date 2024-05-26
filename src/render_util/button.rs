use crate::{colors::TRANSPARENT, component_wise::ComponentWise};
use nexus::imgui::{ColorStackToken, StyleColor, StyleStackToken, StyleVar, Ui};

pub struct ButtonDisabledToken<'ui> {
    _alpha: StyleStackToken<'ui>,
    _color: ColorStackToken<'ui>,
    _hover_color: ColorStackToken<'ui>,
    _active_color: ColorStackToken<'ui>,
}

pub fn button_disabled<'ui>(ui: &'ui Ui, enabled: bool) -> Option<ButtonDisabledToken<'ui>> {
    (!enabled).then(|| ButtonDisabledToken {
        _alpha: ui.push_style_var(StyleVar::Alpha(0.5)),
        _color: ui.push_style_color(StyleColor::Button, TRANSPARENT),
        _hover_color: ui.push_style_color(StyleColor::ButtonHovered, TRANSPARENT),
        _active_color: ui.push_style_color(StyleColor::ButtonActive, TRANSPARENT),
    })
}

pub fn close_button(ui: &Ui, id_label: impl AsRef<str>) -> bool {
    let size = ui.frame_height();
    let clicked = ui.button_with_size(id_label, [size, size]);

    let min = ui.item_rect_min();
    let center = min.add([0.5 * size, 0.5 * size]);
    let len = 0.35 * size - 1.0;
    let first = [len, len];
    let second = [len, -len];
    let color = ui.style_color(StyleColor::Text);

    let draw_list = ui.get_window_draw_list();
    draw_list
        .add_line(center.add(first), center.sub(first), color)
        .build();
    draw_list
        .add_line(center.add(second), center.sub(second), color)
        .build();

    clicked
}
