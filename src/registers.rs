pub struct Register {
    pub high: u8,
    pub low: u8,
}

pub struct Flags {
    pub zero: bool,
    pub subtract: bool,
    pub half_carry: bool,
    pub carry: bool,
}

pub struct Registers {
    pub af: Register,
    pub bc: Register,
    pub de: Register,
    pub hl: Register,
    pub sp: u16,
    pub pc: u16,

    pub f: Flags,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            af: Register {
                high: 0x00,
                low: 0x00,
            },
            bc: Register {
                high: 0x00,
                low: 0x00,
            },
            de: Register {
                high: 0x00,
                low: 0x00,
            },
            hl: Register {
                high: 0x00,
                low: 0x00,
            },
            sp: 0x0000,
            pc: 0x0000,

            f: Flags {
                zero: false,
                subtract: false,
                half_carry: false,
                carry: false,
            },
        }
    }

    pub fn get_af(&self) -> u16 {
        self.af.high as u16 | ((self.af.low as u16) << 8)
    }

    pub fn set_af(&mut self, value: u16) {
        self.af.high = (value >> 8) as u8;
        self.af.low = value as u8;
    }

    pub fn get_bc(&self) -> u16 {
        self.bc.high as u16 | ((self.bc.low as u16) << 8)
    }

    pub fn set_bc(&mut self, value: u16) {
        self.bc.high = (value >> 8) as u8;
        self.bc.low = value as u8;
    }

    pub fn get_de(&self) -> u16 {
        self.de.high as u16 | ((self.de.low as u16) << 8)
    }

    pub fn set_de(&mut self, value: u16) {
        self.de.high = (value >> 8) as u8;
        self.de.low = value as u8;
    }

    pub fn get_hl(&self) -> u16 {
        self.hl.high as u16 | ((self.hl.low as u16) << 8)
    }

    pub fn set_hl(&mut self, value: u16) {
        self.hl.high = (value >> 8) as u8;
        self.hl.low = value as u8;
    }

    pub fn get_flag(&self) -> u8 {
        let mut flag = 0x00;
        if self.f.zero {
            flag |= 0x80;
        }
        if self.f.subtract {
            flag |= 0x40;
        }
        if self.f.half_carry {
            flag |= 0x20;
        }
        if self.f.carry {
            flag |= 0x10;
        }
        flag
    }

    pub fn set_flag(&mut self, value: u8) {
        self.f.zero = value & 0x80 == 0x80;
        self.f.subtract = value & 0x40 == 0x40;
        self.f.half_carry = value & 0x20 == 0x20;
        self.f.carry = value & 0x10 == 0x10;
    }
}
