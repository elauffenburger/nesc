pub fn write_memory(memory: &mut Vec<u8>, starting_at: usize, len: usize, buf: &[u8]) {
    for i in 0..len {
        let address = starting_at + i;
        let value = buf[i];

        memory[address] = value;
    }
}

pub fn resolve_mirrored_address(address: u16, start_address: u16, region_size: u16) -> u16 {
    let mirror_address = start_address + (address % region_size);

    mirror_address
}

pub fn crosses_page_boundary(in_page: u16, address: u16) -> bool {
    (in_page & 0xff00) == (address & 0xff00)
}