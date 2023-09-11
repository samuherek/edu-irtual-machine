
pub enum Instruction {
    MovLitReg   = 0x10,
    MovRegReg   = 0x11,
    MovRegMem   = 0x12,
    MovMemReg   = 0x13,
    AddRegReg   = 0x14,
    JmpNotEq    = 0x15,
    PshLit      = 0x17,
    PshReg      = 0x18,
    Pop         = 0x1A,
    CalLit      = 0x5E,
    CalReg      = 0x5F,
    Ret         = 0x60,
}

impl Instruction {
    pub fn from_u8(val: u8) -> Option<Self> {
        match val {
            0x10 => Some(Instruction::MovLitReg),
            0x11 => Some(Instruction::MovRegReg),
            0x12 => Some(Instruction::MovRegMem),
            0x13 => Some(Instruction::MovMemReg),
            0x14 => Some(Instruction::AddRegReg),
            0x15 => Some(Instruction::JmpNotEq),
            0x17 => Some(Instruction::PshLit),
            0x18 => Some(Instruction::PshReg),
            0x1A => Some(Instruction::Pop),
            0x5E => Some(Instruction::CalLit),
            0x5F => Some(Instruction::CalReg),
            0x60 => Some(Instruction::Ret),
            _ => None
        }
    }
}
