mod memory;
mod cpu;

use crate::memory::create_memory;
use crate::cpu::CPU;

fn main() {
    let mut cpu = CPU::new(create_memory(64));

    // Instruction mov lit r1.
    cpu.set_memory(0, 0x10);
    cpu.set_memory(1, 0x12); // 0x1234
    cpu.set_memory(2, 0x34);

    // Instruction mov lit r2
    cpu.set_memory(3, 0x11);
    cpu.set_memory(4, 0xAB); // 0x1234
    cpu.set_memory(5, 0xCD);

    // Instruction add reg reg
    cpu.set_memory(6, 0x12);
    cpu.set_memory(7, 2); // r1 idx 
    cpu.set_memory(8, 3); // r2 idx

    // cpu.peak_mem();
    cpu.debug();

    cpu.step();
    cpu.debug();

    cpu.step();
    cpu.debug();
    //cpu.peak_reg();

    cpu.step();
    cpu.debug();
    //cpu.peak_reg();
}

