use crate::colors::TRANSPARENT;
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
