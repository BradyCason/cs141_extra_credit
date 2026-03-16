use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

use std::fs::OpenOptions;
use std::io::Write;
use std::io::Result;

use crate::GuiState;

pub struct Printer {
    id: usize,
    name: String,
    gui_state: Arc<Mutex<GuiState>>,
}

impl Printer {
    const PRINT_DELAY: u64 = 275;

    // initialize and return new Disk
    pub fn new(id: usize, gui_state: Arc<Mutex<GuiState>>) -> Self {
        Self {
            id,
            name: format!("printers/PRINTER{id}"),
            gui_state,
        }
    }

    pub fn print(&mut self, data: String) -> Result<()> {
        thread::sleep(Duration::from_millis(Self::PRINT_DELAY));
        
        let mut file = OpenOptions::new()
            // .write(true)
            .append(true)
            .create(true)
            .open(&self.name)?;

        writeln!(file, "{data}").expect("Print failed");

        let mut gui_state = self.gui_state.lock().unwrap();
        gui_state.update_printer(self.id, data);

        Ok(())
    }
}