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