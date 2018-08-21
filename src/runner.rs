use crate::win::{comparewindow, configwindow, waitwindow, Win, WindowContents};
use failure::Error;
use img_dedup::{self as scanner, Config, SimilarPair};
use log::debug;
use std::collections::BinaryHeap;
use std::sync::{
    atomic::AtomicU32,
    mpsc::{channel, Sender},
    Arc, Mutex,
};
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
        // of update_gui return a Option<ThreadMsg>?
        let (gui_tx, gui_rx) = channel::<ThreadMsg>();
        let config = Arc::new(Mutex::new(config));

        let current_window: Arc<Mutex<Box<WindowContents>>> = Arc::new(Mutex::new(Box::new(
            configwindow::ConfigWindow::new(Arc::clone(&config), gui_tx.clone()),
        )));
        let gui_window = Arc::clone(&current_window);

        thread::spawn(move || {
            let mut gui = Win::new(gui_window).unwrap();;
            gui.update();
        });

        for received in gui_rx {
            debug!("Got: {:?}", received);
            match received {
                ThreadMsg::ConfigDone() => {
                    self.run_scanner(&current_window, &config, gui_tx.clone())?
                }
                ThreadMsg::ScanDone(files) => {
                    self.show_compare(&current_window, files, gui_tx.clone())?
                }
                // TODO: Delete files to be deleted and notify user
                // Or maybe give them one last review
                ThreadMsg::CompareDone() => return Ok(()),
                ThreadMsg::Error(error) => return Err(error),
            }
        }

        Ok(())
    }

    fn run_scanner(
        &self,
        current_window: &Arc<Mutex<Box<WindowContents>>>,
        config: &Arc<Mutex<Config>>,
        tx: Sender<ThreadMsg>,
    ) -> Result<(), Error> {
        // I would like to send the progress to the GUI
        // We have to determine if we want to share an Atomic integer between
        // the scanner and the GUI, or if we want to send messages.
        // Atomic Integers seems simpler if the scanner will update one or more
        // times per widget refresh.  Does conrod run at 60fps or as fast as possible?

        // Otherwise, maybe we'll use message passing.  The widget will have to perform
        // one or more non-blocking channel reads on its receiver, and increment the counter
        // on its side.  This seems complicated and possibly slow.

        // The third option is to buffer it somehow.  This seems unnecessary and I'll only do
        // that if performance becomes an issue.
        let total = Arc::new(AtomicU32::new(0));
        let processed = Arc::new(AtomicU32::new(0));

        let config = config.lock().unwrap();

        let mut state = (*current_window).lock().unwrap();
        *state = Box::new(waitwindow::WaitWindow::new(
            Arc::clone(&processed),
            Arc::clone(&total),
        )); // Pass in both integers
        let directory = config.directory.clone();
        let method = config.method.clone();
        let hash_length = config.hash_length;

        thread::spawn(move || {
            let files = scanner::scan_files(
                directory,
                method,
                hash_length,
                &Arc::clone(&total),
                Arc::clone(&processed),
            ).unwrap();

            tx.send(ThreadMsg::ScanDone(files)).unwrap();
        });

        Ok(())
    }

    fn show_compare(
        &self,
        current_window: &Arc<Mutex<Box<WindowContents>>>,
        files: BinaryHeap<SimilarPair>,
        gui_tx: Sender<ThreadMsg>,
    ) -> Result<(), Error> {
        let mut state = (*current_window).lock().unwrap();
        // Todo: Throw an error if no files were returned
        // Maybe bring the user back to Config with a message
        *state = Box::new(comparewindow::CompareWindow::new(files, gui_tx).unwrap());
        Ok(())
    }
}

#[derive(Debug)]
pub enum ThreadMsg {
    ConfigDone(),
    ScanDone(BinaryHeap<SimilarPair>),
    // TODO: Return a list of files to delete with CompareDone?
    // That way users can
    CompareDone(),
    Error(Error),
}
