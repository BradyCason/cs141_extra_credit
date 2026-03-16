use std::sync::{Arc, RwLock, Mutex};

pub mod disk;
use disk::Disk;

pub mod directory_manager;
use directory_manager::DirectoryManager;
use directory_manager::FileInfo;

use crate::GuiState;

pub struct DiskManager {
    num_disks: usize,
    disks: Vec<Arc<RwLock<Disk>>>,
    disks_free: Vec<bool>,
    next_free_sectors: Vec<usize>,
    directory_manager: DirectoryManager,
}

impl DiskManager {
    pub fn new(num_disks: usize, gui_state: Arc<Mutex<GuiState>>) -> Self {
        let mut disks = Vec::new();
        let mut disks_free = Vec::new();
        let mut next_free_sectors = Vec::new();

        for i in 0..num_disks {
            disks.push(Arc::new(RwLock::new(Disk::new(i, Arc::clone(&gui_state)))));
            disks_free.push(true);
            next_free_sectors.push(0);
        }

        Self {
            num_disks,
            disks,
            disks_free,
            next_free_sectors,
            directory_manager: DirectoryManager::new(),
        }
    }

    // Returns (disk_id, disk, base_sector)
    pub fn request(&mut self) -> (Option<Arc<RwLock<Disk>>>, usize, usize) {
        for id in 0..self.num_disks {
            if self.disks_free[id] {
                self.disks_free[id] = false;

                return (
                    Some(Arc::clone(&self.disks[id])),
                    id,
                    self.get_next_free_sector(id)
                );
            }
        }
        (None, 0, 0)
    }

    pub fn release(&mut self, id: usize) {
        self.disks_free[id] = true;
    }

    fn get_next_free_sector(&self, disk_id: usize) -> usize {
        self.next_free_sectors[disk_id]
    }

    pub fn set_next_free_sector(&mut self, disk_id: usize, next: usize) {
        self.next_free_sectors[disk_id] = next;
    }

    pub fn enter_file_info(&mut self, file_name: String, file_info: FileInfo) {
        self.directory_manager.enter(file_name, file_info);
    }

    pub fn get_file_info(&self, file_name: &String) -> Option<FileInfo> {
        self.directory_manager.lookup(&file_name)
    }

    pub fn get_disk(&self, disk_id: usize) -> Option<Arc<RwLock<Disk>>> {
        if disk_id < self.num_disks {
            return Some(Arc::clone(&self.disks[disk_id]));
        }
        None
    }
}