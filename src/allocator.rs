use std::{collections::HashMap, io};

// MemBlock is a wrapper for raw pointer.
pub struct MemBlock {
    pub typ: MemBlockType,
    pub mem: *mut u8,
    pub size: usize,
}
pub enum MemBlockType {
    Code(String),
    Text(String),
    Other,
}

// Allocator manages memories which is used for compiled assembly code.
pub struct Allocator {
    pub blocks: HashMap<String, MemBlock>,
    pub cur_pos: usize,
}
impl Allocator {
    pub fn new() -> Self {
        return Self {
            blocks: HashMap::new(),
            cur_pos: 0,
        };
    }
    // allocate allocates memory by mmap syscall.
    // If it fails to allocate, it returns error.
    pub fn allocate(
        &mut self,
        typ: MemBlockType,
        label: String,
        size: usize,
    ) -> Result<(), io::Error> {
        return Ok(());
    }
    // find finds the memblock by the block's key.
    // If not found, return None.
    pub fn find(&self, str: String) -> Option<MemBlock> {
        return None;
    }
}
