use super::bits;

use std::fs::File;
use std::io::Read;

pub struct NesRom {
    pub data: Vec<u8>,
    identifier: Vec<u8>,
    format: u8,

    num_prg_banks: u8,
    num_chr_banks: u8,
    num_ram_banks: u8,
    mirroring_type: MirroringType,
    has_battery_backed_ram: bool,
    has_trainer: bool,
    mapper_number: u8,
}

enum MirroringType {
    Horizontal,
    Vertical,
    Both,
}

impl NesRom {
    pub fn from_nes_file(mut nes_file: File) -> Self {
        fn take_one_byte(slice: &[u8]) -> u8 {
            slice[0]
        }

        let mut buf: Vec<u8> = vec![];
        nes_file.read_to_end(&mut buf).unwrap();

        let header = &buf[..16];
        let rom = &buf[16..];

        // TODO verify identifier eq 'NES'
        let identifier = &header[0..3];

        // TODO verify identifier eq 1A
        let format = take_one_byte(&header[3..4]);

        let num_prg_banks = take_one_byte(&header[4..5]);
        let num_chr_banks = take_one_byte(&header[5..6]);
        let control_byte_one = take_one_byte(&header[6..7]);
        let control_byte_two = take_one_byte(&header[7..8]);
        let num_ram_banks = take_one_byte(&header[8..9]);

        // TODO actually verify these are all 0
        // let future_usage = &header[9..16];

        let mirroring_type = NesRom::get_mirroring_type(control_byte_one);
        let has_battery_backed_ram = (control_byte_one & 0b10) >> 1 == 1;
        let has_trainer = (control_byte_one & 0b100) >> 2 == 1;
        let mapper_number = NesRom::get_maper_number(control_byte_one, control_byte_two);

        NesRom {
            data: Vec::from(rom),
            identifier: Vec::from(identifier),
            format: format,
            num_prg_banks: num_prg_banks,
            num_chr_banks: num_chr_banks,
            num_ram_banks: num_ram_banks,
            mirroring_type: mirroring_type,
            has_battery_backed_ram: has_battery_backed_ram,
            has_trainer: has_trainer,
            mapper_number: mapper_number
        }
    }

    fn get_mirroring_type(control_byte_one: u8) -> MirroringType {
        match (control_byte_one & 0b1000) >> 3 == 1 {
            true => MirroringType::Both,
            _ => {
                match control_byte_one & 1 == 1 {
                    false => MirroringType::Horizontal,
                    _ => MirroringType::Vertical
                }
            }
        }
    }

    fn get_maper_number(control_byte_one: u8, control_byte_two: u8) -> u8 {
        bits::overlay(control_byte_two, control_byte_one)
    }
}
