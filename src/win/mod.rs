use gtk::prelude::*;
use gtk::Orientation::Vertical;
use gtk::{FileChooserAction, FileChooserDialog};
use log::info;
use relm::{connect, connect_stream};
use relm::{Relm, Widget};
use relm_attributes::widget;
use relm_core::{Channel, Sender};
use relm_derive::Msg;
use std::sync::{atomic::AtomicU32, Arc};

mod configwidget;
mod radiowidget;
mod waitwidget;

use configwidget::Msg::*;
use configwidget::{ConfigWidget, Msg as ConfigMsg};
use img_dedup::{self as scanner, Config};
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use waitwidget::{Msg as WaitMsg, WaitWidget};

pub struct Model {
    config: Config,
    _channel: Channel<u32>,
    sender: Sender<u32>,
    total: Arc<AtomicU32>,
    processed: Arc<AtomicU32>,
}

#[derive(Msg)]
pub enum Msg {
    Deduplicate,
    SelectFolder,
    ChangeHashLen(u32),
    ChangeMethod(&'static str),
    UpdateNumber(u32),
    Quit,
}

#[widget]
impl Widget for Win {
    fn model(relm: &Relm<Self>, config: Config) -> Model {
        let stream = relm.stream().clone();
        let (channel, sender) = Channel::new(move |num| {
            // This closure is executed whenever a message is received from the sender.
            // We send a message to the current widget.
            stream.emit(Msg::UpdateNumber(num));
        });
        let total = Arc::new(AtomicU32::new(0));
        let processed = Arc::new(AtomicU32::new(0));

        let p = Arc::clone(&processed);
        let t = Arc::clone(&total);

        thread::spawn(move || loop {
            thread::sleep(Duration::from_millis(200));
            info!("Yoo we're at {:?} / {:?}", p, t);
        });

        Model {
            config: config,
            _channel: channel,
            sender: sender,
            total: total,
            processed: processed,
        }
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
            Msg::UpdateNumber(i) => self.wait.emit(WaitMsg::UpdateNumber(i)),
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
                    WaitWidget(32) {}
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
        // let sender = self.model.sender.clone();
        let total = Arc::clone(&self.model.total);
        let processed = Arc::clone(&self.model.processed);
        let directory = self.model.config.directory.clone();
        let method = self.model.config.method.clone();
        let hash_size = self.model.config.hash_size;
        thread::spawn(move || {
            let files = scanner::scan_files(
                directory,
                method,
                hash_size,
                &Arc::clone(&total),
                Arc::clone(&processed),
            )
            .unwrap();
            // sender.send(i).expect("send message");
            info!("Done");
        });
    }
}
