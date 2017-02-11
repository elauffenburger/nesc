use super::memory_map;

use std::fs::File;
use std::io::Read;

#[derive(Debug, Default)]
pub struct Cpu {
    rom: Vec<u8>,

    memory_map: memory_map::MemoryMap,

    // Number of cycles left for the last instruction to execute
    pending_cycles: u8,

    // PC
    reg_program_counter: u16,

    // SP
    reg_stack_pointer: u8,

    // A
    reg_accumulator: u8,

    // X
    reg_index_x: u8,

    // Y
    reg_index_y: u8,

    // P
    processor_status: ProcessorStatus,
}

impl Cpu {
    pub fn step_instruction(&mut self) {
        if self.pending_cycles == 0 {
            let instruction = self.next_word();

            println!("{:x}", instruction);

            match instruction {
                0x4E => {
                    let opcode = instruction;
                    let mem_loc = self.next_double_word();

                    let val = self.read_memory(mem_loc);
                    self.write_memory(mem_loc, val >> 1);

                    self.take_cycles(3);
                } 
                _ => panic!("unknown instruction: {:x}", &instruction),
            };
        }

        self.finish_cycle();
    }

    pub fn load(&mut self, mut rom: File) {
        rom.read_to_end(&mut self.rom).unwrap();
    }

    pub fn run(&mut self) {
        if self.rom.len() == 0 {
            panic!("No rom loaded!")
        }

        loop {
            self.step_instruction();
        }
    }

    fn write_memory(&mut self, mem_loc: u16, val: u8) {
        self.memory_map.write(mem_loc, val);
    }

    fn read_memory(&self, mem_loc: u16) -> u8 {
        self.memory_map.read(mem_loc)
    }

    fn read_word(&self, address: u16) -> u8 {
        self.rom[address as usize]
    }

    fn finish_cycle(&mut self) {
        self.pending_cycles -= 1;
    }

    fn next_word(&mut self) -> u8 {
        let word = self.read_word(self.reg_program_counter);
        self.reg_program_counter += 8;

        word
    }

    fn next_double_word(&mut self) -> u16 {
        let first = self.next_word();
        let second = self.next_word();

        let first_ex = ((first as u16) << 8) | 0x00ff;
        let second_ex = (second as u16) | 0xff00;

        first_ex & second_ex
    }

    fn take_cycles(&mut self, cycles: u8) {
        self.pending_cycles = cycles;
    }
}

#[derive(Default, Debug)]
pub struct ProcessorStatus {
    // Carry Flag (C)
    last_instruction_overunderflow: bool,

    // Zero Flag (Z)
    last_instruction_zero: bool,

    // Interrupt Disable (I)
    interrupts_disabled: bool,

    // Decimal Mode (D)
    decimal_mode: bool,

    // Break Command (B)
    break_instruction_executed: bool,

    // Overflow Flag (V)
    invalid_twos_complement_result: bool,

    // Negative Flag (N)
    last_operation_result_negative: bool,
}