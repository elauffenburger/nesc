use super::ppu;
use super::system;

use std::fmt;
use std::fmt::Debug;

const MEMORY_MAP_SIZE: usize = 65535;

#[derive(Default)]
pub struct MemoryMap {
    memory: Vec<u8>,

    num_prg_banks: u8,
    num_chr_banks: u8,
    num_ram_banks: u8,
}

impl MemoryMap {
    pub fn read(&self, loc: u16) -> u8 {
        self.memory[loc as usize]
    }

    pub fn write(&mut self, loc: u16, val: u8) {
        self.memory[loc as usize] = val;
    }

    pub fn configure(&mut self, config: system::SystemConfiguration) {}
}

impl Debug for MemoryMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[...]")
    }
}