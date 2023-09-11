extern crate byteorder;

use byteorder::{ByteOrder, BigEndian};


#[derive(Debug)]
pub struct Memory {
    data: Vec<u8>,
}

impl Memory {
    pub fn new(size: usize) -> Memory {
        return Memory {
            data: vec![0; size],
        }
    }

    pub fn write_u8(&mut self, idx: usize, value: u8) {
        self.data[idx] = value;
    }

    pub fn read_u8(&self, idx: usize) -> u8 {
        return self.data[idx];
    }

    pub fn read_u16(&self, idx: usize) -> u16 {
        return BigEndian::read_u16(&self.data[idx..idx+2]);
    }

    pub fn debug(&self) {
        println!("------- Memory:");
        println!("{:?}", self.data.iter().map(|v| format!("0x{:02x}", v)).collect::<Vec<_>>());
    }
}

