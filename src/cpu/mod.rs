pub mod instructions;

mod processor_status;
mod test;
mod debug;
mod stack;

pub use self::processor_status::*;
pub use self::debug::*;
pub use self::stack::*;

use ::memory_map;
use ::memory_map::MemoryMapper;
use ::rom;

use std::io;
use std::io::Write;

// [Cpu]
// The heart of the system: a MOS 6502 (really, a Ricoh clone)
//
// [Resources]
// stack, push, pop => http://www.cs.jhu.edu/~phi/csf/slides/lecture-6502-stack.pdf
// pc start address => http://forums.nesdev.com/viewtopic.php?t=5494
// page boundaries => http://atariage.com/forums/topic/250652-what-is-a-page-boundary/?p=3475052
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
pub struct Cpu<T: MemoryMapper> {
    memory_map: T,

    // Number of cycles left for the last instruction to execute
    pending_cycles: u8,

    // PC
    reg_program_counter: u16,

    // SP
    // Note that this is the u8 OFFSET from the bottom of the stack
    reg_stack_pointer: u8,

    // A
    reg_accumulator: i8,

    // X
    reg_index_x: u8,

    // Y
    reg_index_y: u8,

    // P
    processor_status: ProcessorStatus,

    // The last disassembled instruction
    last_instr_disasm: String,
}

impl<T: MemoryMapper> Cpu<T> {
    pub fn step_instruction(&mut self) {
        self.last_instr_disasm = "".to_string();

        match self.pending_cycles {
            0 => {
                // println!("{:#?}", &self);
                let pc = self.reg_program_counter;

                let opcode = self.next_word();

                match opcode {
                    0x4e => {
                        // lsr -- absolute
                        let mem_loc = self.next_double_word();

                        let val = self.read(mem_loc);
                        let (result, overflowed) = val.overflowing_shr(1);

                        self.processor_status.carry_flag = overflowed;
                        self.write(mem_loc, result);
                        self.take_cycles(3);

                        self.set_last_instr_disasm(format!("lsr {:#x}", &result));
                    }
                    0x9a => {
                        // txs -- implied
                        self.reg_stack_pointer = self.reg_index_x;
                        self.take_cycles(2);

                        self.set_last_instr_disasm_str("txs");
                    }
                    0x4c => {
                        // jmp -- absolute
                        let address = self.next_double_word();

                        self.reg_program_counter = address;
                        self.take_cycles(3);

                        self.set_last_instr_disasm(format!("jmp {:#x}", &address));
                    }
                    0xa2 => {
                        // ldx -- immediate
                        let immediate = self.next_word();

                        self.reg_index_x = immediate;
                        self.take_cycles(2);

                        self.set_last_instr_disasm(format!("ldx {:#x}", &immediate));
                    }
                    0x86 => {
                        // stx -- zero page
                        let address = self.next_word() as u16;
                        let x = self.reg_index_x;

                        self.write(address as u16, x);
                        self.take_cycles(3);

                        self.set_last_instr_disasm(format!("stx {:#x}", &address));
                    }
                    0x20 => {
                        // jsr -- absolute
                        let address = self.next_double_word();

                        // push the starting pc + 2 and move pc to new address
                        self.push_u16(pc + 2);

                        self.reg_program_counter = address;
                        self.take_cycles(6);

                        self.set_last_instr_disasm(format!("jsr {:#x}", &address));
                    }
                    0x38 => {
                        // sec -- immediate
                        self.processor_status.carry_flag = true;
                        self.take_cycles(2);

                        self.set_last_instr_disasm_str("sec");
                    }
                    0xb0 => {
                        // bcs -- relative
                        instructions::branch::bcs(self);
                    }
                    0xea => {
                        // nop -- implied
                        self.take_cycles(2);

                        self.set_last_instr_disasm_str("nop");
                    }
                    0x18 => {
                        // clc -- implied
                        self.processor_status.carry_flag = false;
                        self.take_cycles(2);

                        self.set_last_instr_disasm_str("clc");
                    }
                    0x90 => {
                        // bcc -- relative
                        instructions::branch::bcc(self);
                    }
                    0xa9 => {
                        // lda -- immediate
                        let immediate = self.next_signed_word();

                        self.processor_status.zero = immediate == 0;
                        self.processor_status.negative = immediate < 0;

                        self.reg_accumulator = immediate;
                        self.take_cycles(2);

                        self.set_last_instr_disasm(format!("lda {:#x}", immediate));
                    }
                    0xf0 => {
                        // beq -- relative
                        instructions::branch::beq(self);
                    }
                    0xd0 => {
                        // bne -- relative
                        instructions::branch::bne(self);
                    }
                    0x85 => {
                        // sta -- zero page
                        let address = self.next_word() as u16;
                        let acc = self.reg_accumulator;

                        self.write(address, acc as u8);
                        self.take_cycles(3);

                        self.set_last_instr_disasm(format!("sta {:#x}", address));
                    }
                    0x24 => {
                        // bit -- zero page
                        instructions::bit::bit_zero_page(self);
                    }
                    0x70 => {
                        // bvs -- relative
                        instructions::branch::bvs(self);
                    }
                    0x50 => {
                        // bvc -- relative
                        instructions::branch::bvc(self);
                    }
                    0x69 => {
                        // adc -- immediate
                        instructions::adc::immediate(self);
                    }
                    0x10 => {
                        // bpl -- relative
                        instructions::branch::bpl(self);
                    }
                    0x60 => {
                        // rts -- implied

                        // get the stored return address, then add 1 to go to next instr
                        let return_addr = self.pop_u16() + 1;

                        self.reg_program_counter = return_addr;
                        self.take_cycles(6);

                        self.set_last_instr_disasm(format!("rts {:#x}", return_addr));
                    }
                    0x78 => {
                        // sei -- implied
                        self.processor_status.interrupts_disabled = true;
                        self.take_cycles(2);

                        self.set_last_instr_disasm_str("sei");
                    }
                    0xf8 => {
                        // sed -- implied
                        self.processor_status.decimal_mode = true;
                        self.take_cycles(2);

                        self.set_last_instr_disasm_str("sed");
                    }
                    0x8 => {
                        // php -- implied
                        let mut status = self.processor_status.clone();
                        status.bit_four = true;

                        self.push(ProcessorStatus::to_u8(&status));
                        self.take_cycles(3);

                        self.set_last_instr_disasm_str("php");
                    }
                    0x68 => {
                        // pla -- implied
                        self.reg_accumulator = self.pop() as i8;
                        self.take_cycles(4);

                        self.set_last_instr_disasm_str("pla");
                    }
                    0x29 => {
                        // and -- immediate

                        let immediate = self.next_signed_word();
                        let acc = self.reg_accumulator;
                        let result = acc & immediate;

                        self.processor_status.negative = result < 0;
                        self.processor_status.zero = result == 0;

                        self.reg_accumulator = result;
                        self.take_cycles(2);

                        self.set_last_instr_disasm(format!("and {:#x}", immediate));
                    }
                    0xc9 => {
                        // cmp -- immediate

                        let immediate = self.next_signed_word();
                        let acc = self.reg_accumulator;
                        let (result, carry) = acc.overflowing_sub(immediate);

                        self.processor_status.negative = result < 0;
                        self.processor_status.zero = result == 0;
                        self.processor_status.carry_flag = carry;

                        self.take_cycles(2);

                        self.set_last_instr_disasm(format!("cmp {:#x}", immediate));
                    }
                    0xd8 => {
                        // cld -- implied

                        self.processor_status.decimal_mode = false;

                        self.take_cycles(2);

                        self.set_last_instr_disasm_str("cld");
                    }
                    0x48 => {
                        // pha -- implied

                        let acc = self.reg_accumulator;
                        self.push(acc as u8);

                        self.take_cycles(3);

                        self.set_last_instr_disasm_str("pha");
                    }
                    0x28 => {
                        // plp -- implied

                        let status = self.pop();
                        self.processor_status = ProcessorStatus::from_u8(status);

                        self.take_cycles(4);

                        self.set_last_instr_disasm_str("plp");
                    }
                    _ => panic!("unknown opcode: {:#x}", &opcode),
                };

                self.debug_write_instr(pc);
            }
            _ => {}
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

    pub fn init_registers(&mut self) {
        // set pc to prg_rom start address
        self.reg_program_counter = memory_map::PRG_ROM_START;

        // set sp to top of stack
        self.reg_stack_pointer = memory_map::STACK_END as u8;
    }

    fn take_cycles(&mut self, cycles: u8) {
        self.pending_cycles = cycles;
    }

    fn finish_cycle(&mut self) {
        self.pending_cycles -= 1;
    }

    // Memory Operations

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

    fn next_signed_word(&mut self) -> i8 {
        self.next_word() as i8
    }

    fn add_relative_address(&self, relative_address: i16) -> u16 {
        match relative_address >= 0 {
            true => self.reg_program_counter.wrapping_add(relative_address as u16),
            false => self.reg_program_counter.wrapping_sub(relative_address as u16),
        }
    }

    // General Methods

    #[allow(unused_must_use)]
    fn debug_write_instr(&self, pc: u16) {
        io::stdout().write(format!("{:#x}: {}\n", pc, self.last_instr_disasm).as_bytes());
    }
}

// Debug Operations
impl<T: MemoryMapper> CpuDebug for Cpu<T> {
    fn exec_instr(&mut self, instruction: &[u8]) {
        for i in 0..instruction.len() {
            self.memory_map.write(self.reg_program_counter + (i as u16), instruction[i]);
        }

        self.step_instruction();
    }

    fn set_last_instr_disasm(&mut self, disassembly: String) {
        self.last_instr_disasm = disassembly;
    }

    fn set_last_instr_disasm_str(&mut self, disassembly: &'static str) {
        self.last_instr_disasm = disassembly.to_string();
    }
}

// Stack Operations
impl<T: MemoryMapper> CpuStack for Cpu<T> {
    fn resolve_stack_pointer(&self) -> u16 {
        memory_map::STACK_START + (self.reg_stack_pointer as u16)
    }

    fn push(&mut self, value: u8) {
        // write to sp
        let sp = self.resolve_stack_pointer();
        self.write(sp, value);

        // decrement sp
        self.reg_stack_pointer -= 1;
    }

    fn push_u16(&mut self, value: u16) {
        // decrement sp
        self.reg_stack_pointer -= 2;

        // write to sp + 1
        let sp = self.resolve_stack_pointer();
        self.write_u16(sp + 1, value);
    }

    fn pop(&mut self) -> u8 {
        self.reg_stack_pointer += 1;
        let val = self.read(self.resolve_stack_pointer());

        val
    }

    fn pop_u16(&mut self) -> u16 {
        self.reg_stack_pointer += 2;

        let val = self.read_u16(self.resolve_stack_pointer() - 1);

        val
    }
}