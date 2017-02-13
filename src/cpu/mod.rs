mod test;

use ::memory_map;
use ::memory_map::MemoryMapper;
use ::rom;

use std::fmt::Debug;

// [Cpu]
// The heart of the system: a MOS 6502 (really, a Ricoh clone)
//
// [Resources]
// stack, push, pop => http://www.cs.jhu.edu/~phi/csf/slides/lecture-6502-stack.pdf
// pc start address => http://forums.nesdev.com/viewtopic.php?t=5494
//
// [Stack operations]
// Stack operations are simple with u8, but tricky with u16
//
// With u8, just write 1 byte, then decrement one 1 byte
//
// With u16, decrement 2 bytes (so, overshoot the last address
//    of where the value willl be stored by 1 byte), then write
//    2 bytes from sp + 1 (so, starting at that previously overshot
//    last address)

#[derive(Debug, Default)]
pub struct Cpu<T: MemoryMapper + Debug> {
    memory_map: T,

    // Number of cycles left for the last instruction to execute
    pending_cycles: u8,

    // PC
    reg_program_counter: u16,

    // SP
    // Note that this is the u8 OFFSET from the bottom of the stack
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

impl<T: MemoryMapper + Debug> Cpu<T> {
    pub fn step_instruction(&mut self) {
        match self.pending_cycles {
            0 => {
                println!("{:#?}", &self);

                let opcode = self.next_word();

                match opcode {
                    0x4e => {
                        // lsr -- absolute
                        let mem_loc = self.next_double_word();

                        let val = self.read(mem_loc);
                        self.write(mem_loc, val >> 1);

                        self.take_cycles(3);
                    }
                    0x9a => {
                        // txs -- implied
                        self.reg_stack_pointer = self.reg_index_x;

                        self.take_cycles(2);
                    }
                    0x4c => {
                        // jmp -- absolute
                        let address = self.next_double_word();
                        self.reg_program_counter = address;

                        self.take_cycles(3);
                    }
                    0xa2 => {
                        // ldx -- immediate
                        let immediate = self.next_word();
                        self.reg_index_x = immediate;

                        self.take_cycles(2);
                    }
                    0x86 => {
                        // stx -- zero page
                        let address = self.next_word();
                        let x = self.reg_index_x;

                        self.write(address as u16, x);

                        self.take_cycles(3);
                    }
                    0x20 => {
                        // jsr -- absolute
                        let address = self.next_double_word();

                        // push the pc and move pc to new address
                        let pc = self.reg_program_counter;
                        self.push_u16(pc);

                        self.reg_program_counter = address + 1;

                        self.take_cycles(6);
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

    pub fn load(&mut self, rom: &rom::NesRom) {
        self.memory_map.load(rom);
    }

    pub fn run(&mut self) {
        self.init_registers();

        loop {
            self.step_instruction();
        }
    }

    pub fn push(&mut self, value: u8) {
        // write to sp
        let sp = self.resolve_stack_pointer();
        self.write(sp, value);

        // decrement sp
        self.reg_stack_pointer -= 1;
    }

    pub fn push_u16(&mut self, value: u16) {
        // decrement sp
        self.reg_stack_pointer -= 2;

        // write to sp + 1
        let sp = self.resolve_stack_pointer();
        self.write_u16(sp + 1, value);
    }

    pub fn pop(&mut self) -> u8 {
        self.reg_stack_pointer += 1;
        let val = self.read(self.resolve_stack_pointer());

        val
    }

    pub fn pop_u16(&mut self) -> u16 {
        self.reg_stack_pointer += 2;

        let val = self.read_u16(self.resolve_stack_pointer() - 1);

        val
    }

    pub fn init_registers(&mut self) {
        // set pc to prg_rom start address
        self.reg_program_counter = memory_map::PRG_ROM_START;

        // set sp to top of stack
        self.reg_stack_pointer = (memory_map::STACK_SIZE - 1) as u8;
    }

    fn write_u16(&mut self, mem_loc: u16, val: u16) {
        self.memory_map.write_u16(mem_loc, val);
    }

    fn write(&mut self, mem_loc: u16, val: u8) {
        self.memory_map.write(mem_loc, val);
    }

    fn read(&self, address: u16) -> u8 {
        self.memory_map.read(address)
    }

    fn read_u16(&self, address: u16) -> u16 {
        self.memory_map.read_u16(address)
    }

    fn finish_cycle(&mut self) {
        self.pending_cycles -= 1;
    }

    fn next_word(&mut self) -> u8 {
        let word = self.read(self.reg_program_counter);
        self.reg_program_counter += 1;

        word
    }

    fn next_double_word(&mut self) -> u16 {
        let double_word = self.read_u16(self.reg_program_counter);
        self.reg_program_counter += 2;

        double_word
    }

    fn take_cycles(&mut self, cycles: u8) {
        self.pending_cycles = cycles;
    }

    pub fn resolve_stack_pointer(&self) -> u16 {
        memory_map::STACK_START + (self.reg_stack_pointer as u16)
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