use gtk::prelude::*;
use gtk::Orientation::Vertical;
use relm::{connect, connect_stream};
use relm::{Channel, Relm, Widget};
use relm_attributes::widget;
use relm_derive::Msg;
use std::path::PathBuf;

use self::Msg::*;

pub struct Model {
    folder: PathBuf,
}

#[derive(Clone, Msg)]
pub enum Msg {
    OpenFileChooser,
    ShowSelectedFolder(PathBuf),
}

#[widget]
impl Widget for ConfigWidget {
    fn model(relm: &Relm<Self>, _: ()) -> Model {
        Model {
            folder: PathBuf::from("./"),
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            ShowSelectedFolder(folder) => self.model.folder = folder,
            OpenFileChooser => (),
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
                text: &self.model.folder.to_str().unwrap(),
            },
        },
    }
}

impl ConfigWidget {}
