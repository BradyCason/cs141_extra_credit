use std::fs;
use std::thread::{self, JoinHandle};
use std::sync::{Arc, Mutex, RwLock};

use crate::disk_manager::{DiskManager, disk::Disk, directory_manager::FileInfo};
use crate::printer_manager::{PrinterManager, printer::Printer};

pub struct User {
    name: String,
    cur_file: Option<String>,
    cur_disk_id: usize,
    cur_disk: Option<Arc<RwLock<Disk>>>,
    cur_base_sector: usize,
    cur_file_length: usize,
    print_thread_handles: Vec<JoinHandle<()>>,
    disk_manager: Arc<RwLock<DiskManager>>,
    printer_manager: Arc<Mutex<PrinterManager>>,
}

impl User {
    pub fn new(id: usize, disk_manager: Arc<RwLock<DiskManager>>, printer_manager: Arc<Mutex<PrinterManager>>) -> Self {
        Self {
            name: format!("users/USER{id}"),
            cur_file: None,
            cur_disk_id: 0,
            cur_disk: None,
            cur_base_sector: 0,
            cur_file_length: 0,
            print_thread_handles: Vec::new(),
            disk_manager,
            printer_manager,
        }
    }

    pub fn run(&mut self) {
        // Read user file
        let contents = fs::read_to_string(&self.name)
            .expect(&format!("Could not read file {}", self.name));

        // Iterate through lines
        for line in contents.lines() {
            self.handle_line(line);
        }

        // Join print job handles
        for handle in self.print_thread_handles.drain(..) {
            handle.join().unwrap();
        }
    }

    fn handle_line(&mut self, line: &str) {
        if line.starts_with(".end") {
            // End current file
            self.handle_end_command();
        } else if let Some(rest) = line.strip_prefix(".save ") {
            // Begin saving a file
            self.handle_save_command(rest.to_string());
            println!("Saving file: {}", rest);
        } else if let Some(rest) = line.strip_prefix(".print ") {
            // Create new print thread
            self.handle_print_command(rest.to_string());

            println!("Printing file: {rest}");
        } else if self.cur_file.is_some(){
            // Save line in file
            self.save_line(line.to_string());
        } else {
            println!("Unknown Command");
        }
    }

    fn handle_end_command(&mut self){
        let mut dm = self.disk_manager.write().unwrap();

        let Some(file_name) = &self.cur_file else {
            println!("No file to end");
            return;
        };

        dm.enter_file_info(file_name.clone(), FileInfo {
            disk_number: self.cur_disk_id,
            file_length: self.cur_file_length,
            starting_sector: self.cur_base_sector
        });
        dm.set_next_free_sector(self.cur_disk_id, self.cur_base_sector + self.cur_file_length);

        // Release disk
        dm.release(self.cur_disk_id);

        self.cur_disk = None;
        self.cur_file = None;
    }

    fn handle_save_command(&mut self, file_name: String) {
        self.cur_file = Some(file_name);
        self.cur_file_length = 0;
        self.cur_file_length = 0;

        // Request disk from disk manager
        loop {
            let mut dm = self.disk_manager.write().unwrap();
            (self.cur_disk, self.cur_disk_id, self.cur_base_sector) = dm.request();
            if self.cur_disk.is_some() {
                break;
            }
        }
    }

    fn save_line(&mut self, line: String) {
        if let Some(disk_mutex) = &self.cur_disk {
            // Get and lock the disk
            let mut disk = disk_mutex.write().unwrap();

            // Write line to correct sector
            disk.write(self.cur_base_sector + self.cur_file_length, line);
            self.cur_file_length += 1;
        } else {
            println!("No disk to save file");
        }
    }

    fn handle_print_command(&mut self, file_name: String) {
        let pm: Arc<Mutex<PrinterManager>> = Arc::clone(&self.printer_manager);
        let dm: Arc<RwLock<DiskManager>> = Arc::clone(&self.disk_manager);

        self.print_thread_handles.push(thread::spawn(move || {
            // Request printer
            let printer: Arc<Mutex<Printer>>;
            let printer_id: usize;
            loop {
                let mut printer_manager = pm.lock().unwrap();

                match printer_manager.request() {
                    (Some(p), p_id) => {
                        printer = p;
                        printer_id = p_id;
                        break;
                    },
                    (None, _) => {}
                }
            }

            // Get file info
            let disk_manager = dm.read().unwrap();
            let Some(file_info) = disk_manager.get_file_info(&file_name) else {
                println!("Cannot print file. File does not exist");
                let mut printer_manager = pm.lock().unwrap();
                printer_manager.release(printer_id);
                return;
            };

            // Get disk using file info
            let Some(disk) = disk_manager.get_disk(file_info.disk_number) else {
                println!("Failed to get disk");
                let mut printer_manager = pm.lock().unwrap();
                printer_manager.release(printer_id);
                return;
            };

            drop(disk_manager);

            // Gain control of disk
            let disk = disk.read().unwrap();
            let mut printer = printer.lock().unwrap();

            // Print each line to the printer
            for i in 0..file_info.file_length {
                let line = disk.read(file_info.starting_sector + i);
                // println!("{line}");
                printer.print(line).expect("Failed to print line");
            }

            // Release printer
            let mut printer_manager = pm.lock().unwrap();
            printer_manager.release(printer_id);
        }));
    }
}