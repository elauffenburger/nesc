#[derive(Default, Debug, Clone)]
pub struct ProcessorStatus {
    // Carry Flag (C)
    pub carry_flag: bool,

    // Zero Flag (Z)
    pub zero: bool,

    // Interrupt Disable (I)
    pub interrupts_disabled: bool,

    // Decimal Mode (D)
    pub decimal_mode: bool,

    pub bit_four: bool,

    // Break Command (B)
    pub break_instruction_executed: bool,

    // Overflow Flag (V)
    pub overflow_flag: bool,

    // Negative Flag (N)
    pub negative: bool,
}

impl ProcessorStatus {
    pub fn to_u8(&self) -> u8 {
        // the starting result
        let mut result = 0b0001_0000;

        if self.negative {
            result |= 1 << 7;
        }

        if self.overflow_flag {
            result |= 1 << 6;
        }

        if self.bit_four {
            result |= 1 << 4;
        }

        if self.decimal_mode {
            result |= 1 << 3;
        }

        if self.interrupts_disabled {
            result |= 1 << 2;
        }

        if self.carry_flag {
            result |= 1;
        }

        result
    }
}
