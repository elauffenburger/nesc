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
    pub fn to_u8(status: &ProcessorStatus) -> u8 {
        // the starting result
        let mut result = 0b0010_0000;

        if status.negative {
            result |= 1 << 7;
        }

        if status.overflow_flag {
            result |= 1 << 6;
        }

        if status.bit_four {
            result |= 1 << 4;
        }

        if status.decimal_mode {
            result |= 1 << 3;
        }

        if status.interrupts_disabled {
            result |= 1 << 2;
        }

        if status.zero {
            result |= 1 << 1;
        }

        if status.carry_flag {
            result |= 1;
        }

        result
    }

    fn test_bit(val: u8, test: u8) -> bool {
        val & (1 << test) > 0
    }

    pub fn from_u8(status: u8) -> Self {
        let mut result = ProcessorStatus::default();

        result.negative = ProcessorStatus::test_bit(status, 7);
        result.overflow_flag = ProcessorStatus::test_bit(status, 6);
        result.bit_four = ProcessorStatus::test_bit(status, 4);
        result.decimal_mode = ProcessorStatus::test_bit(status, 3);
        result.interrupts_disabled = ProcessorStatus::test_bit(status, 2);
        result.zero = ProcessorStatus::test_bit(status, 1);
        result.carry_flag = ProcessorStatus::test_bit(status, 0);

        result
    }
}

#[cfg(test)]
mod test {
    use super::ProcessorStatus;

    #[test]
    pub fn test_from_u8() {
        let result: ProcessorStatus = ProcessorStatus::from_u8(0b1010_1111);

        assert_eq!(result.negative, true);
        assert_eq!(result.overflow_flag, false);
        assert_eq!(result.bit_four, false);
        assert_eq!(result.decimal_mode, true);
        assert_eq!(result.interrupts_disabled, true);
        assert_eq!(result.zero, true);
        assert_eq!(result.carry_flag, true);
    }

    #[test]
    pub fn test_symmetry() {
        let status = 0b1010_1111;
        assert_eq!(ProcessorStatus::to_u8(&ProcessorStatus::from_u8(status)),
                   status);
    }
}