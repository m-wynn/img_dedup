use crate::config::Config;
use crate::win::*;
use failure::Error;
use log::{debug, log};
use std::sync::{mpsc::channel, Arc, Mutex};
use std::thread;

/// Holds all the main logic and messaging between the workers and the GUI
pub struct Runner {}

impl Runner {
    pub fn new() -> Runner {
        Runner {}
    }

    pub fn run(self, config: Config) -> Result<(), Error> {
        // We need to provide the GUI with the WindowContent objects
        // which provide the widgets and the handlers around them
        // Everything else we can stick in the WindowContent itself
        // However, it may be worth sending the `gui_tx` to the
        // main window instead of the inner display, and have each call
        // of update_gui return a Option<GuiMsg>?
        let (gui_tx, gui_rx) = channel::<GuiMsg>();

        let current_window: Arc<Mutex<Box<WindowContents>>> = Arc::new(Mutex::new(Box::new(
            configwindow::ConfigWindow::new(config, gui_tx),
        )));
        let gui_window = Arc::clone(&current_window);

        thread::spawn(move || {
            let mut gui = Win::new(gui_window).unwrap();;
            gui.update();
        });

        for received in gui_rx {
            debug!("Got: {:?}", received);
            match received {
                GuiMsg::ConfigDone() => self.run_scanner(&current_window)?,
                GuiMsg::Error(error) => return Err(error),
            }
        }

        Ok(())
    }

    fn run_scanner(&self, current_window: &Arc<Mutex<Box<WindowContents>>>) -> Result<(), Error> {
        // TODO
        // I don't think going so far as message-passing is worth it in this case when we can easily share memory
        // This function will share an Arc'd AtomicU32 or something with the GUI thread
        // and the scanner thread (it's a thread because we might want to watch for errors/cancel/close from gui later on)
        // This integer will hold the number of photos processed
        // Ideally, we can also tell the estimated number of photos to scan, which should be simple:
        // We use our existing WalkDir iterator, add some of the logic from image::dynimage::open_impl to improve that,
        // and then call count() on the iterator.  This does consume the iterator, so we'll need to clone it first.

        // Init Arc'd AtomicU32 to 0
        // Init another Arc'd AtomicU32 to 0 (for estimate.  We could pass this in immediately, but I want to display "wait" asap)
        let mut state = (*current_window).lock().unwrap();
        *state = Box::new(waitwindow::WaitWindow::new()); // Pass in both ints

        // scanner::scan_files(), also passing in both ints
        Ok(())
    }
}

#[derive(Debug)]
pub enum GuiMsg {
    ConfigDone(),
    Error(Error),
}
