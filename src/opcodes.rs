use crate::{
    cpu::Cpu,
    registers::{Reg16, Reg8},
};

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
    Rl(Reg8, bool),
    Rr(Reg8, bool),
    Sla(Reg8),
    Sra(Reg8),
    Srl(Reg8),

    // Load Instructions
    // bool for enable checking if address is btwn FF00 and FFFF
    Ld(Reg8, Reg8, bool),
    LdMem(u8, Reg8),
    Ld16(Reg16, Reg16),

    // Jumps and Subroutines
    // bool to check for condition
    Call(u16, bool),
    Jmp(Reg16, bool),
    JmpR(Reg16, bool),
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
    pub fn from_byte(byte: u8) -> Option<OP> {
        match byte {
            0x00 => Some(OP::Nop),
            _ => panic!("Unable to convert byte to opcode: {:X}", byte),
        }
    }
}
