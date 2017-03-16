use ::memory_map;
use ::memory_map::MemoryMapper;
use ::cpu::Cpu;

use ::cpu::CpuDebug;

fn do_branch_instruction<T: MemoryMapper>(cpu: &mut Cpu<T>, relative_address: i8, take_branch: bool) -> u16 {
    let absolute_address = cpu.add_relative_address(relative_address as i16);

    let num_cycles = match () {
        _ if !take_branch => 2,
        _ => {

            let num_cycles = match memory_map::crosses_page_boundary(cpu.reg_program_counter, absolute_address) {
                false => 3,
                true => 4,
            };

            cpu.reg_program_counter = absolute_address;

            num_cycles
        }
    };

    cpu.take_cycles(num_cycles);

    absolute_address
}

fn do_branch_overflow_instruction<T: MemoryMapper>(cpu: &mut Cpu<T>, branch_if_flag_set: bool) -> u16 {
    let relative_address = cpu.next_signed_word();
    let take_branch = cpu.processor_status.overflow_flag == branch_if_flag_set;

    do_branch_instruction(cpu, relative_address, take_branch)
}

fn do_branch_carry_instruction<T: MemoryMapper>(cpu: &mut Cpu<T>, branch_if_flag_set: bool) -> u16 {
    // relative values are signed
    let relative_address = cpu.next_signed_word();

    let is_set = cpu.processor_status.carry_flag;
    let take_branch = branch_if_flag_set == is_set;

    do_branch_instruction(cpu, relative_address, take_branch)
}

pub fn bcs<T: MemoryMapper>(cpu: &mut Cpu<T>) {
    let absolute_address = do_branch_carry_instruction(cpu, true);

    cpu.set_last_instr_disasm(format!("bcs {:#x}", absolute_address));
}

pub fn bcc<T: MemoryMapper>(cpu: &mut Cpu<T>) {
    let absolute_address = do_branch_carry_instruction(cpu, false);

    cpu.set_last_instr_disasm(format!("bcc {:#x}", absolute_address));
}

pub fn beq<T: MemoryMapper>(cpu: &mut Cpu<T>) {
    let relative_address = cpu.next_signed_word();
    let take_branch = cpu.processor_status.zero;

    let absolute_address = do_branch_instruction(cpu, relative_address, take_branch);

    cpu.set_last_instr_disasm(format!("beq {:#x}", absolute_address));
}

pub fn bne<T: MemoryMapper>(cpu: &mut Cpu<T>) {
    let relative_address = cpu.next_signed_word();
    let take_branch = !cpu.processor_status.zero;

    let absolute_address = do_branch_instruction(cpu, relative_address, take_branch);

    cpu.set_last_instr_disasm(format!("bne {:#x}", absolute_address));
}

pub fn bvs<T: MemoryMapper>(cpu: &mut Cpu<T>) {
    let absolute_addr = do_branch_overflow_instruction(cpu, true);

    cpu.set_last_instr_disasm(format!("bvs {:#x}", absolute_addr));
}

pub fn bvc<T: MemoryMapper>(cpu: &mut Cpu<T>) {
    let absolute_addr = do_branch_overflow_instruction(cpu, false);

    cpu.set_last_instr_disasm(format!("bvc {:#x}", absolute_addr));
}

pub fn bpl<T: MemoryMapper>(cpu: &mut Cpu<T>) {
    let relative_address = cpu.next_word() as i8;
    let take_branch = cpu.processor_status.negative == false;

    let absolute_addr = do_branch_instruction(cpu, relative_address, take_branch);

    cpu.set_last_instr_disasm(format!("bpl {:#x}", absolute_addr));
} 