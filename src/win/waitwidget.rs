use self::Msg::*;
use gtk::prelude::*;
use gtk::Orientation::Vertical;
use relm::{Relm, Widget};
use relm_attributes::widget;
use relm_derive::Msg;

pub struct Model {
    total: usize,
    processed: usize,
    text: String,
    fraction: f64,
}

#[derive(Msg)]
pub enum Msg {
    UpdateTotal(usize),
    UpdateProcessed,
}

#[widget]
impl Widget for WaitWidget {
    fn model(_relm: &Relm<Self>, _: ()) -> Model {
        Model {
            processed: 0,
            total: 0,
            fraction: 0.0,
            text: String::from("Searching for image files..."),
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            UpdateTotal(i) => self.model.total = i,
            UpdateProcessed => self.model.processed += 1,
        };
        if self.model.total > 0 && self.model.total == self.model.processed {
            self.model.text = format!("Calculating differences for {} hashes", self.model.total);
            self.model.fraction = 1.0;
        } else {
            self.model.fraction = self.model.processed as f64 / self.model.total as f64;
            // TODO: Estimate time left?
            self.model.text = format!(
                "Processing files {} / {}",
                self.model.processed, self.model.total
            );
        }
    }

    view! {
        gtk::Box {
            orientation: Vertical,
            gtk::Label {
                text: &self.model.text,
            },
            gtk::ProgressBar {
                fraction: self.model.fraction,
                show_text: true,
            },
        },
    }
}
