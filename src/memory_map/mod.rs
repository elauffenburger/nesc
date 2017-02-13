// TODO: make this cpu::memory_map

mod common;
mod constants;
mod mappers;

pub use self::constants::*;
pub use self::common::*;
pub use self::mappers::*;

use ::rom;

pub trait MemoryMapper {
    fn read(&self, address: u16) -> u8;
    fn read_u16(&self, address: u16) -> u16;
    fn write(&mut self, address: u16, val: u8);
    fn write_u16(&mut self, address: u16, val: u16);
    fn load(&mut self, rom: &rom::NesRom);
}
