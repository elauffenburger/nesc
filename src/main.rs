pub mod cpu;
pub mod memory_map;

use std::fs::File;

#[derive(Default,Debug)]
struct Nes {
    cpu: cpu::Cpu,
}

impl Nes {
    pub fn load(&mut self, rom: File) {
        self.cpu.load(rom);
    }

    pub fn power_on(&mut self) {}

    pub fn run(&mut self) {
        self.cpu.run();
    }
}

fn main() {
    let rom_file = "c:/users/eric lauffenburger/downloads/roms/nes/nestest.nes";
    let rom = File::open(rom_file).unwrap();

    let mut nes = Nes::default();
    nes.load(rom);
    nes.power_on();

    nes.run();
}