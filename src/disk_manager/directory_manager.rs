use std::collections::HashMap;

#[derive(Clone, Copy)]
pub struct FileInfo {
    pub disk_number: usize,
    pub starting_sector: usize,
    pub file_length: usize,
}

pub struct DirectoryManager {
    files: HashMap<String, FileInfo>,
}

impl DirectoryManager {
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
        }
    }

    pub fn enter(&mut self, file_name: String, file: FileInfo) {
        self.files.insert(file_name, file);
    }

    pub fn lookup(&self, file_name: &String) -> Option<FileInfo> {
        self.files.get(file_name).cloned()
    }
}