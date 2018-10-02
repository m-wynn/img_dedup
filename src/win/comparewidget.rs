use self::Msg::*;
use gtk::prelude::*;
use gtk::Orientation::Vertical;
use img_dedup::SimilarPair;
use log::debug;
use relm::{connect, Relm, Widget};
use relm_attributes::widget;
use relm_derive::Msg;
use std::collections::BinaryHeap;

pub struct Model {
    files: BinaryHeap<SimilarPair>,
    current_pair: Option<SimilarPair>,
}

#[derive(Msg)]
pub enum Msg {
    SetFiles(BinaryHeap<SimilarPair>),
    Next,
}

#[widget]
impl Widget for CompareWidget {
    fn model(_relm: &Relm<Self>, files: BinaryHeap<SimilarPair>) -> Model {
        Model {
            files,
            current_pair: None,
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            SetFiles(files) => {
                self.model.files = files;
                self.next();
            }
            Next => self.next(),
        };
    }

    view! {
        gtk::Box {
            orientation: Vertical,
            gtk::Label {
                text: "helo",
            },
            #[name="leftimage"]
            gtk::Image {
            },
            #[name="rightimage"]
            gtk::Image {
            },
            #[name="nextbutton"]
            gtk::Button {
                clicked => Next,
                label: "Next Pair",
            },
        },
    }
}

// Maybe more of this should be moved to the library
impl CompareWidget {
    fn next(&mut self) {
        self.model.current_pair = self.model.files.pop();
        if let Some(pair) = &self.model.current_pair {
            debug!("{:?}", pair);
            self.leftimage.set_from_file(&pair.left.path);
            self.rightimage.set_from_file(&pair.right.path);
        }
    }
}
