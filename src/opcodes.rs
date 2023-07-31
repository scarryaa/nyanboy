use crate::registers::{Reg16, Reg8};

#[derive(Debug)]
pub enum OP {
    // 8-bit Arithmetic and Logic Instructions
    Adc(Reg8, Reg8),
    Add(Reg8, Reg8),
    And(Reg8, Reg8),
    Cp(Reg8, Reg8),
    Dec(Reg8),
    Inc(Reg8),
    Or(Reg8, Reg8),
    Sbc(Reg8, Reg8),
    Sub(Reg8, Reg8),
    Xor(Reg8, Reg8),

    // 16-bit Arithmetic and Logic Instructions
    Add16(Reg16, Reg16),
    Dec16(Reg16),
    Inc16(Reg16),

    // Bit operations Instructions
    Bit(i32, Reg8),
    Res(i32, Reg8),
    Set(i32, Reg8),
    Swap(Reg8),

    // Bit shift Instructions
    // Rotate left, bool for carry
    Rl(Reg8),
    RlC(Reg8),
    Rr(Reg8),
    RrC(Reg8),
    Sla(Reg8),
    Sra(Reg8),
    Srl(Reg8),

    // Load Instructions
    // bool for enable checking if address is btwn FF00 and FFFF
    Ld(Reg8, Reg8),
    LdImm(Reg8, u8),
    LdMem8(Reg16, Reg8),
    Ld8Mem(Reg8, Reg16),
    Ld16(Reg16, Reg16),
    LdImm16(Reg16, u16),
    LdMem16(u16, Reg16),

    // Jumps and Subroutines
    // bool to check for condition
    Call(u16, bool),
    Jmp(Reg16, bool),
    JmpR(i8),
    Ret(bool),
    RetI,
    Rst(u16),

    // Stack Operations Instructions
    Pop(Reg16),
    Push(Reg16),

    // Misc Instructions
    Ccf,
    Cpl,
    Daa,
    Di,
    Ei,
    Halt,
    Nop,
    Scf,
    Stop,
}

impl OP {
    pub fn from_bytes(bytes: &[u8]) -> Option<(OP, usize, usize)> {
        let n = *bytes.get(1)?;
        let n16 = ((*bytes.get(1)? as u16) << 8) | (*bytes.get(2)? as u16);
        let rel = n as i8;

        match bytes.get(0)? {
            0x00 => Some((OP::Nop, 1, 4)),
            0x01 => Some((OP::LdImm16(Reg16::BC, n16), 3, 12)),
            0x02 => Some((OP::LdMem8(Reg16::BC, Reg8::A), 1, 8)),
            0x03 => Some((OP::Inc16(Reg16::BC), 1, 8)),
            0x04 => Some((OP::Inc(Reg8::B), 1, 4)),
            0x05 => Some((OP::Dec(Reg8::B), 1, 4)),
            0x06 => Some((OP::LdImm(Reg8::B, n), 2, 8)),
            0x07 => Some((OP::RlC(Reg8::A), 1, 4)),
            0x08 => Some((OP::LdMem16(n16, Reg16::SP), 3, 20)),
            0x09 => Some((OP::Add16(Reg16::HL, Reg16::BC), 1, 8)),
            0x0A => Some((OP::Ld8Mem(Reg8::A, Reg16::BC), 1, 8)),
            0x0B => Some((OP::Dec16(Reg16::BC), 1, 8)),
            0x0C => Some((OP::Inc(Reg8::C), 1, 4)),
            0x0D => Some((OP::Dec(Reg8::C), 1, 4)),
            0x0E => Some((OP::LdImm(Reg8::C, n), 2, 8)),
            0x0F => Some((OP::RrC(Reg8::A), 1, 4)),
            0x10 => Some((OP::Stop, 2, 4)),
            0x11 => Some((OP::LdImm16(Reg16::DE, n16), 3, 12)),
            0x12 => Some((OP::LdMem8(Reg16::DE, Reg8::A), 1, 8)),
            0x13 => Some((OP::Inc16(Reg16::DE), 1, 8)),
            0x14 => Some((OP::Inc(Reg8::D), 1, 4)),
            0x15 => Some((OP::Dec(Reg8::D), 1, 4)),
            0x16 => Some((OP::LdImm(Reg8::D, n), 2, 8)),
            0x17 => Some((OP::Rl(Reg8::A), 1, 4)),
            0x18 => Some((OP::JmpR(rel), 2, 12)),
            0x19 => Some((OP::Add16(Reg16::HL, Reg16::BC), 1, 8)),
            _ => None,
        }
    }
}
