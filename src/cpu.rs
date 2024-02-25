use std::fmt::format;

use crate::register::Registers;
use crate::instruction::*;

struct MemoryBus {
    memory: [u8; 0xFFFF]
}

impl MemoryBus {
    fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
}

struct CPU {
    registers: Registers,
    pc: u16,
    bus: MemoryBus
}

impl CPU {
    fn execute(&mut self, instruction: Instruction) -> u16{
        match instruction {
            Instruction::ADD(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value = self.registers.a;
                        let res = self.add(value);
                        self.registers.a = res;
                        self.pc.wrapping_add(1)
                    },
                    ArithmeticTarget::B => {
                        let value = self.registers.b;
                        let res = self.add(value);
                        self.registers.a = res;
                        self.pc.wrapping_add(1)
                    },
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let res = self.add(value);
                        self.registers.a = res;
                        self.pc.wrapping_add(1)
                    },
                    ArithmeticTarget::D => {
                        let value = self.registers.d;
                        let res = self.add(value);
                        self.registers.a = res;
                        self.pc.wrapping_add(1)
                    },
                    ArithmeticTarget::E => {
                        let value = self.registers.e;
                        let res = self.add(value);
                        self.registers.a = res;
                        self.pc.wrapping_add(1)
                    },
                    ArithmeticTarget::H => {
                        let value = self.registers.h;
                        let res = self.add(value);
                        self.registers.a = res;
                        self.pc.wrapping_add(1)
                    },
                    ArithmeticTarget::L => {
                        let value = self.registers.l;
                        let res = self.add(value);
                        self.registers.a = res;
                        self.pc.wrapping_add(1)
                    }
                }
            }
            _ => { 
                /* TODO: support all instructions */ 
                self.pc
            }
        }
    }

    fn step(&mut self) {
        let mut instruction_byte = self.bus.read_byte(self.pc);
        let prefixed = instruction_byte == 0xCB;
        if prefixed {
            instruction_byte = self.bus.read_byte(self.pc + 1);
        }
        let next_pc = if let Some(instruction) = Instruction::from_byte(instruction_byte, prefixed) {
            self.execute(instruction)
        } else {
            let description = format!("0x{}{:x}", if prefixed {"cb"} else {""}, instruction_byte);
            panic!("Unkown instruction found for: 0x{:x}", instruction_byte);
        };

        self.pc = next_pc;
    }

    // Instructions
    fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.a & 0x0F) + (value & 0x0F) > 0x0F;
        new_value
    } 
    fn jump(&self, should_jump: bool) -> u16 {
        if should_jump {
            let least_significant_byte = self.bus.read_byte(self.pc + 1) as u16;
            let most_significant_byte = self.bus.read_byte(self.pc + 2) as u16;
            (most_significant_byte << 8) | least_significant_byte
        } else {
            self.pc.wrapping_add(3)
        }
    }
}