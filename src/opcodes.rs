use crate::registers::{Flag, Reg16, Reg8};

#[derive(Debug)]
pub enum OP {
    // 8-bit Arithmetic and Logic
    AdcAR8(Reg8),
    AdcAHL,
    AdcAImm(u8),
    AddR8(Reg8),
    AddAHL,
    AddAImm(u8),
    AddR16(Reg16),
    AndAR8(Reg8),
    AndAHL,
    AndAImm(u8),
    CpAR8(Reg8),
    CpAHL,
    CpAImm(u8),
    DecR8(Reg8),
    DecHL,
    IncR8(Reg8),
    IncHL,
    OrAR8(Reg8),
    OrAHL,
    OrAImm(u8),
    SbcAR8(Reg8),
    SbcAHL,
    SbcAImm(u8),
    SubAR8(Reg8),
    SubAHL,
    SubAImm(u8),
    XorAR8(Reg8),
    XorAHL,
    XorAImm(u8),
    AdcR8(Reg8),
    AdcR16(Reg16),
    SubR8(Reg8),
    SubR16(Reg16),
    SbcR8(Reg8),
    SbcR16(Reg16),

    // 16-bit Arithmetic and Logic
    AddHLR16(Reg16),
    DecR16(Reg16),
    IncR16(Reg16),

    // Bit Operations
    BitBR8(u8, Reg8),
    BitBHL(u8),
    ResBR8(u8, Reg8),
    ResBHL(u8),
    SetBR8(u8, Reg8),
    SetBHL(u8),
    SwapR8(Reg8),
    SwapHL,

    // Bit Shift
    RlR8(Reg8),
    RlHL,
    RlA,
    RlcR8(Reg8),
    RlcHL,
    RlcA,
    RrR8(Reg8),
    RrHL,
    RrA,
    RrcR8(Reg8),
    RrcHL,
    RrcA,
    SlaR8(Reg8),
    SlaHL,
    SrAR8(Reg8),
    SrAHL,
    SrlR8(Reg8),
    SrlHL,

    // Load instructions
    LdR8R8(Reg8, Reg8),
    LdMemR8(Reg16, Reg8),
    LdR8Mem(Reg8, Reg16),
    LdR8Imm(Reg8, u8),
    LdR16Imm(Reg16, u16),
    LdHLR8(Reg8),
    LdHLImm(u8),
    LdR8HL(Reg8),
    LdR16A(Reg16),
    LdR16R8(Reg16, Reg8),
    LdImmA(u16),
    LdhImmA(u8),
    LdhCA,
    LdAR16(Reg16),
    LdAImm(u16),
    LdhAImm(u16),
    LdhAC,
    LdHLIA,
    LdHLDA,
    LdAHLI,
    LdAHLD,

    // Jumps and Subroutines
    Call(u16),
    CallCond(Flag, u16),
    JpHL,
    JpImm(u16),
    JpCond(Flag, u16),
    Jr(u16),
    JrCond(Flag, u16),
    RetCond(Flag),
    Ret,
    Reti,
    Rst(u16),

    // Stack Operations
    AddHLSP,
    AddSPImm(i8),
    DecSP,
    IncSP,
    LdSPImm(u16),
    LdImmSP(u16),
    LdHLSPImm(i8),
    LdSPHL,
    PopAF,
    PopR16(Reg16),
    PushAF,
    PushR16(Reg16),

    // Misc
    Ccf,
    Cpl,
    Daa,
    Di,
    Ei,
    Halt,
    Nop,
    Scf,
    Stop,

    AndR8(Reg8),
    AndR16(Reg16),
    XorR8(Reg8),
    XorR16(Reg16),
    OrR8(Reg8),
    OrR16(Reg16),
    CpR8(Reg8),
    CpR16(Reg16),
    JPCondImm16(Flag),
    CallCondImm16(Flag),
    JPImm16,
    AddImm8,
    CBPrefix,
    CallImm16,
    AdcImm8,
    SubImm8,
    SbcImm8,
    LdIOImm8A,
    LdIOC,
    AndImm8,
    AddSPImm8,
    LdImm16A,
    JPHL,
    XorImm8,
    LdAIOImm8,
    LdACIO,
    OrImm8,
    LdHLSPImm8,
    LdAImm16,
    CpImm8,
}

impl OP {
    pub fn from_bytes(bytes: &[u8]) -> Option<(OP, usize, usize)> {
        let n = *bytes.get(1)?;
        let n16 = ((*bytes.get(1)? as u16) << 8) | (*bytes.get(2)? as u16);
        let rel = n as i8;

        match bytes.get(0)? {
            0x00 => Some((OP::Nop, 1, 4)),
            0x01 => Some((OP::LdR16Imm(Reg16::BC, n16), 3, 12)),
            0x02 => Some((OP::LdMemR8(Reg16::BC, Reg8::A), 1, 8)),
            0x03 => Some((OP::IncR16(Reg16::BC), 1, 8)),
            0x04 => Some((OP::IncR8(Reg8::B), 1, 4)),
            0x05 => Some((OP::DecR8(Reg8::B), 1, 4)),
            0x06 => Some((OP::LdR8Imm(Reg8::B, n), 2, 8)),
            0x07 => Some((OP::RlR8(Reg8::A), 1, 4)),
            0x08 => Some((OP::LdImmSP(n16), 3, 20)),
            0x09 => Some((OP::AddHLR16(Reg16::BC), 1, 8)),
            0x0A => Some((OP::LdR8Mem(Reg8::A, Reg16::BC), 1, 8)),
            0x0B => Some((OP::DecR16(Reg16::BC), 1, 8)),
            0x0C => Some((OP::IncR8(Reg8::C), 1, 4)),
            0x0D => Some((OP::DecR8(Reg8::C), 1, 4)),
            0x0E => Some((OP::LdR8Imm(Reg8::C, n), 2, 8)),
            0x0F => Some((OP::RrR8(Reg8::A), 1, 4)),

            0x10 => Some((OP::Stop, 2, 4)),
            0x11 => Some((OP::LdR16Imm(Reg16::DE, n16), 3, 12)),
            0x12 => Some((OP::LdMemR8(Reg16::DE, Reg8::A), 1, 8)),
            0x13 => Some((OP::IncR16(Reg16::DE), 1, 8)),
            0x14 => Some((OP::IncR8(Reg8::D), 1, 4)),
            0x15 => Some((OP::DecR8(Reg8::D), 1, 4)),
            0x16 => Some((OP::LdR8Imm(Reg8::D, n), 2, 8)),
            0x17 => Some((OP::RlA, 1, 4)),
            0x18 => Some((OP::Jr(rel as u16), 2, 12)),
            0x19 => Some((OP::AddHLR16(Reg16::DE), 1, 8)),
            0x1A => Some((OP::LdR8Mem(Reg8::A, Reg16::DE), 1, 8)),
            0x1B => Some((OP::DecR16(Reg16::DE), 1, 8)),
            0x1C => Some((OP::IncR8(Reg8::E), 1, 4)),
            0x1D => Some((OP::DecR8(Reg8::E), 1, 4)),
            0x1E => Some((OP::LdR8Imm(Reg8::E, n), 2, 8)),
            0x1F => Some((OP::RrA, 1, 4)),

            0x20 => Some((OP::JrCond(Flag::NZ, rel as u16), 2, 8)),
            0x21 => Some((OP::LdR16Imm(Reg16::HL, n16), 3, 12)),
            0x22 => Some((OP::LdHLIA, 1, 8)),
            0x23 => Some((OP::IncR16(Reg16::HL), 1, 8)),
            0x24 => Some((OP::IncR8(Reg8::H), 1, 4)),
            0x25 => Some((OP::DecR8(Reg8::H), 1, 4)),
            0x26 => Some((OP::LdR8Imm(Reg8::H, n), 2, 8)),
            0x27 => Some((OP::Daa, 1, 4)),
            0x28 => Some((OP::JrCond(Flag::Z, rel as u16), 2, 8)),
            0x29 => Some((OP::AddHLR16(Reg16::HL), 1, 8)),
            0x2A => Some((OP::LdAHLI, 1, 8)),
            0x2B => Some((OP::DecR16(Reg16::HL), 1, 8)),
            0x2C => Some((OP::IncR8(Reg8::L), 1, 4)),
            0x2D => Some((OP::DecR8(Reg8::L), 1, 4)),
            0x2E => Some((OP::LdR8Imm(Reg8::L, n), 2, 8)),
            0x2F => Some((OP::Cpl, 1, 4)),

            0x30 => Some((OP::JrCond(Flag::NC, rel as u16), 2, 8)),
            0x31 => Some((OP::LdR16Imm(Reg16::SP, n16), 3, 12)),
            0x32 => Some((OP::LdHLDA, 1, 8)),
            0x33 => Some((OP::IncR16(Reg16::SP), 1, 8)),
            0x34 => Some((OP::IncHL, 1, 12)),
            0x35 => Some((OP::DecHL, 1, 12)),
            0x36 => Some((OP::LdHLImm(n), 2, 12)),
            0x37 => Some((OP::Scf, 1, 4)),
            0x38 => Some((OP::JrCond(Flag::C, rel as u16), 2, 8)),
            0x39 => Some((OP::AddHLR16(Reg16::SP), 1, 8)),
            0x3A => Some((OP::LdAHLD, 1, 8)),
            0x3B => Some((OP::DecR16(Reg16::SP), 1, 8)),
            0x3C => Some((OP::IncR8(Reg8::A), 1, 4)),
            0x3D => Some((OP::DecR8(Reg8::A), 1, 4)),
            0x3E => Some((OP::LdR8Imm(Reg8::A, n), 2, 8)),
            0x3F => Some((OP::Ccf, 1, 4)),

            0x40 => Some((OP::LdR8R8(Reg8::B, Reg8::B), 1, 4)),
            0x41 => Some((OP::LdR8R8(Reg8::B, Reg8::C), 1, 4)),
            0x42 => Some((OP::LdR8R8(Reg8::B, Reg8::D), 1, 4)),
            0x43 => Some((OP::LdR8R8(Reg8::B, Reg8::E), 1, 4)),
            0x44 => Some((OP::LdR8R8(Reg8::B, Reg8::H), 1, 4)),
            0x45 => Some((OP::LdR8R8(Reg8::B, Reg8::L), 1, 4)),
            0x46 => Some((OP::LdR8Mem(Reg8::B, Reg16::HL), 1, 8)),
            0x47 => Some((OP::LdR8R8(Reg8::B, Reg8::A), 1, 4)),
            0x48 => Some((OP::LdR8R8(Reg8::C, Reg8::B), 1, 4)),
            0x49 => Some((OP::LdR8R8(Reg8::C, Reg8::C), 1, 4)),
            0x4A => Some((OP::LdR8R8(Reg8::C, Reg8::D), 1, 4)),
            0x4B => Some((OP::LdR8R8(Reg8::C, Reg8::E), 1, 4)),
            0x4C => Some((OP::LdR8R8(Reg8::C, Reg8::H), 1, 4)),
            0x4D => Some((OP::LdR8R8(Reg8::C, Reg8::L), 1, 4)),
            0x4E => Some((OP::LdR8Mem(Reg8::C, Reg16::HL), 1, 8)),
            0x4F => Some((OP::LdR8R8(Reg8::C, Reg8::A), 1, 4)),

            0x50 => Some((OP::LdR8R8(Reg8::D, Reg8::B), 1, 4)),
            0x51 => Some((OP::LdR8R8(Reg8::D, Reg8::C), 1, 4)),
            0x52 => Some((OP::LdR8R8(Reg8::D, Reg8::D), 1, 4)),
            0x53 => Some((OP::LdR8R8(Reg8::D, Reg8::E), 1, 4)),
            0x54 => Some((OP::LdR8R8(Reg8::D, Reg8::H), 1, 4)),
            0x55 => Some((OP::LdR8R8(Reg8::D, Reg8::L), 1, 4)),
            0x56 => Some((OP::LdR8Mem(Reg8::D, Reg16::HL), 1, 8)),
            0x57 => Some((OP::LdR8R8(Reg8::D, Reg8::A), 1, 4)),
            0x58 => Some((OP::LdR8R8(Reg8::E, Reg8::B), 1, 4)),
            0x59 => Some((OP::LdR8R8(Reg8::E, Reg8::C), 1, 4)),
            0x5A => Some((OP::LdR8R8(Reg8::E, Reg8::D), 1, 4)),
            0x5B => Some((OP::LdR8R8(Reg8::E, Reg8::E), 1, 4)),
            0x5C => Some((OP::LdR8R8(Reg8::E, Reg8::H), 1, 4)),
            0x5D => Some((OP::LdR8R8(Reg8::E, Reg8::L), 1, 4)),
            0x5E => Some((OP::LdR8Mem(Reg8::E, Reg16::HL), 1, 8)),
            0x5F => Some((OP::LdR8R8(Reg8::E, Reg8::A), 1, 4)),

            0x60 => Some((OP::LdR8R8(Reg8::H, Reg8::B), 1, 4)),
            0x61 => Some((OP::LdR8R8(Reg8::H, Reg8::C), 1, 4)),
            0x62 => Some((OP::LdR8R8(Reg8::H, Reg8::D), 1, 4)),
            0x63 => Some((OP::LdR8R8(Reg8::H, Reg8::E), 1, 4)),
            0x64 => Some((OP::LdR8R8(Reg8::H, Reg8::H), 1, 4)),
            0x65 => Some((OP::LdR8R8(Reg8::H, Reg8::L), 1, 4)),
            0x66 => Some((OP::LdR8Mem(Reg8::H, Reg16::HL), 1, 8)),
            0x67 => Some((OP::LdR8R8(Reg8::H, Reg8::A), 1, 4)),
            0x68 => Some((OP::LdR8R8(Reg8::L, Reg8::B), 1, 4)),
            0x69 => Some((OP::LdR8R8(Reg8::L, Reg8::C), 1, 4)),
            0x6A => Some((OP::LdR8R8(Reg8::L, Reg8::D), 1, 4)),
            0x6B => Some((OP::LdR8R8(Reg8::L, Reg8::E), 1, 4)),
            0x6C => Some((OP::LdR8R8(Reg8::L, Reg8::H), 1, 4)),
            0x6D => Some((OP::LdR8R8(Reg8::L, Reg8::L), 1, 4)),
            0x6E => Some((OP::LdR8Mem(Reg8::L, Reg16::HL), 1, 8)),
            0x6F => Some((OP::LdR8R8(Reg8::L, Reg8::A), 1, 4)),

            0x70 => Some((OP::LdR16R8(Reg16::HL, Reg8::B), 1, 8)),
            0x71 => Some((OP::LdR16R8(Reg16::HL, Reg8::C), 1, 8)),
            0x72 => Some((OP::LdR16R8(Reg16::HL, Reg8::D), 1, 8)),
            0x73 => Some((OP::LdR16R8(Reg16::HL, Reg8::E), 1, 8)),
            0x74 => Some((OP::LdR16R8(Reg16::HL, Reg8::H), 1, 8)),
            0x75 => Some((OP::LdR16R8(Reg16::HL, Reg8::L), 1, 8)),
            0x76 => Some((OP::Halt, 1, 4)),
            0x77 => Some((OP::LdR16R8(Reg16::HL, Reg8::A), 1, 8)),
            0x78 => Some((OP::LdR8R8(Reg8::A, Reg8::B), 1, 4)),
            0x79 => Some((OP::LdR8R8(Reg8::A, Reg8::C), 1, 4)),
            0x7A => Some((OP::LdR8R8(Reg8::A, Reg8::D), 1, 4)),
            0x7B => Some((OP::LdR8R8(Reg8::A, Reg8::E), 1, 4)),
            0x7C => Some((OP::LdR8R8(Reg8::A, Reg8::H), 1, 4)),
            0x7D => Some((OP::LdR8R8(Reg8::A, Reg8::L), 1, 4)),
            0x7E => Some((OP::LdR8Mem(Reg8::A, Reg16::HL), 1, 8)),
            0x7F => Some((OP::LdR8R8(Reg8::A, Reg8::A), 1, 4)),

            0x80 => Some((OP::AddR8(Reg8::B), 1, 4)),
            0x81 => Some((OP::AddR8(Reg8::C), 1, 4)),
            0x82 => Some((OP::AddR8(Reg8::D), 1, 4)),
            0x83 => Some((OP::AddR8(Reg8::E), 1, 4)),
            0x84 => Some((OP::AddR8(Reg8::H), 1, 4)),
            0x85 => Some((OP::AddR8(Reg8::L), 1, 4)),
            0x86 => Some((OP::AddR16(Reg16::HL), 1, 8)),
            0x87 => Some((OP::AddR8(Reg8::A), 1, 4)),
            0x88 => Some((OP::AdcR8(Reg8::B), 1, 4)),
            0x89 => Some((OP::AdcR8(Reg8::C), 1, 4)),
            0x8A => Some((OP::AdcR8(Reg8::D), 1, 4)),
            0x8B => Some((OP::AdcR8(Reg8::E), 1, 4)),
            0x8C => Some((OP::AdcR8(Reg8::H), 1, 4)),
            0x8D => Some((OP::AdcR8(Reg8::L), 1, 4)),
            0x8E => Some((OP::AdcR16(Reg16::HL), 1, 8)),
            0x8F => Some((OP::AdcR8(Reg8::A), 1, 4)),

            0x90 => Some((OP::SubR8(Reg8::B), 1, 4)),
            0x91 => Some((OP::SubR8(Reg8::C), 1, 4)),
            0x92 => Some((OP::SubR8(Reg8::D), 1, 4)),
            0x93 => Some((OP::SubR8(Reg8::E), 1, 4)),
            0x94 => Some((OP::SubR8(Reg8::H), 1, 4)),
            0x95 => Some((OP::SubR8(Reg8::L), 1, 4)),
            0x96 => Some((OP::SubR16(Reg16::HL), 1, 8)),
            0x97 => Some((OP::SubR8(Reg8::A), 1, 4)),
            0x98 => Some((OP::SbcR8(Reg8::B), 1, 4)),
            0x99 => Some((OP::SbcR8(Reg8::C), 1, 4)),
            0x9A => Some((OP::SbcR8(Reg8::D), 1, 4)),
            0x9B => Some((OP::SbcR8(Reg8::E), 1, 4)),
            0x9C => Some((OP::SbcR8(Reg8::H), 1, 4)),
            0x9D => Some((OP::SbcR8(Reg8::L), 1, 4)),
            0x9E => Some((OP::SbcR16(Reg16::HL), 1, 8)),
            0x9F => Some((OP::SbcR8(Reg8::A), 1, 4)),

            0xA0 => Some((OP::AndR8(Reg8::B), 1, 4)),
            0xA1 => Some((OP::AndR8(Reg8::C), 1, 4)),
            0xA2 => Some((OP::AndR8(Reg8::D), 1, 4)),
            0xA3 => Some((OP::AndR8(Reg8::E), 1, 4)),
            0xA4 => Some((OP::AndR8(Reg8::H), 1, 4)),
            0xA5 => Some((OP::AndR8(Reg8::L), 1, 4)),
            0xA6 => Some((OP::AndR16(Reg16::HL), 1, 8)),
            0xA7 => Some((OP::AndR8(Reg8::A), 1, 4)),
            0xA8 => Some((OP::XorR8(Reg8::B), 1, 4)),
            0xA9 => Some((OP::XorR8(Reg8::C), 1, 4)),
            0xAA => Some((OP::XorR8(Reg8::D), 1, 4)),
            0xAB => Some((OP::XorR8(Reg8::E), 1, 4)),
            0xAC => Some((OP::XorR8(Reg8::H), 1, 4)),
            0xAD => Some((OP::XorR8(Reg8::L), 1, 4)),
            0xAE => Some((OP::XorR16(Reg16::HL), 1, 8)),
            0xAF => Some((OP::XorR8(Reg8::A), 1, 4)),

            0xB0 => Some((OP::OrR8(Reg8::B), 1, 4)),
            0xB1 => Some((OP::OrR8(Reg8::C), 1, 4)),
            0xB2 => Some((OP::OrR8(Reg8::D), 1, 4)),
            0xB3 => Some((OP::OrR8(Reg8::E), 1, 4)),
            0xB4 => Some((OP::OrR8(Reg8::H), 1, 4)),
            0xB5 => Some((OP::OrR8(Reg8::L), 1, 4)),
            0xB6 => Some((OP::OrR16(Reg16::HL), 1, 8)),
            0xB7 => Some((OP::OrR8(Reg8::A), 1, 4)),
            0xB8 => Some((OP::CpR8(Reg8::B), 1, 4)),
            0xB9 => Some((OP::CpR8(Reg8::C), 1, 4)),
            0xBA => Some((OP::CpR8(Reg8::D), 1, 4)),
            0xBB => Some((OP::CpR8(Reg8::E), 1, 4)),
            0xBC => Some((OP::CpR8(Reg8::H), 1, 4)),
            0xBD => Some((OP::CpR8(Reg8::L), 1, 4)),
            0xBE => Some((OP::CpR16(Reg16::HL), 1, 8)),
            0xBF => Some((OP::CpR8(Reg8::A), 1, 4)),

            0xC0 => Some((OP::RetCond(Flag::NZ), 1, 20)),
            0xC1 => Some((OP::PopR16(Reg16::BC), 1, 12)),
            0xC2 => Some((OP::JPCondImm16(Flag::NZ), 3, 16)),
            0xC3 => Some((OP::JPImm16, 3, 16)),
            0xC4 => Some((OP::CallCondImm16(Flag::NZ), 3, 24)),
            0xC5 => Some((OP::PushR16(Reg16::BC), 1, 16)),
            0xC6 => Some((OP::AddImm8, 2, 8)),
            0xC7 => Some((OP::Rst(0x00), 1, 16)),
            0xC8 => Some((OP::RetCond(Flag::Z), 1, 20)),
            0xC9 => Some((OP::Ret, 1, 16)),
            0xCA => Some((OP::JPCondImm16(Flag::Z), 3, 16)),
            0xCB => Some((OP::CBPrefix, 1, 4)),
            0xCC => Some((OP::CallCondImm16(Flag::Z), 3, 24)),
            0xCD => Some((OP::CallImm16, 3, 24)),
            0xCE => Some((OP::AdcImm8, 2, 8)),
            0xCF => Some((OP::Rst(0x08), 1, 16)),

            0xD0 => Some((OP::RetCond(Flag::NC), 1, 20)),
            0xD1 => Some((OP::PopR16(Reg16::DE), 1, 12)),
            0xD2 => Some((OP::JPCondImm16(Flag::NC), 3, 16)),
            0xD3 => None,
            0xD4 => Some((OP::CallCondImm16(Flag::NC), 3, 24)),
            0xD5 => Some((OP::PushR16(Reg16::DE), 1, 16)),
            0xD6 => Some((OP::SubImm8, 2, 8)),
            0xD7 => Some((OP::Rst(0x10), 1, 16)),
            0xD8 => Some((OP::RetCond(Flag::C), 1, 20)),
            0xD9 => Some((OP::Reti, 1, 16)),
            0xDA => Some((OP::JPCondImm16(Flag::C), 3, 16)),
            0xDB => None,
            0xDC => Some((OP::CallCondImm16(Flag::C), 3, 24)),
            0xDD => None,
            0xDE => Some((OP::SbcImm8, 2, 8)),
            0xDF => Some((OP::Rst(0x18), 1, 16)),

            0xE0 => Some((OP::LdIOImm8A, 2, 12)),
            0xE1 => Some((OP::PopR16(Reg16::HL), 1, 12)),
            0xE2 => Some((OP::LdIOC, 1, 8)),
            0xE3 => None,
            0xE4 => None,
            0xE5 => Some((OP::PushR16(Reg16::HL), 1, 16)),
            0xE6 => Some((OP::AndImm8, 2, 8)),
            0xE7 => Some((OP::Rst(0x20), 1, 16)),
            0xE8 => Some((OP::AddSPImm8, 2, 16)),
            0xE9 => Some((OP::JPHL, 1, 4)),
            0xEA => Some((OP::LdImm16A, 3, 16)),
            0xEB => None,
            0xEC => None,
            0xED => None,
            0xEE => Some((OP::XorImm8, 2, 8)),
            0xEF => Some((OP::Rst(0x28), 1, 16)),

            0xF0 => Some((OP::LdAIOImm8, 2, 12)),
            0xF1 => Some((OP::PopR16(Reg16::AF), 1, 12)),
            0xF2 => Some((OP::LdACIO, 1, 8)),
            0xF3 => Some((OP::Di, 1, 4)),
            0xF4 => None,
            0xF5 => Some((OP::PushR16(Reg16::AF), 1, 16)),
            0xF6 => Some((OP::OrImm8, 2, 8)),
            0xF7 => Some((OP::Rst(0x30), 1, 16)),
            0xF8 => Some((OP::LdHLSPImm8, 2, 12)),
            0xF9 => Some((OP::LdSPHL, 1, 8)),
            0xFA => Some((OP::LdAImm16, 3, 16)),
            0xFB => Some((OP::Ei, 1, 4)),
            0xFC => None,
            0xFD => None,
            0xFE => Some((OP::CpImm8, 2, 8)),
            0xFF => Some((OP::Rst(0x38), 1, 16)),
        }
    }
}
