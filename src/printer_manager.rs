use std::sync::{Arc, Mutex};

pub mod printer;
use printer::Printer;

use crate::GuiState;

pub struct PrinterManager {
    num_printers: usize,
    printers: Vec<Arc<Mutex<Printer>>>,
    printers_free: Vec<bool>,
}

impl PrinterManager {
    pub fn new(num_printers: usize, gui_state: Arc<Mutex<GuiState>>) -> Self {
        let mut printers = Vec::new();
        let mut printers_free = Vec::new();

        for i in 0..num_printers {
            printers.push(Arc::new(Mutex::new(Printer::new(i, Arc::clone(&gui_state)))));
            printers_free.push(true);
        }

        Self {
            num_printers,
            printers,
            printers_free,
        }
    }

    pub fn request(&mut self) -> (Option<Arc<Mutex<Printer>>>, usize) {
        for id in 0..self.num_printers {
            if self.printers_free[id] {
                self.printers_free[id] = false;

                return (
                    Some(Arc::clone(&self.printers[id])),
                    id
                );
            }
        }
        (None, 0)
    }

    pub fn release(&mut self, id: usize) {
        self.printers_free[id] = true;
    }
}