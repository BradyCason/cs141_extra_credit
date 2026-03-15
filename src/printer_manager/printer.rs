use std::thread;
use std::time::Duration;

use std::fs::OpenOptions;
use std::io::Write;
use std::io::Result;

pub struct Printer {
    name: String,
}

impl Printer {
    const PRINT_DELAY: u64 = 275;

    // initialize and return new Disk
    pub fn new(id: usize) -> Self {
        Self {
            name: format!("printers/PRINTER{id}")
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

        Ok(())
    }
}