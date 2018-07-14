use super::{Ids, WindowContents};
use config::Config;
use conrod::backend::glium::glium;
use conrod::{self, color, widget, Colorable, Labelable, Positionable, Sizeable, Widget};

pub struct CompareWindow {}

impl WindowContents for CompareWindow {
    fn set_ui(
        &mut self,
        display: &glium::Display,
        image_map: &mut conrod::image::Map<glium::Texture2d>,
        ui: &mut conrod::UiCell,
        ids: &Ids,
        config: &mut Config,
    ) -> Option<Box<WindowContents>> {
        widget::Canvas::new().color(color::LIGHT_BLUE).set(
            ids.background,
            ui,
        );

        if widget::Button::new()
            .label("Reduplicate!")
            .w_h(320.0, 40.0)
            .mid_bottom_of(ids.background)
            .set(ids.submit, ui)
            .was_clicked()
        {
            println!("Submit 2.0");
        }
        None
    }
}
