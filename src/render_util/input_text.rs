use arc_util::ui::render::item_context_menu;
use nexus::imgui::{
    InputTextCallbackHandler, InputTextFlags, InputTextMultilineCallback, MenuItem, Ui,
};
use std::{cell::Cell, mem, ops::Range};

/// Renders a simple context menu for text inputs.
///
/// Cut/copy/paste will use the entire text.
pub fn input_text_simple_menu(ui: &Ui, id: impl Into<String>, text: &mut String) {
    item_context_menu(id, || {
        if MenuItem::new("Cut").build(ui) {
            ui.set_clipboard_text(mem::take(text));
        }

        if MenuItem::new("Copy").build(ui) {
            ui.set_clipboard_text(text.clone());
        }

        let clipboard = ui.clipboard_text();
        if MenuItem::new("Paste")
            .enabled(clipboard.is_some())
            .build(ui)
        {
            *text = clipboard.expect("paste without clipboard text");
        }
    });
}

pub fn input_text_multi_with_menu(
    ui: &Ui,
    label: impl AsRef<str>,
    text: &mut String,
    size: [f32; 2],
    flags: InputTextFlags,
) -> bool {
    MENU.with(|menu| {
        let label = label.as_ref();
        let changed = ui
            .input_text_multiline(label, text, size)
            .flags(flags | InputTextFlags::CALLBACK_RESIZE)
            .callback(InputTextMultilineCallback::ALWAYS, menu)
            .build();
        menu.render_context_menu(ui, format!("##{label}ctx"), text);
        changed
    })
}

// single instance is sufficient, callback is only called for focused input
thread_local! { static MENU: InputTextContextMenu = const { InputTextContextMenu::new() }; }

#[derive(Debug, Default)]
pub struct InputTextContextMenu {
    start: Cell<usize>,
    end: Cell<usize>,
}

impl InputTextContextMenu {
    pub const fn new() -> Self {
        Self {
            start: Cell::new(0),
            end: Cell::new(0),
        }
    }

    pub fn range(&self, text: &str) -> Range<usize> {
        let start = self.start.get();
        let end = self.end.get();
        if start == 0 && end == 0 {
            0..text.len()
        } else {
            start..end
        }
    }

    pub fn render_context_menu(&self, ui: &Ui, id: impl Into<String>, text: &mut String) {
        item_context_menu(id, || {
            if MenuItem::new("Cut").build(ui) {
                let selected = text.drain(self.range(text)).collect::<String>();
                ui.set_clipboard_text(selected);
            }

            if MenuItem::new("Copy").build(ui) {
                let selected = text.get(self.range(text)).expect("invalid range for copy");
                ui.set_clipboard_text(selected);
            }

            let clipboard = ui.clipboard_text();
            if MenuItem::new("Paste")
                .enabled(clipboard.is_some())
                .build(ui)
            {
                text.replace_range(
                    self.range(text),
                    &clipboard.expect("paste without clipboard text"),
                );
            }
        });
    }
}

impl InputTextCallbackHandler for &InputTextContextMenu {
    fn on_always(&mut self, data: nexus::imgui::TextCallbackData) {
        let Range { mut start, mut end } = data.selection();
        if start > end {
            mem::swap(&mut start, &mut end);
        }
        self.start.set(start);
        self.end.set(end);
    }
}
