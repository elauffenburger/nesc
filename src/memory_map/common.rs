use super::constants;

pub fn write_memory(memory: &mut Vec<u8>, starting_at: usize, buf: &[u8]) {
    for i in 0..constants::PRG_ROM_BANK_SIZE {
        let address = starting_at + i;
        let value = buf[i];

        memory[address] = value;
    }
}

pub fn resolve_mirrored_address(address: u16, start_address: u16, region_size: u16) -> u16 {
    let mirror_address = start_address + (address % region_size);

    mirror_address
}