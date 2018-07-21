use super::{WindowContents, WindowContext};
use conrod::{color, widget, Colorable, Labelable, Positionable, Sizeable, UiCell, Widget};

pub struct CompareWindow {}

impl WindowContents for CompareWindow {
    fn set_ui(
        &mut self,
        win: &mut WindowContext,
        ui: &mut UiCell,
    ) -> Option<Box<WindowContents>> {
        widget::Canvas::new()
            .color(color::LIGHT_BLUE)
            .set(win.ids.background, ui);

        if widget::Button::new()
            .label("Reduplicate!")
            .w_h(320.0, 40.0)
            .mid_bottom_of(win.ids.background)
            .set(win.ids.submit, ui)
            .was_clicked()
        {
            println!("Submit 2.0");
        }
        None
    }
}
