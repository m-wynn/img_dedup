use gtk::prelude::*;
use gtk::Orientation::Vertical;
use relm::connect;
use relm::{Relm, Update, Widget};
use relm_derive::Msg;

use self::Msg::*;

pub struct Widgets {
    root: gtk::Box,
    _buttons: Vec<gtk::RadioButton>,
}

pub struct RadioWidget {
    widgets: Widgets,
}

pub struct Model {
    list: Vec<(&'static str, &'static str)>,
}

#[derive(Msg)]
pub enum Msg {
    Clicked(&'static str),
}

impl Update for RadioWidget {
    // Specify the model used for this widget.
    type Model = Model;
    // Specify the model parameter used to init the model.
    type ModelParam = Vec<(&'static str, &'static str)>;
    // Specify the type of the messages sent to the update function.
    type Msg = Msg;

    fn model(_relm: &Relm<Self>, list: Vec<(&'static str, &'static str)>) -> Model {
        Model { list }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Clicked(_) => (),
        }
    }
}

impl Widget for RadioWidget {
    type Root = gtk::Box;

    // Return the root widget.
    fn root(&self) -> Self::Root {
        self.widgets.root.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        let vbox = gtk::Box::new(Vertical, 0);

        let mut buttons: Vec<gtk::RadioButton> = Vec::new();

        for (text, desc) in model.list.into_iter() {
            let radio_button = match buttons.get(0) {
                Some(old_button) => gtk::RadioButton::new_with_label_from_widget(old_button, text),
                None => gtk::RadioButton::new_with_label(text),
            };
            radio_button.set_tooltip_text(Some(desc));
            vbox.add(&radio_button);
            connect!(relm, radio_button, connect_clicked(_), Msg::Clicked(text));
            buttons.push(radio_button);
        }

        vbox.show_all();

        RadioWidget {
            widgets: Widgets {
                root: vbox,
                _buttons: buttons,
            },
        }
    }
}
