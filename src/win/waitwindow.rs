use super::{Ids, WindowContents};
use conrod::{color, widget, Colorable, Positionable, UiCell, Widget};
use std::sync::{
    atomic::{AtomicU32, Ordering},
    Arc,
};

pub struct WaitWindow {
    total: Arc<AtomicU32>,
    processed: Arc<AtomicU32>,
}

impl WindowContents for WaitWindow {
    fn set_ui(&mut self, ui: &mut UiCell, ids: &mut Ids) {
        widget::Canvas::new()
            .color(color::LIGHT_BLUE)
            .set(ids.background, ui);

        widget::Text::new(&format!(
            "Processing: {} / {}",
            self.processed.load(Ordering::Relaxed),
            self.total.load(Ordering::Relaxed)
        )).middle()
        .set(ids.waiting, ui);
    }
}

impl WaitWindow {
    pub fn new(total: Arc<AtomicU32>, processed: Arc<AtomicU32>) -> WaitWindow {
        WaitWindow { total, processed }
    }
}
