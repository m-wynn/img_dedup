use super::{WindowContents, Ids};
use conrod::{color, widget, Colorable, Positionable, UiCell, Widget};

pub struct WaitWindow {}

impl WindowContents for WaitWindow {
    fn set_ui(&mut self, ui: &mut UiCell, ids: &mut Ids) {
        widget::Canvas::new()
            .color(color::LIGHT_BLUE)
            .set(ids.background, ui);

        widget::Text::new("Working")
            .middle()
            .set(ids.waiting, ui);

    }
}

impl WaitWindow {
    pub fn new() -> WaitWindow {
        WaitWindow {}
    }
}
