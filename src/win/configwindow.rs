use super::{Ids, WindowContents};
use config::Config;
use conrod::{self, color, widget, Colorable, Labelable, Positionable, Sizeable, Widget};

pub struct ConfigWindow {}

impl WindowContents for ConfigWindow {
    fn set_ui(&self, ui: &mut conrod::UiCell, ids: &Ids, config: &Config) {
        let mut sdirectory: String = config.directory.to_str().unwrap().to_owned();
        let method = vec!["mean", "block", "doublegradient", "dct", "gradient"];
        let mut selected_method_index: usize =
            method.iter().position(|&r| r == config.method_str).unwrap();

        widget::Canvas::new()
            .color(color::LIGHT_BLUE)
            .set(ids.background, ui);

        for event in widget::TextBox::new(&sdirectory)
            .color(color::WHITE)
            .text_color(color::BLACK)
            .font_size(20)
            .w_h(320.0, 40.0)
            .middle()
            .set(ids.directory, ui)
        {
            match event {
                widget::text_box::Event::Enter => println!("TextBox: {:?}", sdirectory),
                widget::text_box::Event::Update(string) => sdirectory = string,
            }
        }

        if let Some(i) = widget::DropDownList::new(&method, Some(selected_method_index))
            .color(color::WHITE)
            .w_h(320.0, 40.0)
            .mid_top_of(ids.background)
            .set(ids.method, ui)
        {
            selected_method_index = i;
        }

        if widget::Button::new()
            .label("Deduplicate!")
            .w_h(320.0, 40.0)
            .mid_bottom_of(ids.background)
            .set(ids.submit, ui)
            .was_clicked()
        {
            println!("Submit {} ", selected_method_index);
        }
    }
}
