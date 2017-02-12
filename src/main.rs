pub mod cpu;
pub mod ppu;
pub mod memory_map;
pub mod rom;
pub mod bits;
pub mod system;

use std::fs::File;

#[derive(Debug, Default)]
struct Nes {
    cpu: cpu::Cpu,
}

impl Nes {
    pub fn power_on(&mut self) {}

    pub fn run(&mut self) {
        self.cpu.run();
    }

    pub fn load_rom(&mut self, rom: rom::NesRom) {
        let config = rom.to_system_configuration();
        self.cpu.load(rom);

        self.cpu.configure(config);
        // TODO configure ppu
    }
}

fn main() {
    let rom_path = "c:/users/eric lauffenburger/downloads/roms/nes/nestest.nes";
    let rom_file = File::open(rom_path).unwrap();

    let rom = rom::NesRom::from_nes_file(rom_file);
    let mut nes = Nes::default();

    nes.load_rom(rom);
    nes.power_on();

    nes.run();
}