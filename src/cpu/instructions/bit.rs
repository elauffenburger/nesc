use ::cpu::Cpu;
use ::memory_map::MemoryMapper;

use ::cpu::CpuDebug;

pub fn bit_zero_page<T: MemoryMapper>(cpu: &mut Cpu<T>) {
    let address = cpu.next_word() as u16;
    let value = cpu.read(address) as i8;
    let result = cpu.reg_accumulator & value;

    cpu.processor_status.last_instruction_zero = result == 0;
    cpu.processor_status.last_operation_result_negative = result < 0;
    cpu.processor_status.overflow_flag = ((result & (0b0100_0000)) >> 6) == 1;

    cpu.take_cycles(3);

    cpu.set_last_instr_disasm(format!("bit {:#x}", address));
}