use ::cpu::Cpu;
use ::memory_map;
use ::memory_map::MemoryMapper;

pub fn immediate<T: MemoryMapper>(cpu: &mut Cpu<T>) {
    let immediate = cpu.next_signed_word();
    adc(cpu, immediate);

    cpu.take_cycles(2);
}

pub fn zero_page<T: MemoryMapper>(cpu: &mut Cpu<T>) {
    let address = cpu.next_word();
    let value = cpu.read(address as u16);

    adc(cpu, value as i8);

    cpu.take_cycles(3);
}

pub fn zero_page_x<T: MemoryMapper>(cpu: &mut Cpu<T>) {
    let address = cpu.next_word() as u16;
    let x = cpu.reg_index_x as u16;
    let value = cpu.read(address + x);

    adc(cpu, value as i8);

    cpu.take_cycles(4);
}

pub fn abs<T: MemoryMapper>(cpu: &mut Cpu<T>) {
    let address = cpu.next_double_word();
    let value = cpu.read(address);

    adc(cpu, value as i8);

    cpu.take_cycles(4);
}

pub fn abs_x<T: MemoryMapper>(cpu: &mut Cpu<T>) {
    let abs_addr = cpu.next_double_word();
    let x = cpu.reg_index_x as u16;

    abs_indexed(cpu, abs_addr, x);
}

pub fn abs_y<T: MemoryMapper>(cpu: &mut Cpu<T>) {
    let abs_addr = cpu.next_double_word();
    let y = cpu.reg_index_y as u16;

    abs_indexed(cpu, abs_addr, y);
}

pub fn indirect_x<T: MemoryMapper>(cpu: &mut Cpu<T>) {}

pub fn indirect_y<T: MemoryMapper>(cpu: &mut Cpu<T>) {}

fn adc<T: MemoryMapper>(cpu: &mut Cpu<T>, value: i8) {
    let (result, carry) = cpu.reg_accumulator.overflowing_add(value).0.overflowing_add(cpu.processor_status.carry_flag as i8);

    cpu.reg_accumulator = result;

    cpu.processor_status.carry_flag = carry;


}

fn abs_indexed<T: MemoryMapper>(cpu: &mut Cpu<T>, abs_addr: u16, offset: u16) {
    let indexed_addr = abs_addr + offset;

    let value = cpu.read(indexed_addr);
    adc(cpu, value as i8);

    let cycles = match memory_map::crosses_page_boundary(abs_addr, indexed_addr) {
        false => 4,
        true => 5,
    };

    cpu.take_cycles(cycles);
}

fn indirect<T: MemoryMapper>(cpu: &mut Cpu<T>) {}