use super::{Ids, WindowContents};
use conrod::{color, widget, Colorable, Labelable, Positionable, Sizeable, UiCell, Widget};
use crate::runner::ThreadMsg;
use img_dedup::SimilarPair;
use std::collections::BinaryHeap;
use std::sync::mpsc::Sender;

pub struct CompareWindow {
    current: SimilarPair,
    duplicates: BinaryHeap<SimilarPair>,
    tx: Sender<ThreadMsg>,
}

impl WindowContents for CompareWindow {
    fn set_ui(&mut self, ui: &mut UiCell, ids: &mut Ids) {
        widget::Canvas::new()
            .color(color::LIGHT_BLUE)
            .set(ids.background, ui);

        if widget::Button::new()
            .label("Next")
            .w_h(320.0, 40.0)
            .mid_bottom_of(ids.background)
            .set(ids.submit, ui)
            .was_clicked()
        {
            match self.duplicates.pop() {
                Some(similar_pair) => self.current = similar_pair,
                None => self.tx.send(ThreadMsg::CompareDone()).unwrap(),
            };
        }
    }
}

impl CompareWindow {
    pub fn new(
        mut duplicates: BinaryHeap<SimilarPair>,
        tx: Sender<ThreadMsg>,
    ) -> Option<CompareWindow> {
        match duplicates.pop() {
            Some(current) => Some(CompareWindow {
                current,
                duplicates,
                tx,
            }),
            None => None,
        }
    }
}
