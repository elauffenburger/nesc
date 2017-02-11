use std::fmt;
use std::fmt::Debug;

const MEMORY_MAP_SIZE: usize = 65535;

pub struct MemoryMap {
    memory: Vec<u8>,
}

impl MemoryMap {
    pub fn read(&self, loc: u16) -> u8 {
        self.memory[loc as usize]
    }

    pub fn write(&mut self, loc: u16, val: u8) {
        self.memory[loc as usize] = val;
    }
}

impl Default for MemoryMap {
    fn default() -> Self {
        MemoryMap { memory: vec![0; MEMORY_MAP_SIZE] }
    }
}

impl Debug for MemoryMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[...]")
    }
}