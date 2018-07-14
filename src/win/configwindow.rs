use super::waitwindow::WaitWindow;
use super::{Ids, WindowContents};
use config::Config;
use conrod::backend::glium::glium;
use conrod::{self, color, widget, Colorable, Labelable, Positionable, Sizeable, Widget};
use hash_type::HashType;

pub struct ConfigWindow {
    directory: String,
    methods: Vec<&'static str>,
    selected_method_index: usize,
}

impl WindowContents for ConfigWindow {
    /*
     * Returns whether or not to step forward
     */
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

        for event in widget::TextBox::new(&self.directory)
            .color(color::WHITE)
            .text_color(color::BLACK)
            .font_size(20)
            .w_h(320.0, 40.0)
            .middle()
            .set(ids.directory, ui)
        {
            match event {
                widget::text_box::Event::Enter => println!("TextBox: {:?}", self.directory),
                widget::text_box::Event::Update(string) => self.directory = string,
            }
        }

        if let Some(i) = widget::DropDownList::new(&self.methods, Some(self.selected_method_index))
            .color(color::WHITE)
            .w_h(320.0, 40.0)
            .mid_top_of(ids.background)
            .set(ids.method, ui)
        {
            self.selected_method_index = i;
        }

        if widget::Button::new()
            .label("Deduplicate!")
            .w_h(320.0, 40.0)
            .mid_bottom_of(ids.background)
            .set(ids.submit, ui)
            .was_clicked()
        {
            config.set_directory(&self.directory);
            config.set_method(self.methods[self.selected_method_index].parse().unwrap());
            return Some(Box::new(WaitWindow::new(display, image_map)));
        }

        None
    }
}

impl ConfigWindow {
    pub fn new(config: &Config) -> ConfigWindow {
        let directory: String = config.directory.to_str().unwrap().to_owned();
        let methods: Vec<&'static str> = HashType::available_methods()
            .iter()
            .map(|(name, _)| *name)
            .collect();
        let selected_method_index: usize = methods
            .iter()
            .position(|&r| r == config.method.to_string())
            .unwrap();
        ConfigWindow {
            directory,
            methods,
            selected_method_index,
        }
    }
}
