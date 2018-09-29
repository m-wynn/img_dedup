use crate::win::radiowidget::{Msg::Clicked, RadioWidget};
use gtk::prelude::*;
use gtk::Orientation::Vertical;
use img_dedup::HashType;
use relm::{connect, connect_stream};
use relm::{Relm, Widget};
use relm_attributes::widget;
use relm_derive::Msg;
use std::path::PathBuf;

use self::Msg::*;

pub struct Model {
    directory: PathBuf,
    hash_size: u32,
}

#[derive(Msg)]
pub enum Msg {
    OpenFileChooser,
    ChangeDirectory(PathBuf),
    ChangeMethod(&'static str),
    ChangeHashLen(u32),
    Deduplicate,
}

#[widget]
impl Widget for ConfigWidget {
    fn model(_relm: &Relm<Self>, (directory, hash_size): (PathBuf, u32)) -> Model {
        Model {
            directory,
            hash_size,
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            ChangeDirectory(directory) => self.model.directory = directory,
            _ => (),
        }
    }

    view! {
        gtk::Box {
            orientation: Vertical,
            gtk::Button {
                clicked => OpenFileChooser,
                label: "Select Folder",
            },
            gtk::Label {
                text: &self.model.directory.to_str().unwrap(),
            },
            RadioWidget(HashType::available_methods()) {
                // I don't really like passing around this as a string.
                Clicked(m) => ChangeMethod(m)
            },
            gtk::Label {
                text: "Hash Length",
            },
            #[name="hashlength"]
            gtk::SpinButton {
                adjustment: &gtk::Adjustment::new(self.model.hash_size.into(), 0., f64::from(::std::u32::MAX), 1., 8., 1.,),
                value_changed(w) => ChangeHashLen(w.get_value_as_int() as u32),
            },
            gtk::Button {
                clicked => Deduplicate,
                label: "Deduplicate!",
            },
        },
    }
}
