use super::{WindowContents, WindowContext};
use conrod::{color, widget, Colorable, Positionable, UiCell, Widget};

pub struct WaitWindow {}

impl WindowContents for WaitWindow {
    fn set_ui(&mut self, win: &mut WindowContext, ui: &mut UiCell) -> Option<Box<WindowContents>> {
        widget::Canvas::new()
            .color(color::LIGHT_BLUE)
            .set(win.ids.background, ui);

        widget::Text::new("Working")
            .middle()
            .set(win.ids.waiting, ui);

        None
    }
}

impl WaitWindow {
    pub fn new() -> WaitWindow {
        WaitWindow {}
    }
}
