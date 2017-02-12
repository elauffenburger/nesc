use super::ppu;
use super::system;

use std::fmt;
use std::fmt::Debug;

const MEMORY_MAP_SIZE: usize = 0x010000;

pub struct MemoryMap {
    memory: Vec<u8>,

    num_prg_banks: u8,
    num_chr_banks: u8,
    num_ram_banks: u8,
}

impl MemoryMap {
    pub fn read(&self, address: u16) -> u8 {
        let address = match MemoryMap::get_mirror_region_for_address(address) {
            MirrorRegion::One => MemoryMap::resolve_address_in_region_one(address),
            MirrorRegion::Two => MemoryMap::resolve_address_in_region_two(address),
            MirrorRegion::None => address,
        };

        println!("reading address {:x}", address);

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

    pub fn configure(&mut self, config: system::SystemConfiguration) {}
}

impl Debug for MemoryMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[...]")
    }
}

impl Default for MemoryMap {
    fn default() -> Self {
        MemoryMap {
            memory: vec![0; MEMORY_MAP_SIZE],
            num_chr_banks: 0,
            num_prg_banks: 0,
            num_ram_banks: 0,
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
    let mut map = MemoryMap::default();

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
    let mut map = MemoryMap::default();

    map.write_to_mirrored_region_two(0x2009, 1);

    // test we wrote to the right places
    assert_eq!(&map.read(0x2001), &1);
    assert_eq!(&map.read(0x2009), &1);
    assert_eq!(&map.read(0x3ff9), &1);

    // test we stayed within our limits
    assert_eq!(&map.read(0x4000), &0);
}