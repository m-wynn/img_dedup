use super::{Ids, WindowContents};
use config::Config;
use conrod::backend::glium::glium;
use conrod::{self, color, widget, Colorable, Labelable, Positionable, Sizeable, Widget};
use image;

pub struct WaitWindow {
    spinner: conrod::image::Id,
    spinner_w: f64,
    spinner_h: f64,
}

impl WindowContents for WaitWindow {
    fn set_ui(
        &mut self,
        display: &glium::Display,
        image_map: &mut conrod::image::Map<glium::Texture2d>,
        ui: &mut conrod::UiCell,
        ids: &Ids,
        config: &mut Config,
    ) -> Option<Box<WindowContents>> {
        widget::Canvas::new()
            .color(color::LIGHT_BLUE)
            .set(ids.background, ui);

        widget::Image::new(self.spinner)
            .w_h(self.spinner_w, self.spinner_h)
            .middle()
            .set(ids.spinner, ui);

        None
    }
}

impl WaitWindow {
    fn load_spinner(display: &glium::Display) -> glium::texture::Texture2d {
        let rgba_image = image::load_from_memory(include_bytes!("assets/spinningwheel.gif"))
            .unwrap()
            .to_rgba();
        let image_dimensions = rgba_image.dimensions();
        let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(
            &rgba_image.into_raw(),
            image_dimensions,
        );
        let texture = glium::texture::Texture2d::new(display, raw_image).unwrap();
        texture
    }

    pub fn new(display: &glium::Display, image_map: &mut conrod::image::Map<glium::Texture2d>) -> WaitWindow {
        let spinner_img = Self::load_spinner(&display);
        let (w, h) = (spinner_img.get_width(), spinner_img.get_height().unwrap());

        let spinner = image_map.insert(spinner_img);
        WaitWindow{
            spinner,
            spinner_w: w as f64,
            spinner_h: h as f64
        }
    }
}
