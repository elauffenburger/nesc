// TODO: make this cpu::memory_map

mod test;
mod common;
mod constants;

pub use self::constants::*;
pub use self::common::*;

use super::ppu;
use super::rom;

use std::fmt;
use std::fmt::Debug;

pub struct MemoryMap {
    memory: Vec<u8>,

    num_prg_banks: u8,
    num_chr_banks: u8,
}

impl MemoryMap {
    fn load_prg_rom_upper(&mut self, buf: &[u8]) {
        let upper_bank_start = PRG_ROM_START + (PRG_ROM_BANK_SIZE as u16);

        self.write_memory(upper_bank_start as usize, buf);
    }

    fn load_prg_rom_lower(&mut self, buf: &[u8]) {
        let lower_bank_start = PRG_ROM_START;

        self.write_memory(lower_bank_start as usize, buf);
    }

    fn get_mirror_region_for_address(address: u16) -> MirrorRegion {
        // TODO: make these constants
        match () {
            _ if address < 0x2000 => MirrorRegion::One,
            _ if address >= 0x2000 && address < 0x4000 => MirrorRegion::Two,
            _ => MirrorRegion::None,
        }
    }

    fn write_memory(&mut self, starting_at: usize, buf: &[u8]) {
        write_memory(&mut self.memory, starting_at, buf);
    }

    fn write_to_mirrored_region_one(&mut self, address: u16, val: u8) {
        let address = MemoryMap::resolve_address_in_region_one(address);

        self.write_direct(address, val);
    }

    fn write_to_mirrored_region_two(&mut self, address: u16, val: u8) {
        let address = MemoryMap::resolve_address_in_region_two(address);

        self.write_direct(address, val);
    }

    fn resolve_address_in_region_one(address: u16) -> u16 {
        // TODO: make these constants
        resolve_mirrored_address(address, 0x0000, 0x0800)
    }

    fn resolve_address_in_region_two(address: u16) -> u16 {
        // TODO: make these constants
        resolve_mirrored_address(address, 0x2000, 0x0008)
    }

    fn write_direct(&mut self, address: u16, val: u8) {
        self.memory[address as usize] = val;
    }
}

impl MemoryMapper for MemoryMap {
    fn read(&self, address: u16) -> u8 {
        let address = match MemoryMap::get_mirror_region_for_address(address) {
            MirrorRegion::One => MemoryMap::resolve_address_in_region_one(address),
            MirrorRegion::Two => MemoryMap::resolve_address_in_region_two(address),
            MirrorRegion::None => address,
        };

        self.memory[address as usize]
    }

    fn write(&mut self, address: u16, val: u8) {
        // handle mirroring
        match MemoryMap::get_mirror_region_for_address(address) {
            MirrorRegion::One => self.write_to_mirrored_region_one(address, val),
            MirrorRegion::Two => self.write_to_mirrored_region_two(address, val),
            MirrorRegion::None => self.write_direct(address, val),
        }
    }

    fn load(&mut self, rom: &rom::NesRom) {
        self.num_prg_banks = rom.num_prg_banks;
        self.num_chr_banks = rom.num_chr_banks;

        // load prg_rom
        let prg_rom = &rom.prg_rom;
        match rom.num_prg_banks {
            1 => {
                self.load_prg_rom_upper(&prg_rom);
                // TODO implement an actual memory mapper -- this is poor
                // man's mirror for sure
                self.load_prg_rom_lower(&prg_rom);
            }
            _ => {
                panic!("invalid number of prg_banks to load into memory!");
            }
        }
    }
}

impl Debug for MemoryMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[...]")
    }
}

impl Default for MemoryMap {
    fn default() -> Self {
        MemoryMap {
            memory: vec![0; MEMORY_MAP_TOTAL_SIZE],
            num_chr_banks: 0,
            num_prg_banks: 0,
        }
    }
}

enum MirrorRegion {
    One,
    Two,
    None,
}

pub trait MemoryMapper {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, val: u8);
    fn load(&mut self, rom: &rom::NesRom);
}
