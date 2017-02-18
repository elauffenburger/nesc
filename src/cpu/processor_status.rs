#[derive(Default, Debug)]
pub struct ProcessorStatus {
    // Carry Flag (C)
    pub carry_flag: bool,

    // Zero Flag (Z)
    pub last_instruction_zero: bool,

    // Interrupt Disable (I)
    pub interrupts_disabled: bool,

    // Decimal Mode (D)
    pub decimal_mode: bool,

    // Break Command (B)
    pub break_instruction_executed: bool,

    // Overflow Flag (V)
    pub overflow_flag: bool,

    // Negative Flag (N)
    pub last_operation_result_negative: bool,
}

impl ProcessorStatus {}
