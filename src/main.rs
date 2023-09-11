mod memory;
mod register;
mod cpu;
mod instructions;

use crate::memory::Memory;
use crate::cpu::CPU;
use crate::register::RegisterKey;

fn load_program(memory: &mut Memory) {
    // Instruction mov lit r1.
    memory.write_u8(0, 0x10);
    memory.write_u8(1, 0x12);
    memory.write_u8(2, 0x34);

    // Instruction mov lit r2
    memory.write_u8(3, 0x11);
    memory.write_u8(4, 0xAB);
    memory.write_u8(5, 0xCD);

    // Instruction add reg reg
    memory.write_u8(6, 0x12);
    memory.write_u8(7, RegisterKey::R1 as u8);
    memory.write_u8(8, RegisterKey::R2 as u8);
}

fn main() {
    let mut memory = Memory::new(256);
    load_program(&mut memory);

    let mut cpu = CPU::new(&memory);

    cpu.step();
    cpu.step();
    cpu.step();

    cpu.debug();
    memory.debug();
}

