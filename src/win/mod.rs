use gtk::prelude::*;
use gtk::Orientation::Vertical;
use gtk::{FileChooserAction, FileChooserDialog};
use log::debug;
use relm::{connect, connect_stream};
use relm::{Channel, Relm, Widget};
use relm_attributes::widget;
use relm_derive::Msg;

mod configwidget;
use configwidget::ConfigWidget;
use configwidget::Msg as ConfigMsg;
use std::path::PathBuf;

pub struct Model {
    text: String,
}

#[derive(Clone, Msg)]
pub enum Msg {
    Quit,
    Value(i32),
    SelectFolder,
}

#[widget]
impl Widget for Win {
    fn model(relm: &Relm<Self>, _: ()) -> Model {
        Model {
            text: "test".to_string(),
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Quit => gtk::main_quit(),
            Msg::Value(num) => self.model.text = num.to_string(),
            Msg::SelectFolder => {
                if let Some(folder) = self.select_folder() {
                    debug!("User selected: {:?}", folder);
                    self.blah.emit(ConfigMsg::ShowSelectedFolder(folder));
                }
            }
        }
    }

    view! {
        #[name="window"]
        gtk::Window {
            title: "Welcome to the window",
            gtk::Box {
                orientation: Vertical,
                #[name="blah"]
                ConfigWidget {
                    OpenFileChooser => Msg::SelectFolder,
                },
            },
            delete_event(_, _) => (Msg::Quit, Inhibit(false)),
        }
    }
}
impl Win {
    fn select_folder(&mut self) -> Option<PathBuf> {
        let dialog = FileChooserDialog::new(
            Some("Select a folder"),
            Some(&self.window),
            FileChooserAction::Open,
        );
        dialog.add_button("Ok", gtk::ResponseType::Ok.into());
        dialog.add_button("Cancel", gtk::ResponseType::Cancel.into());
        // TODO: Add a filter to only allow folders
        let response_ok: i32 = gtk::ResponseType::Ok.into();
        if dialog.run() == response_ok {
            let path = dialog.get_filename();
            dialog.destroy();
            return path;
        }
        dialog.destroy();
        None
    }
}
