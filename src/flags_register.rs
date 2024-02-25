const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

pub struct FlagsRegister {
    pub zero: bool, // set to true if operation is equal to 0
    pub subtract: bool, // set to true if the operation was a subtraction
    pub half_carry: bool, // set to true if lower nibble resulted in an overflow
    pub carry: bool, // set to true if the operation resulted in an overflow
}

impl std::convert::From<&FlagsRegister> for u8 {
    fn from(value: &FlagsRegister) -> u8 {
        let mut flag_byte: u8 = 0;
        if value.zero {
            flag_byte |= (1 as u8) << ZERO_FLAG_BYTE_POSITION;
        }
        if value.subtract {
            flag_byte |= (1 as u8) << SUBTRACT_FLAG_BYTE_POSITION;
        }
        if value.half_carry {
            flag_byte |= (1 as u8) << HALF_CARRY_FLAG_BYTE_POSITION;
        }
        if value.carry {
            flag_byte |= (1 as u8) << CARRY_FLAG_BYTE_POSITION;
        }
        flag_byte
    }
}

impl std::convert::From<u8> for FlagsRegister {
    fn from(byte: u8) -> FlagsRegister {
        let zero = ((byte >> ZERO_FLAG_BYTE_POSITION) & 0b1) != 0;
        let subtract = ((byte >> ZERO_FLAG_BYTE_POSITION) & 0b1) != 0;
        let half_carry = ((byte >> ZERO_FLAG_BYTE_POSITION) & 0b1) != 0;
        let carry = ((byte >> ZERO_FLAG_BYTE_POSITION) & 0b1) != 0;
        FlagsRegister {
            zero,
            subtract,
            half_carry,
            carry
        }
    }
}