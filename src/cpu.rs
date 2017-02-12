use super::memory_map;
use super::bits;
use super::system;
use super::rom;

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
        match self.pending_cycles {
            0 => {
                let opcode = self.next_word();

                match opcode {
                    0x4e => {
                        // lsr -- absolute

                        let mem_loc = self.next_double_word();

                        let val = self.read_memory(mem_loc);
                        self.write_memory(mem_loc, val >> 1);

                        self.take_cycles(3);
                    }
                    0x9a => {
                        // txs -- implied

                        self.reg_stack_pointer = self.reg_index_x;

                        self.take_cycles(2);
                    }
                    _ => panic!("unknown opcode: {:x}", &opcode),
                };
            } 
            ref cycles => {
                println!("waiting for {} more cycles", cycles);
            }
        }

        self.finish_cycle();
    }

    pub fn load(&mut self, rom: rom::NesRom) {
        self.rom = rom.data;
    }

    pub fn configure(&mut self, config: system::SystemConfiguration) {
        self.memory_map.configure(config);
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
        let word = self.rom[address as usize];

        println!("read word {:x}", word);

        word
    }

    fn finish_cycle(&mut self) {
        self.pending_cycles -= 1;
    }

    fn next_word(&mut self) -> u8 {
        let word = self.read_word(self.reg_program_counter);
        self.reg_program_counter += 1;

        word
    }

    fn next_double_word(&mut self) -> u16 {
        let first = self.next_word();
        let second = self.next_word();

        bits::merge(first, second)
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