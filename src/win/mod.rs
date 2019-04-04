use gtk::prelude::*;
use gtk::{FileChooserAction, FileChooserDialog, Orientation::Vertical};
use log::{debug, info};
use relm::{connect, connect_stream, Channel, EventStream, Relm, Widget};
use relm_attributes::widget;
use relm_derive::Msg;

mod comparewidget;
mod configwidget;
mod radiowidget;
mod waitwidget;

use self::comparewidget::{CompareWidget, Msg as CompareMsg};
use self::configwidget::{ConfigWidget, Msg as ConfigMsg, Msg::*};
use self::waitwidget::{Msg as WaitMsg, WaitWidget};
use img_dedup::{self as scanner, Config, SimilarPair, StatusMsg};
use std::collections::BinaryHeap;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::thread;

/// An unsafe container for my Binary Heap.
///
/// I want to create this off of the main thread
/// At the same time, I want this to contain `Rc`'s, which are not `Send`
/// So I'm unsafe-impl-ing `Send`.
/// I promise the creator does not have any references.
///
/// I did test this

pub struct ScannedFiles(BinaryHeap<SimilarPair>);
unsafe impl Send for ScannedFiles {}

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
    Done(ScannedFiles),
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
            Msg::Done(ScannedFiles(files)) => {
                debug!("{:#?}", files);
                self.compare.emit(CompareMsg::SetFiles(files));
                self.stack.set_visible_child(self.compare.widget());
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
                    WaitWidget() { },
                    #[name="compare"]
                    CompareWidget(BinaryHeap::<SimilarPair>::new()) { },
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

    fn run_scanner(&self) {
        let (sender, receiver) = channel::<StatusMsg>();
        let directory = self.model.config.directory.clone();
        let method = self.model.config.method.clone();
        let hash_size = self.model.config.hash_size;

        let stream = self.model.stream.clone();
        // Spawn a thread to scan the files
        thread::spawn(move || {
            let files =
                ScannedFiles(scanner::scan_files(&directory, method, hash_size, sender).unwrap());

            stream.emit(Msg::Done(files));
            info!("Done");
        });
        let wait_stream = self.wait.stream().clone();
        // Relm's channels wake up the thread
        let (_channel, relm_sender) = Channel::new(move |msg| match msg {
            StatusMsg::Total(i) => wait_stream.emit(WaitMsg::UpdateTotal(i)),
            StatusMsg::ImageProcessed => wait_stream.emit(WaitMsg::UpdateProcessed),
        });
        // Forward mpsc messages to relm messages
        thread::spawn(move || receiver.iter().for_each(|r| relm_sender.send(r).unwrap()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::sync::mpsc::channel;

    #[test]
    fn scanned_files_is_safe() {
        let (sender, _receiver) = channel::<StatusMsg>();
        let directory = PathBuf::from("./test");
        let config = Config::default();
        let files = ScannedFiles(
            scanner::scan_files(&directory, config.method, config.hash_size, sender).unwrap(),
        );
        files
            .0
            .into_iter()
            .for_each(|x| println!("{:?} > {:?}", x.left.borrow_mut(), x.right.borrow_mut()));
    }
}
