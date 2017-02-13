use ::memory_map::constants::*;
use ::memory_map::common::*;
use ::memory_map::MemoryMapper;
use ::rom;

use std::fmt;
use std::fmt::Debug;

use byteorder;
use byteorder::ByteOrder;

pub struct NROMMemoryMap {
    memory: Vec<u8>,

    num_prg_banks: u8,
    num_chr_banks: u8,
}

impl NROMMemoryMap {
    fn load_prg_rom_upper(&mut self, buf: &[u8]) {
        let upper_bank_start = PRG_ROM_START + (PRG_ROM_BANK_SIZE as u16);

        self.write_memory(upper_bank_start as usize, PRG_ROM_BANK_SIZE, buf);
    }

    fn load_prg_rom_lower(&mut self, buf: &[u8]) {
        let lower_bank_start = PRG_ROM_START;

        self.write_memory(lower_bank_start as usize, PRG_ROM_BANK_SIZE, buf);
    }

    fn get_mirror_region_for_address(address: u16) -> MirrorRegion {
        // TODO: make these constants
        match () {
            _ if address < 0x2000 => MirrorRegion::One,
            _ if address >= 0x2000 && address < 0x4000 => MirrorRegion::Two,
            _ => MirrorRegion::None,
        }
    }

    fn write_memory(&mut self, starting_at: usize, len: usize, buf: &[u8]) {
        write_memory(&mut self.memory, starting_at, len, buf);
    }

    fn write_to_mirrored_region_one(&mut self, address: u16, val: u8) {
        let address = NROMMemoryMap::resolve_address_in_region_one(address);

        self.write_direct(address, val);
    }

    fn write_to_mirrored_region_two(&mut self, address: u16, val: u8) {
        let address = NROMMemoryMap::resolve_address_in_region_two(address);

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

impl MemoryMapper for NROMMemoryMap {
    fn read(&self, address: u16) -> u8 {
        let address = match NROMMemoryMap::get_mirror_region_for_address(address) {
            MirrorRegion::One => NROMMemoryMap::resolve_address_in_region_one(address),
            MirrorRegion::Two => NROMMemoryMap::resolve_address_in_region_two(address),
            MirrorRegion::None => address,
        };

        self.memory[address as usize]
    }

    fn read_u16(&self, address: u16) -> u16 {
        let first = self.read(address);
        let second = self.read(address + 0x1);

        byteorder::LittleEndian::read_u16(&vec![first, second])
    }

    fn write(&mut self, address: u16, val: u8) {
        // handle mirroring
        match NROMMemoryMap::get_mirror_region_for_address(address) {
            MirrorRegion::One => self.write_to_mirrored_region_one(address, val),
            MirrorRegion::Two => self.write_to_mirrored_region_two(address, val),
            MirrorRegion::None => self.write_direct(address, val),
        }
    }

    fn write_u16(&mut self, address: u16, val: u16) {
        let mut bytes = [0; 2];
        byteorder::LittleEndian::write_u16(&mut bytes, val);

        self.write_memory(address as usize, 2, &bytes);
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

impl Debug for NROMMemoryMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[...]")
    }
}

impl Default for NROMMemoryMap {
    fn default() -> Self {
        NROMMemoryMap {
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

#[test]
fn test_mirrored_region_one_writes() {
    let mut map = super::NROMMemoryMap::default();

    map.write_to_mirrored_region_one(0x0000, 1);

    // test we wrote to the right places
    assert_eq!(&map.read(0x0000), &1);
    assert_eq!(&map.read(0x0800), &1);
    assert_eq!(&map.read(0x1000), &1);
    assert_eq!(&map.read(0x1800), &1);

    // test we stayed within our limits
    assert_eq!(&map.read(0x2000), &0);
}

#[test]
fn test_mirrored_region_two_writes() {
    let mut map = super::NROMMemoryMap::default();

    map.write_to_mirrored_region_two(0x2009, 1);

    // test we wrote to the right places
    assert_eq!(&map.read(0x2001), &1);
    assert_eq!(&map.read(0x2009), &1);
    assert_eq!(&map.read(0x3ff9), &1);

    // test we stayed within our limits
    assert_eq!(&map.read(0x4000), &0);
}