use super::ppu;

pub struct SystemConfiguration {
    pub num_prg_banks: u8,
    pub num_chr_banks: u8,
    pub num_ram_banks: u8,
    pub mirroring_type: ppu::MirroringType
}