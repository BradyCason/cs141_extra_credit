use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

use crate::GuiState;

pub struct Disk {
    sectors: Vec<String>,
    id: usize,
    gui_state: Arc<Mutex<GuiState>>,
    last_sector: usize,
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
            last_sector: 0,
        }
    }

    pub fn write(&mut self, sector: usize, data: String) {
        thread::sleep(Duration::from_millis(Self::DISK_DELAY));
        self.sectors[sector] = data;
        self.last_sector = sector;
    }

    pub fn read(&self, sector: usize) -> String {
        thread::sleep(Duration::from_millis(Self::DISK_DELAY));
        self.sectors[sector].clone()
    }

    pub fn update_gui(&self, action: String) {
        let mut gui_state = self.gui_state.lock().unwrap();
        gui_state.update_disk(self.id, action,self.last_sector as f32 / Self::NUM_SECTORS as f32);
    }
}