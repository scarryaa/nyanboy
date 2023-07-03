pub enum Instruction {
    ADD(ArithmeticTarget),
    JP(JumpTest),
    INC(IncDecTarget),
    RLC(PrefixTarget),
    LD(LoadType),
    PUSH(PushTarget),
    POP(PopTarget),
    RET(JumpTest),
    CALL(JumpTest),
    NOP(),
}

impl Instruction {
    pub fn from_byte(byte: u8, prefixed: bool) -> Option<Instruction> {
        if prefixed {
            Instruction::from_byte_prefixed(byte)
        } else {
            Instruction::from_byte_unprefixed(byte)
        }
    }

    pub fn from_byte_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x00 => Some(Instruction::RLC(PrefixTarget::B)),
            _ => None,
        }
    }

    pub fn from_byte_unprefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x02 => Some(Instruction::INC(IncDecTarget::BC)),
            0x13 => Some(Instruction::INC(IncDecTarget::DE)),
            // TODO - Add more instructions
            _ => None,
        }
    }
}

pub enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

pub enum JumpTest {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always,
}

pub enum IncDecTarget {
    BC,
    DE,
    HL,
    SP,
}

pub enum PrefixTarget {
    B,
    C,
    D,
    E,
    H,
    L,
    A,
}

pub enum LoadByteTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HLI,
}

pub enum LoadByteSource {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    D8,
    HLI,
}

pub enum PushTarget {
    BC,
    DE,
    HL,
    AF,
    SP,
}

pub enum PopTarget {
    BC,
    DE,
    HL,
    AF,
    SP,
}

pub enum LoadType {
    Byte(LoadByteTarget, LoadByteSource),
}
