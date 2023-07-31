use crate::registers::{Flag, Reg16, Reg8};

#[derive(Debug)]
pub enum OP {
    // 8-bit Arithmetic and Logic Instructions
    Adc(Reg8, Reg8),
    Adc8Mem(Reg8, Reg16),
    Add(Reg8, Reg8),
    Add8Mem(Reg8, Reg16),
    And(Reg8, Reg8),
    And8Mem(Reg8, Reg16),
    Cp(Reg8, Reg8),
    Cp8Mem(Reg8, Reg16),
    Dec(Reg8),
    Inc(Reg8),
    Or(Reg8, Reg8),
    Or8Mem(Reg8, Reg16),
    Sbc(Reg8, Reg8),
    Sbc8Mem(Reg8, Reg16),
    Sub(Reg8, Reg8),
    Sub8Mem(Reg8, Reg16),
    Xor(Reg8, Reg8),
    Xor8Mem(Reg8, Reg16),

    // 16-bit Arithmetic and Logic Instructions
    Add16(Reg16, Reg16),
    Dec16(Reg16),
    Inc16(Reg16),
    Dec16Mem(Reg16),
    Inc16Mem(Reg16),

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
    LdMemImm(Reg16, u8),
    LdMem8(Reg16, Reg8),
    Ld8Mem(Reg8, Reg16),
    Ld16(Reg16, Reg16),
    LdImm16(Reg16, u16),
    LdMem16(u16, Reg16),

    // Special cases
    LdHLI(Reg16, Reg8),
    Ld8HLD(Reg8, Reg16),
    Ld8HLI(Reg8, Reg16),
    LdHLD(Reg16, Reg8),

    // Jumps and Subroutines
    // bool to check for condition
    Call(u16, bool),
    Jmp(Flag, i8),
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
            0x19 => Some((OP::Add16(Reg16::HL, Reg16::DE), 1, 8)),
            0x1A => Some((OP::Ld8Mem(Reg8::A, Reg16::DE), 1, 8)),
            0x1B => Some((OP::Dec16(Reg16::DE), 1, 8)),
            0x1C => Some((OP::Inc(Reg8::E), 1, 4)),
            0x1D => Some((OP::Dec(Reg8::E), 1, 4)),
            0x1E => Some((OP::LdImm(Reg8::E, n), 2, 8)),
            0x1F => Some((OP::Rr(Reg8::A), 1, 4)),
            0x20 => Some((OP::Jmp(Flag::NZ, rel), 2, 12)),
            0x21 => Some((OP::LdImm16(Reg16::HL, n16), 3, 12)),
            0x22 => Some((OP::LdHLI(Reg16::HL, Reg8::A), 1, 8)),
            0x23 => Some((OP::Inc16(Reg16::HL), 1, 8)),
            0x24 => Some((OP::Inc(Reg8::H), 1, 4)),
            0x25 => Some((OP::Dec(Reg8::H), 1, 4)),
            0x26 => Some((OP::LdImm(Reg8::H, n), 2, 8)),
            0x27 => Some((OP::Daa, 1, 4)),
            0x28 => Some((OP::Jmp(Flag::Z, rel), 2, 12)),
            0x29 => Some((OP::Add16(Reg16::HL, Reg16::HL), 1, 8)),
            0x2A => Some((OP::Ld8HLI(Reg8::A, Reg16::HL), 1, 8)),
            0x2B => Some((OP::Dec16(Reg16::HL), 1, 8)),
            0x2C => Some((OP::Inc(Reg8::L), 1, 4)),
            0x2D => Some((OP::Dec(Reg8::L), 1, 4)),
            0x2E => Some((OP::LdImm(Reg8::L, n), 2, 8)),
            0x2F => Some((OP::Cpl, 1, 4)),
            0x30 => Some((OP::Jmp(Flag::NC, rel), 2, 12)),
            0x31 => Some((OP::LdImm16(Reg16::SP, n16), 3, 12)),
            0x32 => Some((OP::LdHLD(Reg16::HL, Reg8::A), 1, 8)),
            0x33 => Some((OP::Inc16(Reg16::SP), 1, 8)),
            0x34 => Some((OP::Inc16Mem(Reg16::HL), 1, 12)),
            0x35 => Some((OP::Dec16Mem(Reg16::HL), 1, 12)),
            0x36 => Some((OP::LdMemImm(Reg16::HL, n), 2, 12)),
            0x37 => Some((OP::Scf, 1, 4)),
            0x38 => Some((OP::Jmp(Flag::C, rel), 2, 12)),
            0x39 => Some((OP::Add16(Reg16::HL, Reg16::SP), 1, 8)),
            0x3A => Some((OP::Ld8HLD(Reg8::A, Reg16::HL), 1, 8)),
            0x3B => Some((OP::Dec16(Reg16::SP), 1, 8)),
            0x3C => Some((OP::Inc(Reg8::A), 1, 4)),
            0x3D => Some((OP::Dec(Reg8::A), 1, 4)),
            0x3E => Some((OP::LdImm(Reg8::A, n), 2, 8)),
            0x3F => Some((OP::Ccf, 1, 4)),
            0x40 => Some((OP::Ld(Reg8::B, Reg8::B), 1, 4)),
            0x41 => Some((OP::Ld(Reg8::B, Reg8::C), 1, 4)),
            0x42 => Some((OP::Ld(Reg8::B, Reg8::D), 1, 4)),
            0x43 => Some((OP::Ld(Reg8::B, Reg8::E), 1, 4)),
            0x44 => Some((OP::Ld(Reg8::B, Reg8::H), 1, 4)),
            0x45 => Some((OP::Ld(Reg8::B, Reg8::L), 1, 4)),
            0x46 => Some((OP::Ld8Mem(Reg8::B, Reg16::HL), 1, 8)),
            0x47 => Some((OP::Ld(Reg8::B, Reg8::A), 1, 4)),
            0x48 => Some((OP::Ld(Reg8::C, Reg8::B), 1, 4)),
            0x49 => Some((OP::Ld(Reg8::C, Reg8::C), 1, 4)),
            0x4A => Some((OP::Ld(Reg8::C, Reg8::D), 1, 4)),
            0x4B => Some((OP::Ld(Reg8::C, Reg8::E), 1, 4)),
            0x4C => Some((OP::Ld(Reg8::C, Reg8::H), 1, 4)),
            0x4D => Some((OP::Ld(Reg8::C, Reg8::L), 1, 4)),
            0x4E => Some((OP::Ld8Mem(Reg8::C, Reg16::HL), 1, 8)),
            0x4F => Some((OP::Ld(Reg8::C, Reg8::A), 1, 4)),
            0x50 => Some((OP::Ld(Reg8::D, Reg8::B), 1, 4)),
            0x51 => Some((OP::Ld(Reg8::D, Reg8::C), 1, 4)),
            0x52 => Some((OP::Ld(Reg8::D, Reg8::D), 1, 4)),
            0x53 => Some((OP::Ld(Reg8::D, Reg8::E), 1, 4)),
            0x54 => Some((OP::Ld(Reg8::D, Reg8::H), 1, 4)),
            0x55 => Some((OP::Ld(Reg8::D, Reg8::L), 1, 4)),
            0x56 => Some((OP::Ld8Mem(Reg8::D, Reg16::HL), 1, 8)),
            0x57 => Some((OP::Ld(Reg8::D, Reg8::A), 1, 4)),
            0x58 => Some((OP::Ld(Reg8::E, Reg8::B), 1, 4)),
            0x59 => Some((OP::Ld(Reg8::E, Reg8::C), 1, 4)),
            0x5A => Some((OP::Ld(Reg8::E, Reg8::D), 1, 4)),
            0x5B => Some((OP::Ld(Reg8::E, Reg8::E), 1, 4)),
            0x5C => Some((OP::Ld(Reg8::E, Reg8::H), 1, 4)),
            0x5D => Some((OP::Ld(Reg8::E, Reg8::L), 1, 4)),
            0x5E => Some((OP::Ld8Mem(Reg8::E, Reg16::HL), 1, 8)),
            0x5F => Some((OP::Ld(Reg8::E, Reg8::A), 1, 4)),
            0x60 => Some((OP::Ld(Reg8::H, Reg8::B), 1, 4)),
            0x61 => Some((OP::Ld(Reg8::H, Reg8::C), 1, 4)),
            0x62 => Some((OP::Ld(Reg8::H, Reg8::D), 1, 4)),
            0x63 => Some((OP::Ld(Reg8::H, Reg8::E), 1, 4)),
            0x64 => Some((OP::Ld(Reg8::H, Reg8::H), 1, 4)),
            0x65 => Some((OP::Ld(Reg8::H, Reg8::L), 1, 4)),
            0x66 => Some((OP::Ld8Mem(Reg8::H, Reg16::HL), 1, 8)),
            0x67 => Some((OP::Ld(Reg8::H, Reg8::A), 1, 4)),
            0x68 => Some((OP::Ld(Reg8::L, Reg8::B), 1, 4)),
            0x69 => Some((OP::Ld(Reg8::L, Reg8::C), 1, 4)),
            0x6A => Some((OP::Ld(Reg8::L, Reg8::D), 1, 4)),
            0x6B => Some((OP::Ld(Reg8::L, Reg8::E), 1, 4)),
            0x6C => Some((OP::Ld(Reg8::L, Reg8::H), 1, 4)),
            0x6D => Some((OP::Ld(Reg8::L, Reg8::L), 1, 4)),
            0x6E => Some((OP::Ld8Mem(Reg8::L, Reg16::HL), 1, 8)),
            0x6F => Some((OP::Ld(Reg8::L, Reg8::A), 1, 4)),
            0x70 => Some((OP::LdMem8(Reg16::HL, Reg8::B), 1, 8)),
            0x71 => Some((OP::LdMem8(Reg16::HL, Reg8::C), 1, 8)),
            0x72 => Some((OP::LdMem8(Reg16::HL, Reg8::D), 1, 8)),
            0x73 => Some((OP::LdMem8(Reg16::HL, Reg8::E), 1, 8)),
            0x74 => Some((OP::LdMem8(Reg16::HL, Reg8::H), 1, 8)),
            0x75 => Some((OP::LdMem8(Reg16::HL, Reg8::L), 1, 8)),
            0x76 => Some((OP::Halt, 1, 4)),
            0x77 => Some((OP::LdMem8(Reg16::HL, Reg8::A), 1, 8)),
            0x78 => Some((OP::Ld(Reg8::A, Reg8::B), 1, 4)),
            0x79 => Some((OP::Ld(Reg8::A, Reg8::C), 1, 4)),
            0x7A => Some((OP::Ld(Reg8::A, Reg8::D), 1, 4)),
            0x7B => Some((OP::Ld(Reg8::A, Reg8::E), 1, 4)),
            0x7C => Some((OP::Ld(Reg8::A, Reg8::H), 1, 4)),
            0x7D => Some((OP::Ld(Reg8::A, Reg8::L), 1, 4)),
            0x7E => Some((OP::Ld8Mem(Reg8::A, Reg16::HL), 1, 8)),
            0x7F => Some((OP::Ld(Reg8::A, Reg8::A), 1, 4)),

            0x80 => Some((OP::Add(Reg8::A, Reg8::B), 1, 4)),
            0x81 => Some((OP::Add(Reg8::A, Reg8::C), 1, 4)),
            0x82 => Some((OP::Add(Reg8::A, Reg8::D), 1, 4)),
            0x83 => Some((OP::Add(Reg8::A, Reg8::E), 1, 4)),
            0x84 => Some((OP::Add(Reg8::A, Reg8::H), 1, 4)),
            0x85 => Some((OP::Add(Reg8::A, Reg8::L), 1, 4)),
            0x86 => Some((OP::Add8Mem(Reg8::A, Reg16::HL), 1, 8)),
            0x87 => Some((OP::Add(Reg8::A, Reg8::A), 1, 4)),
            0x88 => Some((OP::Adc(Reg8::A, Reg8::B), 1, 4)),
            0x89 => Some((OP::Adc(Reg8::A, Reg8::C), 1, 4)),
            0x8A => Some((OP::Adc(Reg8::A, Reg8::D), 1, 4)),
            0x8B => Some((OP::Adc(Reg8::A, Reg8::E), 1, 4)),
            0x8C => Some((OP::Adc(Reg8::A, Reg8::H), 1, 4)),
            0x8D => Some((OP::Adc(Reg8::A, Reg8::L), 1, 4)),
            0x8E => Some((OP::Adc8Mem(Reg8::A, Reg16::HL), 1, 8)),
            0x8F => Some((OP::Adc(Reg8::A, Reg8::A), 1, 4)),
            0x90 => Some((OP::Sub(Reg8::A, Reg8::B), 1, 4)),
            0x91 => Some((OP::Sub(Reg8::A, Reg8::C), 1, 4)),
            0x92 => Some((OP::Sub(Reg8::A, Reg8::D), 1, 4)),
            0x93 => Some((OP::Sub(Reg8::A, Reg8::E), 1, 4)),
            0x94 => Some((OP::Sub(Reg8::A, Reg8::H), 1, 4)),
            0x95 => Some((OP::Sub(Reg8::A, Reg8::L), 1, 4)),
            0x96 => Some((OP::Sub8Mem(Reg8::A, Reg16::HL), 1, 8)),
            0x97 => Some((OP::Sub(Reg8::A, Reg8::A), 1, 4)),
            0x98 => Some((OP::Sbc(Reg8::A, Reg8::B), 1, 4)),
            0x99 => Some((OP::Sbc(Reg8::A, Reg8::C), 1, 4)),
            0x9A => Some((OP::Sbc(Reg8::A, Reg8::D), 1, 4)),
            0x9B => Some((OP::Sbc(Reg8::A, Reg8::E), 1, 4)),
            0x9C => Some((OP::Sbc(Reg8::A, Reg8::H), 1, 4)),
            0x9D => Some((OP::Sbc(Reg8::A, Reg8::L), 1, 4)),
            0x9E => Some((OP::Sbc8Mem(Reg8::A, Reg16::HL), 1, 8)),
            0x9F => Some((OP::Sbc(Reg8::A, Reg8::A), 1, 4)),
            0xA0 => Some((OP::And(Reg8::A, Reg8::B), 1, 4)),
            0xA1 => Some((OP::And(Reg8::A, Reg8::C), 1, 4)),
            0xA2 => Some((OP::And(Reg8::A, Reg8::D), 1, 4)),
            0xA3 => Some((OP::And(Reg8::A, Reg8::E), 1, 4)),
            0xA4 => Some((OP::And(Reg8::A, Reg8::H), 1, 4)),
            0xA5 => Some((OP::And(Reg8::A, Reg8::L), 1, 4)),
            0xA6 => Some((OP::And8Mem(Reg8::A, Reg16::HL), 1, 8)),
            0xA7 => Some((OP::And(Reg8::A, Reg8::A), 1, 4)),
            0xA8 => Some((OP::Xor(Reg8::A, Reg8::B), 1, 4)),
            0xA9 => Some((OP::Xor(Reg8::A, Reg8::C), 1, 4)),
            0xAA => Some((OP::Xor(Reg8::A, Reg8::D), 1, 4)),
            0xAB => Some((OP::Xor(Reg8::A, Reg8::E), 1, 4)),
            0xAC => Some((OP::Xor(Reg8::A, Reg8::H), 1, 4)),
            0xAD => Some((OP::Xor(Reg8::A, Reg8::L), 1, 4)),
            0xAE => Some((OP::Xor8Mem(Reg8::A, Reg16::HL), 1, 8)),
            0xAF => Some((OP::Xor(Reg8::A, Reg8::A), 1, 4)),
            0xB0 => Some((OP::Or(Reg8::A, Reg8::B), 1, 4)),
            0xB1 => Some((OP::Or(Reg8::A, Reg8::C), 1, 4)),
            0xB2 => Some((OP::Or(Reg8::A, Reg8::D), 1, 4)),
            0xB3 => Some((OP::Or(Reg8::A, Reg8::E), 1, 4)),
            0xB4 => Some((OP::Or(Reg8::A, Reg8::H), 1, 4)),
            0xB5 => Some((OP::Or(Reg8::A, Reg8::L), 1, 4)),
            0xB6 => Some((OP::Or8Mem(Reg8::A, Reg16::HL), 1, 8)),
            0xB7 => Some((OP::Or(Reg8::A, Reg8::A), 1, 4)),
            0xB8 => Some((OP::Cp(Reg8::A, Reg8::B), 1, 4)),
            0xB9 => Some((OP::Cp(Reg8::A, Reg8::C), 1, 4)),
            0xBA => Some((OP::Cp(Reg8::A, Reg8::D), 1, 4)),
            0xBB => Some((OP::Cp(Reg8::A, Reg8::E), 1, 4)),
            0xBC => Some((OP::Cp(Reg8::A, Reg8::H), 1, 4)),
            0xBD => Some((OP::Cp(Reg8::A, Reg8::L), 1, 4)),
            0xBE => Some((OP::Cp8Mem(Reg8::A, Reg16::HL), 1, 8)),
            0xBF => Some((OP::Cp(Reg8::A, Reg8::A), 1, 4)),
            _ => None,
        }
    }
}
