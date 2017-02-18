pub trait CpuDebug {
    fn exec_instr(&mut self, instruction: &[u8]);
    fn set_last_instr_disasm(&mut self, disassembly: String);
    fn set_last_instr_disasm_str(&mut self, disassembly: &'static str);
}