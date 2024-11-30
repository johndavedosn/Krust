
use alloc::string::String;
use hashbrown::HashMap;
#[derive(Clone, Debug)]
pub struct FileDescriptor {
    pub file_id: u32,
    pub file_size: Option<usize>,
    pub file_content: String
}

impl FileDescriptor {
    pub fn new(file_id: u32) -> Self{
        Self {
            file_content: String::new(),
            file_id,
            file_size: None
        }
    }
    pub fn set_content(&mut self, file_content: String) {
        self.file_content = file_content;
        self.file_size = Some(self.file_content.len());
    }
}
pub struct OpenFiles {
    open_files: HashMap<u32, FileDescriptor>
}

impl OpenFiles {
    pub fn new() -> Self {
        Self {
            open_files: HashMap::new()
        }
    }
    pub fn open_file(&mut self, fd: FileDescriptor) {
        self.open_files.insert(fd.file_id, fd);
    }
    pub fn get_file_by_id(&self, fid: u32)  -> Result<&FileDescriptor, &str>{
        let fd = self.open_files.get(&fid);
        if let Some(fd) = fd {
            Ok(fd)
        }
        else {
            Err("Could not find specified file ID")
        }
    }
}
