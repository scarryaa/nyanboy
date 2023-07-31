#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Reg8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Reg16 {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Flag {
    Z,
    N,
    H,
    C,
    NZ,
    NC,
}
