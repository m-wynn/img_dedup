use gtk::prelude::*;
use gtk::Orientation::Vertical;
use gtk::{FileChooserAction, FileChooserDialog};
use log::{debug, info};
use relm::{connect, connect_stream};
use relm::{EventStream, Relm, Widget};
use relm_attributes::widget;
use relm_core::Channel;
use relm_derive::Msg;

mod configwidget;
mod radiowidget;
mod waitwidget;

use configwidget::Msg::*;
use configwidget::{ConfigWidget, Msg as ConfigMsg};
use img_dedup::{self as scanner, Config, SimilarPair, StatusMsg};
use std::collections::BinaryHeap;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::thread;
use waitwidget::{Msg as WaitMsg, WaitWidget};

pub struct Model {
    config: Config,
    stream: EventStream<Msg>,
}

#[derive(Msg)]
pub enum Msg {
    Deduplicate,
    SelectFolder,
    ChangeHashLen(u32),
    ChangeMethod(&'static str),
    Done(BinaryHeap<SimilarPair>),
    Quit,
}

#[widget]
impl Widget for Win {
    fn model(relm: &Relm<Self>, config: Config) -> Model {
        let stream = relm.stream().clone();
        Model { config, stream }
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
            Msg::ChangeHashLen(len) => self.model.config.hash_size = len,
            Msg::ChangeMethod(method) => self.model.config.method = method.parse().unwrap(),
            Msg::Deduplicate => {
                info!("{:#?}", self.model.config);
                self.stack.set_visible_child(self.wait.widget());
                self.run_scanner();
            }
            Msg::Done(files) => {
                info!("{:#?}", files);
            }
        }
    }

    view! {
        #[name="window"]
        gtk::Window {
            title: "Image Deduplicator",
            gtk::Box {
                orientation: Vertical,
                #[name="stack"]
                gtk::Stack {
                    #[name="config"]
                    // I would love to be able to pass in a reference to the config object itself
                    ConfigWidget(self.model.config.directory.clone(), self.model.config.hash_size) {
                        OpenFileChooser => Msg::SelectFolder,
                        Deduplicate => Msg::Deduplicate,
                        ChangeHashLen(len) => Msg::ChangeHashLen(len),
                        ChangeMethod(m) => Msg::ChangeMethod(m),
                    },
                    #[name="wait"]
                    WaitWidget() {
                    }
                }
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
            FileChooserAction::SelectFolder,
        );
        dialog.add_button("Ok", gtk::ResponseType::Ok.into());
        dialog.add_button("Cancel", gtk::ResponseType::Cancel.into());
        let response_ok: i32 = gtk::ResponseType::Ok.into();
        if dialog.run() == response_ok {
            let path = dialog.get_filename();
            dialog.destroy();
            return path;
        }
        dialog.destroy();
        None
    }

    fn run_scanner(&self) -> () {
        let (sender, receiver) = channel::<StatusMsg>();
        let directory = self.model.config.directory.clone();
        let method = self.model.config.method.clone();
        let hash_size = self.model.config.hash_size;

        let stream = self.model.stream.clone();
        // Spawn a thread to scan the files
        thread::spawn(move || {
            let files = scanner::scan_files(directory, method, hash_size, sender).unwrap();
            // Todo: Skip the wait widget and write directly to this widget
            stream.emit(Msg::Done(files));
            info!("Done");
        });
        let wait_stream = self.wait.stream().clone();
        let (_channel, sender) = Channel::new(move |msg| match msg {
            StatusMsg::Total(i) => wait_stream.emit(WaitMsg::UpdateTotal(i)),
            StatusMsg::ImageProcessed => wait_stream.emit(WaitMsg::UpdateProcessed),
        });
        thread::spawn(move || {
            for r in receiver.iter() {
                sender.send(r).unwrap();
                debug!("Message: {:?}", r)
            }
        });
    }
}
