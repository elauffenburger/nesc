pub fn merge(hi: u8, lo: u8) -> u16 {
    let hi_ex = ((hi as u16) << 8) | 0x00ff;
    let lo_ex = (lo as u16) | 0xff00;

    hi_ex & lo_ex
}

pub fn overlay(hi: u8, lo: u8) -> u8 {
    let hi_ex = ((hi >> 4) & 0b1111) << 4;
    let lo_ex = (lo >> 4) & 0b1111;

    (hi_ex | 0x0f) & (lo_ex | 0xf0)
}