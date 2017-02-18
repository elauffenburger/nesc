pub trait CpuStack {
    fn push(&mut self, value: u8);
    fn push_u16(&mut self, value: u16);
    fn pop(&mut self) -> u8;
    fn pop_u16(&mut self) -> u16;
    fn resolve_stack_pointer(&self) -> u16;
}