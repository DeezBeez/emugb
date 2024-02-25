pub enum Instruction {
    ADD(ArithmeticTarget),
    INC,
    RCL,
    JP(JumpTest)
}

pub enum ArithmeticTarget {
    A, B, C, D, E, H, L,
}

pub enum JumpTest {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always
}

impl Instruction {
    pub fn from_byte(byte: u8, prefixed: bool) -> Option<Instruction> {
        if prefixed {
            Instruction::from_byte_prefixed(byte)
        } else {
            Instruction::from_byte_not_prefixed(byte)
        }
    }

    fn from_byte_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            //0x00 => Some(Instruction::RCL),
            _ => None // TODO: Add all Instructions
        }
    }

    fn from_byte_not_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            //0x02 => Some(Instruction::INC),
            //s0x13 => Some(Instruction::INC),
            _ => None // TODO: Add all Instructions
        }
    }
}