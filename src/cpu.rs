extern crate byteorder;

use crate::memory::{create_memory, Memory};
use byteorder::{BigEndian, ByteOrder};

#[derive(Debug, Clone)]
pub enum Register {
    IP,
    ACC,
    R1, R2, R3, R4,
    R5, R6, R7, R8
}

impl Register {
    fn to_idx(self) -> usize {
        return self as usize * 2; 
    }
    fn from_idx(idx: u8) -> Option<Self> {
        match idx {
            0 => Some(Register::IP),
            1 => Some(Register::ACC),
            2 => Some(Register::R1),
            3 => Some(Register::R2),
            4 => Some(Register::R3),
            5 => Some(Register::R4),
            6 => Some(Register::R5),
            7 => Some(Register::R6),
            8 => Some(Register::R7),
            9 => Some(Register::R8),
            _ => None 
        }
    }
}

pub enum Instruction {
    MovLitR1 = 0x10,
    MovLitR2 = 0x11,
    AddRegReg = 0x12,
}

#[derive(Debug)]
pub struct CPU {
    memory: Memory, 
    registers: Memory,
}

impl CPU {
    pub fn new(memory: Memory) -> Self {
        let memory_size = 2 * (Register::R8.to_idx() + 1);

        return CPU {
            memory,
            registers: create_memory(memory_size)
        };
    }

    pub fn set_memory(&mut self, idx: usize, value: u8) {
        self.memory[idx] = value;
    }

    pub fn debug(&self) {
        for reg in &[
            Register::IP, 
            Register::ACC, 
            Register::R1, 
            Register::R2, 
            Register::R3, 
            Register::R4, 
            Register::R5, 
            Register::R6, 
            Register::R7, 
            Register::R8
        ] {
            println!("{:?}: 0x{:04x}", reg, self.get_register(reg.clone()));
        }
        println!();
    }

    fn get_register(&self, register: Register) -> u16 {
        let idx = register.to_idx();
        let val =  BigEndian::read_u16(&self.registers[idx..idx+2]);

        return val;
    }

    fn set_register(&mut self, register: Register, value: u16) {
        let idx = register.to_idx();
        return BigEndian::write_u16(&mut self.registers[idx..idx+2], value);
    }

    fn fetch(&mut self) -> u8 {
        let instruction_addr = self.get_register(Register::IP) as usize;
        let instruction = self.memory[instruction_addr];
        self.set_register(Register::IP, instruction_addr as u16 + 1);

        return instruction;
    }

    fn fetch16(&mut self) -> u16 {
        let instruction_addr = self.get_register(Register::IP) as usize;
        let instruction = BigEndian::read_u16(&self.memory[instruction_addr..instruction_addr+2]);
        self.set_register(Register::IP, instruction_addr as u16 + 2);

        return instruction;
    }

    pub fn execute(&mut self, instruction: u8) {
        match instruction {
            //Instruction::MovLitR1 => {
            0x10 => {
                let literal = self.fetch16();
                self.set_register(Register::R1, literal);
            },
            //Instruction::MovLitR2 => {
            0x11 => {
                let literal = self.fetch16();
                self.set_register(Register::R2, literal);
            },
            //Instruction::AddRegReg => {
            0x12 => {
                let r1_idx = self.fetch();
                let r2_idx = self.fetch();
                let register_value1 = self.get_register(Register::from_idx(r1_idx).unwrap());
                let register_value2 = self.get_register(Register::from_idx(r2_idx).unwrap());
                self.set_register(Register::ACC, register_value1 + register_value2);
            }
            _ => {
                println!("No instruction was matched....")
            },
        }
    }

    pub fn peak_mem(&self) {
        println!("{:?}", self.memory);
    }

    pub fn peak_reg(&self) {
        let _ = self.registers.iter()
            .zip(self.registers.iter().skip(1))
            .inspect(|(a, b)| {
                println!("{:X}{:X}", a, b);
            }).collect::<Vec<_>>();
    }

    pub fn step(&mut self) {
        let instruction = self.fetch();
        self.execute(instruction);
    }
}
