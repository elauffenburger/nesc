// TODO: make this cpu::memory_map

pub mod test;

use super::ppu;
use super::rom;

use std::fmt;
use std::fmt::Debug;

pub const MEMORY_MAP_TOTAL_SIZE: usize = 0x010000;

pub const PRG_ROM_BANK_SIZE: usize = 0x004000;
pub const PRG_ROM_TOTAL_SIZE: usize = PRG_ROM_BANK_SIZE * 2;
pub const PRG_ROM_START: u16 = 0x8000;
pub const PRG_ROM_END: u16 = 0xffff;

pub const SRAM_SIZE: usize = 0x002000;
pub const EXPANSION_ROM_SIZE: usize = 0x001fe0;
pub const IO_REGISTERS_HI_SIZE: usize = 0x000020;
pub const IO_REGISTERS_LO_SIZE: usize = 0x000008;
pub const RAM_SIZE: usize = 0x000600;
pub const STACK_SIZE: usize = 0x000100;
pub const ZERO_PAGE_SIZE: usize = 0x000100;

pub struct MemoryMap {
    memory: Vec<u8>,

    num_prg_banks: u8,
    num_chr_banks: u8,
}

impl MemoryMap {
    pub fn read(&self, address: u16) -> u8 {
        let address = match MemoryMap::get_mirror_region_for_address(address) {
            MirrorRegion::One => MemoryMap::resolve_address_in_region_one(address),
            MirrorRegion::Two => MemoryMap::resolve_address_in_region_two(address),
            MirrorRegion::None => address,
        };

        self.memory[address as usize]
    }

    pub fn write(&mut self, address: u16, val: u8) {
        // handle mirroring
        match MemoryMap::get_mirror_region_for_address(address) {
            MirrorRegion::One => self.write_to_mirrored_region_one(address, val),
            MirrorRegion::Two => self.write_to_mirrored_region_two(address, val),
            MirrorRegion::None => self.write_direct(address, val),
        }
    }

    pub fn load(&mut self, rom: &rom::NesRom) {
        self.num_prg_banks = rom.num_prg_banks;
        self.num_chr_banks = rom.num_chr_banks;

        // load prg_rom
        let prg_rom = &rom.prg_rom;
        match rom.num_prg_banks {
            1 => self.load_prg_rom_upper(&prg_rom),
            _ => {
                panic!("invalid number of prg_banks to load into memory!");
            }
        }
    }

    fn load_prg_rom_upper(&mut self, buf: &[u8]) {
        let upper_bank_start = PRG_ROM_START + (PRG_ROM_BANK_SIZE as u16);

        for i in 0..PRG_ROM_BANK_SIZE {
            self.memory[(upper_bank_start as usize) + i] = buf[i];
        }
    }

    fn get_mirror_region_for_address(address: u16) -> MirrorRegion {
        // TODO: make these constants
        match () {
            _ if address < 0x2000 => MirrorRegion::One,
            _ if address >= 0x2000 && address < 0x4000 => MirrorRegion::Two,
            _ => MirrorRegion::None,
        }
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
        MemoryMap::resolve_mirrored_address(address, 0x0000, 0x0800)
    }

    fn resolve_address_in_region_two(address: u16) -> u16 {
        // TODO: make these constants
        MemoryMap::resolve_mirrored_address(address, 0x2000, 0x0008)
    }

    fn resolve_mirrored_address(address: u16, start_address: u16, region_size: u16) -> u16 {
        let mirror_address = start_address + (address % region_size);

        mirror_address
    }

    fn write_direct(&mut self, address: u16, val: u8) {
        self.memory[address as usize] = val;
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