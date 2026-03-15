use std::thread;
use std::time::Duration;

pub struct Disk {
    sectors: Vec<String>,
}

impl Disk {
    const NUM_SECTORS: usize = 2048;
    const DISK_DELAY: u64 = 80;

    // initialize and return new Disk
    pub fn new() -> Self {
        let sectors = vec![String::new(); Self::NUM_SECTORS];
        Self {sectors}
    }

    pub fn write(&mut self, sector: usize, data: String) {
        thread::sleep(Duration::from_millis(Self::DISK_DELAY));
        self.sectors[sector] = data;
    }

    pub fn read(&self, sector: usize) -> String {
        thread::sleep(Duration::from_millis(Self::DISK_DELAY));
        self.sectors[sector].clone()
    }
}