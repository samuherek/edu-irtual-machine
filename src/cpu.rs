extern crate byteorder;

use crate::memory::Memory;
use crate::register::{Register, RegisterKey};


#[derive(Debug)]
pub struct CPU<'a> {
    memory: &'a Memory, 
    registers: Register,
    stack_frame_size: usize,
}

impl<'a> CPU<'a> {
    pub fn new(memory: &'a Memory) -> Self {
        return CPU {
            memory,
            registers: Register::new(), 
            stack_frame_size: 0,
        };
    }

    fn get_register(&self, key: RegisterKey) -> u16 {
        return self.registers.read_u16(key);
    }

    fn set_register(&mut self, key: RegisterKey, value: u16) {
        self.registers.write_u16(key, value);
    }

    fn fetch(&mut self) -> u8 {
        let instruction_addr = self.get_register(RegisterKey::InstructionPointer) as usize;
        let instruction = self.memory.read_u8(instruction_addr);
        self.set_register(RegisterKey::InstructionPointer, instruction_addr as u16 + 1);

        return instruction;
    }

    fn fetch16(&mut self) -> u16 {
        let instruction_addr = self.get_register(RegisterKey::InstructionPointer) as usize;
        let instruction = self.memory.read_u16(instruction_addr);
        self.set_register(RegisterKey::InstructionPointer, instruction_addr as u16 + 2);

        return instruction;
    }



    pub fn execute(&mut self, instruction: u8) {
        match instruction {
            //Instruction::MovLitR1 => {
            0x10 => {
                let literal = self.fetch16();
                self.set_register(RegisterKey::R1, literal);
            },
            //Instruction::MovLitR2 => {
            0x11 => {
                let literal = self.fetch16();
                self.set_register(RegisterKey::R2, literal);
            },
            //Instruction::AddRegReg => {
            0x12 => {
                let r1_idx = self.fetch();
                let r2_idx = self.fetch();
                let register_value1 = self.get_register(RegisterKey::from_idx(r1_idx).unwrap());
                let register_value2 = self.get_register(RegisterKey::from_idx(r2_idx).unwrap());
                self.set_register(RegisterKey::Accumulator, register_value1 + register_value2);
            }
            _ => {
                println!("No instruction was matched....")
            },
        }
    }

    pub fn step(&mut self) {
        let instruction = self.fetch();
        self.execute(instruction);
    }

    pub fn debug(&self) {
        self.registers.debug();
    }
}


