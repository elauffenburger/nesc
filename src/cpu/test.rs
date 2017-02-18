#[allow(unused_imports)]
use ::memory_map;
#[allow(unused_imports)]
use super::Cpu;

#[test]
fn test_push_pop() {
    let mut cpu: Cpu<memory_map::NROMMemoryMap> = Cpu::default();
    cpu.init_registers();

    assert_eq!(&cpu.resolve_stack_pointer(), &memory_map::STACK_END);

    cpu.push(42);
    assert_eq!(&cpu.resolve_stack_pointer(), &(memory_map::STACK_END - 1));

    assert_eq!(&cpu.pop(), &42);
    assert_eq!(&cpu.resolve_stack_pointer(), &memory_map::STACK_END);
}

#[test]
fn test_push_pop_u16() {
    let mut cpu: Cpu<memory_map::NROMMemoryMap> = Cpu::default();
    cpu.init_registers();

    assert_eq!(&cpu.resolve_stack_pointer(), &memory_map::STACK_END);

    cpu.push_u16(42);
    assert_eq!(&cpu.resolve_stack_pointer(), &(memory_map::STACK_END - 2));

    assert_eq!(&cpu.pop_u16(), &42);
    assert_eq!(&cpu.resolve_stack_pointer(), &memory_map::STACK_END);
}

#[test]
fn test_push_pop_mixed() {
    let mut cpu: Cpu<memory_map::NROMMemoryMap> = Cpu::default();
    cpu.init_registers();

    assert_eq!(&cpu.resolve_stack_pointer(), &memory_map::STACK_END);

    cpu.push_u16(42);
    assert_eq!(&cpu.resolve_stack_pointer(), &(memory_map::STACK_END - 2));

    cpu.push(23);
    assert_eq!(&cpu.resolve_stack_pointer(), &(memory_map::STACK_END - 3));

    cpu.push_u16(1991);
    assert_eq!(&cpu.resolve_stack_pointer(), &(memory_map::STACK_END - 5));

    assert_eq!(&cpu.pop_u16(), &1991);
    assert_eq!(&cpu.resolve_stack_pointer(), &(memory_map::STACK_END - 3));

    assert_eq!(&cpu.pop(), &23);
    assert_eq!(&cpu.resolve_stack_pointer(), &(memory_map::STACK_END - 2));

    assert_eq!(&cpu.pop_u16(), &42);
    assert_eq!(&cpu.resolve_stack_pointer(), &memory_map::STACK_END);
}

#[test]
fn test_exec_instr() {
    let mut cpu: Cpu<memory_map::NROMMemoryMap> = Cpu::default();
    cpu.init_registers();

    // lda #$fe
    cpu.exec_instr(&vec![0xa9, 0xfe]);
    assert_eq!(cpu.reg_accumulator, 0xfe);

    // wait for cpu to finish executing
    cpu.step_instruction();

    // sta $05
    cpu.exec_instr(&vec![0x85, 0x05]);
    assert_eq!(cpu.read(0x05), 0xfe);
}