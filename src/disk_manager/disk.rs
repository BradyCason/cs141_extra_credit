use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

use crate::GuiState;

pub struct Disk {
    sectors: Vec<String>,
    id: usize,
    gui_state: Arc<Mutex<GuiState>>,
}

impl Disk {
    const NUM_SECTORS: usize = 2048;
    const DISK_DELAY: u64 = 80;

    // initialize and return new Disk
    pub fn new(id: usize, gui_state: Arc<Mutex<GuiState>>) -> Self {
        let sectors = vec![String::new(); Self::NUM_SECTORS];
        Self {
            sectors,
            id,
            gui_state,
        }
    }

    pub fn write(&mut self, sector: usize, data: String) {
        thread::sleep(Duration::from_millis(Self::DISK_DELAY));
        self.sectors[sector] = data;

        let mut gui_state = self.gui_state.lock().unwrap();
        gui_state.update_disk(self.id, (sector as f32) / (Self::NUM_SECTORS as f32));
    }

    pub fn read(&self, sector: usize) -> String {
        thread::sleep(Duration::from_millis(Self::DISK_DELAY));
        self.sectors[sector].clone()
    }
}