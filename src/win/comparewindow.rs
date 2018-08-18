use super::{Ids, WindowContents};
use conrod::{color, widget, Colorable, Labelable, Positionable, Sizeable, UiCell, Widget};

pub struct CompareWindow {}

impl WindowContents for CompareWindow {
    fn set_ui(&mut self, ui: &mut UiCell, ids: &mut Ids) {
        widget::Canvas::new()
            .color(color::LIGHT_BLUE)
            .set(ids.background, ui);

        if widget::Button::new()
            .label("Reduplicate!")
            .w_h(320.0, 40.0)
            .mid_bottom_of(ids.background)
            .set(ids.submit, ui)
            .was_clicked()
        {
            println!("Submit 2.0");
        }
    }
}
