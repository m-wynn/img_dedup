use gtk::prelude::*;
use gtk::Orientation::Vertical;
use relm::{connect, connect_stream};
use relm::{Relm, Widget};
use relm_attributes::widget;
use relm_derive::Msg;

use self::Msg::*;

pub struct Model {
    number: u32,
}

#[derive(Msg)]
pub enum Msg {
    UpdateNumber(u32),
}

#[widget]
impl Widget for WaitWidget {
    fn model(_relm: &Relm<Self>, number: u32) -> Model {
        Model { number }
    }

    fn update(&mut self, event: Msg) {
        match event {
            UpdateNumber(i) => self.model.number = i,
            _ => (),
        }
    }

    view! {
        gtk::Box {
            orientation: Vertical,
            gtk::Label {
                text: &self.model.number.to_string(),
            },
        },
    }
}
