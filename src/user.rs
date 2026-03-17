use std::fs;
use std::thread::{self, JoinHandle};
use std::sync::{Arc, Mutex, RwLock};

use crate::disk_manager::{DiskManager, disk::Disk, directory_manager::FileInfo};
use crate::printer_manager::{PrinterManager, printer::Printer};
use crate::GuiState;

pub struct User {
    id: usize,
    name: String,
    cur_file: Option<String>,
    cur_disk_id: usize,
    cur_disk: Option<Arc<RwLock<Disk>>>,
    cur_base_sector: usize,
    cur_file_length: usize,
    print_thread_handles: Vec<JoinHandle<()>>,
    disk_manager: Arc<RwLock<DiskManager>>,
    printer_manager: Arc<Mutex<PrinterManager>>,
    gui_state: Arc<Mutex<GuiState>>,
}

impl User {
    pub fn new(id: usize, disk_manager: Arc<RwLock<DiskManager>>, printer_manager: Arc<Mutex<PrinterManager>>, gui_state: Arc<Mutex<GuiState>>) -> Self {
        Self {
            id,
            name: format!("users/USER{id}"),
            cur_file: None,
            cur_disk_id: 0,
            cur_disk: None,
            cur_base_sector: 0,
            cur_file_length: 0,
            print_thread_handles: Vec::new(),
            disk_manager,
            printer_manager,
            gui_state,
        }
    }

    pub fn run(&mut self) {
        // Read user file
        let contents = fs::read_to_string(&self.name)
            .expect(&format!("Could not read file {}", self.name));

        // Iterate through lines
        for line in contents.lines() {
            loop {
                let gui_state = self.gui_state.lock().unwrap();
                if gui_state.user_active(self.id) {
                    break;
                }
            }
            self.handle_line(line);
        }

        // Update gui
        {
            let mut gui_state = self.gui_state.lock().unwrap();
            gui_state.update_user(self.id, "User Finished".to_string());
        }

        // Join print job handles
        for handle in self.print_thread_handles.drain(..) {
            handle.join().unwrap();
        }
    }

    fn handle_line(&mut self, line: &str) {
        if line.starts_with(".end") {
            // Update disk GUI
            if let Some(disk_mutex) = &self.cur_disk {
                // Get and lock the disk
                let disk = disk_mutex.read().unwrap();
                disk.update_gui("Not in use".to_string());
            }

            // End current file
            self.handle_end_command();
        } else if let Some(rest) = line.strip_prefix(".save ") {
            // Begin saving a file
            self.handle_save_command(rest.to_string());

            // Update user GUI
            {
                let mut gui_state = self.gui_state.lock().unwrap();
                gui_state.update_user(self.id, format!("Saving file: {rest}"));
            }

            // Update disk GUI
            if let Some(disk_mutex) = &self.cur_disk && let Some(file_name) = &self.cur_file {
                // Get and lock the disk
                let disk = disk_mutex.read().unwrap();
                disk.update_gui(format!("Writing file: {file_name}"));
            }
        } else if let Some(rest) = line.strip_prefix(".print ") {
            // Create new print thread
            self.handle_print_command(rest.to_string());

            // Update GUI
            {
                let mut gui_state = self.gui_state.lock().unwrap();
                gui_state.update_user(self.id, format!("Printing file: {rest}"));
            }
        } else if self.cur_file.is_some(){
            loop {
                let gui_state = self.gui_state.lock().unwrap();
                if gui_state.disk_active(self.cur_disk_id) {
                    break;
                }
            }

            // Save line in file
            self.save_line(line.to_string());

            // Update disk GUI
            if let Some(disk_mutex) = &self.cur_disk && let Some(file_name) = &self.cur_file {
                // Get and lock the disk
                let disk = disk_mutex.read().unwrap();
                disk.update_gui(format!("Writing file: {file_name}"));
            }
        } else {
            // Update user GUI
            {
                let mut gui_state = self.gui_state.lock().unwrap();
                gui_state.update_user(self.id, "Unknown Command".to_string());
            }
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
        self.cur_file = Some(file_name.clone());
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
        let gs: Arc<Mutex<GuiState>> = Arc::clone(&self.gui_state);

        self.print_thread_handles.push(thread::spawn(move || {
            {
                let mut gui_state = gs.lock().unwrap();
                gui_state.increase_prints_waiting();
            }

            // Get file info
            let disk_manager = dm.read().unwrap();
            let Some(file_info) = disk_manager.get_file_info(&file_name) else {
                println!("Cannot print file. File does not exist");
                return;
            };

            // Get disk using file info
            let Some(disk) = disk_manager.get_disk(file_info.disk_number) else {
                println!("Failed to get disk");
                return;
            };

            drop(disk_manager);

            // Read all lines of disk
            let disk = disk.read().unwrap();
            let mut lines: Vec<String> = Vec::new();
            for i in 0..file_info.file_length {
                lines.push(disk.read(file_info.starting_sector + i));
            }

            drop(disk);

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

            // Gain control of printer
            let mut printer = printer.lock().unwrap();

            // Update printer gui
            {
                let mut gui_state = gs.lock().unwrap();
                gui_state.update_printer(printer_id, Some(file_name.clone()));
                gui_state.decrease_prints_waiting();
            }

            // Print each line to the printer
            for line in lines {
                loop {
                    let gui_state = gs.lock().unwrap();
                    if gui_state.printer_active(printer_id) {
                        break;
                    }
                }

                printer.print(line).expect("Failed to print line");
                {
                    let mut gui_state = gs.lock().unwrap();
                    gui_state.update_printer(printer_id, Some(file_name.clone()));
                }
            }

            // Release printer
            let mut printer_manager = pm.lock().unwrap();
            printer_manager.release(printer_id);

            // Update printer gui
            {
                let mut gui_state = gs.lock().unwrap();
                gui_state.update_printer(printer_id, None);
            }
        }));
    }
}