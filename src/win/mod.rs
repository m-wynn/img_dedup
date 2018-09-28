use gtk::prelude::*;
use gtk::Orientation::Vertical;
use gtk::{FileChooserAction, FileChooserDialog};
use log::debug;
use relm::{connect, connect_stream};
use relm::{Relm, Widget};
use relm_attributes::widget;
use relm_derive::Msg;

mod configwidget;
mod radiowidget;

use configwidget::ConfigWidget;
use configwidget::Msg as ConfigMsg;
use configwidget::Msg::{ChangeHashLen, ChangeMethod, Deduplicate, OpenFileChooser};
use img_dedup::Config;
use std::path::PathBuf;

pub struct Model {
    config: Config,
}

#[derive(Msg)]
pub enum Msg {
    Deduplicate,
    SelectFolder,
    ChangeHashLen(u32),
    ChangeMethod(&'static str),
    Quit,
}

#[widget]
impl Widget for Win {
    fn model(_relm: &Relm<Self>, config: Config) -> Model {
        Model { config: config }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Quit => gtk::main_quit(),
            Msg::SelectFolder => {
                if let Some(directory) = self.select_directory() {
                    self.config
                        .emit(ConfigMsg::ChangeDirectory(directory.clone()));
                    self.model.config.directory = directory;
                }
            }
            Msg::ChangeHashLen(len) => self.model.config.hash_length = len,
            Msg::ChangeMethod(method) => self.model.config.method = method.parse().unwrap(),
            Msg::Deduplicate => {
                println!("{:?}", self.model.config);
            }
        }
    }

    view! {
        #[name="window"]
        gtk::Window {
            title: "Image Deduplicator",
            gtk::Box {
                orientation: Vertical,
                #[name="config"]
                // I would love to be able to pass in a reference to the config object itself
                ConfigWidget(self.model.config.directory.clone(), self.model.config.hash_length.clone()) {
                    OpenFileChooser => Msg::SelectFolder,
                    Deduplicate => Msg::Deduplicate,
                    ChangeHashLen(len) => Msg::ChangeHashLen(len),
                    ChangeMethod(m) => Msg::ChangeMethod(m),
                },
            },
            delete_event(_, _) => (Msg::Quit, Inhibit(false)),
        }
    }
}
impl Win {
    fn select_directory(&mut self) -> Option<PathBuf> {
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
