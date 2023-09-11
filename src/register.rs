extern crate byteorder;

use byteorder::{BigEndian, ByteOrder};
use strum_macros::EnumIter;
use strum::IntoEnumIterator;

#[derive(EnumIter, Debug, Clone)]
pub enum RegisterKey {
    InstructionPointer,
    Accumulator,
    R1, R2, R3, R4,
    R5, R6, R7, R8,
    StackPointer, StackFramePointer
}

impl RegisterKey {
    fn to_idx(self) -> usize {
        return self as usize * 2; 
    }
    pub fn from_idx(idx: u8) -> Option<Self> {
        match idx {
            0 => Some(RegisterKey::InstructionPointer),
            1 => Some(RegisterKey::Accumulator),
            2 => Some(RegisterKey::R1),
            3 => Some(RegisterKey::R2),
            4 => Some(RegisterKey::R3),
            5 => Some(RegisterKey::R4),
            6 => Some(RegisterKey::R5),
            7 => Some(RegisterKey::R6),
            8 => Some(RegisterKey::R7),
            9 => Some(RegisterKey::R8),
            10 => Some(RegisterKey::StackPointer),
            11 => Some(RegisterKey::StackFramePointer),
            _ => None 
        }
    }
}

#[derive(Debug)]
pub struct Register {
    data: Vec<u8>
}

impl Register {
   pub fn new() -> Register {
        let register_size = RegisterKey::iter().count() * 2;

        return Register {
            data: vec![0; register_size] 
        };
   }

   pub fn length(&self) -> usize {
        return self.data.len() / 2;
   }

    pub fn read_u16(&self, key: RegisterKey) -> u16 {
        let idx = key.to_idx();
        return BigEndian::read_u16(&self.data[idx..idx+2]);
    }

    pub fn write_u16(&mut self, key: RegisterKey, value: u16) {
        let idx = key.to_idx();
        BigEndian::write_u16(&mut self.data[idx..idx+2], value);
    }

    pub fn debug(&self) {
        println!("------- Register:");
       for reg in &[
            RegisterKey::InstructionPointer, 
            RegisterKey::Accumulator, 
            RegisterKey::R1, 
            RegisterKey::R2, 
            RegisterKey::R3, 
            RegisterKey::R4, 
            RegisterKey::R5, 
            RegisterKey::R6, 
            RegisterKey::R7, 
            RegisterKey::R8,
            RegisterKey::StackPointer,
            RegisterKey::StackFramePointer
        ] {
            println!("{:?}: 0x{:04x}", reg, self.read_u16(reg.clone()));
        }
    }
}

