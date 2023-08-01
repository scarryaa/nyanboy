use crate::{opcodes::OP, registers};
use std::{fs::File, io::Read};

pub struct Instruction {
    opcode: OP,
    size: u8,
    duration: u8,
}

pub struct Flags {
    z: bool,
    n: bool,
    h: bool,
    c: bool,
}

pub struct Cpu {
    a: u8,
    f: Flags,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
    pub memory: [u8; 65536],
    stopped: bool,
    halted: bool,
    ime: bool,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            a: 0,
            f: Flags {
                z: false,
                n: false,
                h: false,
                c: false,
            },
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0,
            memory: [0; 65536],
            stopped: false,
            halted: false,
            ime: false,
        }
    }

    pub fn execute(&mut self) {
        let (opcode, size, duration) =
            OP::from_bytes(&self.memory[self.pc as usize..]).expect("Unknown opcode");

        // print!("{:?} ", opcode);
        // println!("Size: {}", size);
        // println!("Duration: {}", duration);
        // println!("PC: {}", self.pc);

        // read from FF01

        match opcode {
            OP::AddR8(reg) => {
                let value = self.get_reg8(reg);
                let result = self.a.wrapping_add(value);

                self.set_all_flags(
                    result == 0,
                    false,
                    (self.a & 0xf) + (value & 0xf) > 0xf,
                    (self.a as u16) + (value as u16) > 0xff,
                );

                self.a = result;
            }
            OP::AddR16(reg) => {
                let address = self.get_reg16(reg);
                let value = self.read_byte(address);
                let result = self.a.wrapping_add(value);

                self.set_all_flags(
                    result == 0,
                    false,
                    (self.a & 0xf) + (value & 0xf) > 0xf,
                    (self.a as u16) + (value as u16) > 0xff,
                );

                self.a = result;
            }
            OP::DecR8(reg) => {
                let value = self.get_reg8(reg);
                let result = value.wrapping_sub(1);

                self.set_all_flags(
                    result == 0,
                    true,
                    (value & 0xf) == 0,
                    self.get_flag(registers::Flag::C),
                );
                self.set_reg8(reg, result);
            }
            OP::DecHL => {
                let address = self.get_reg16(registers::Reg16::HL);
                let value = self.read_byte(address);
                let result = value.wrapping_sub(1);

                self.set_all_flags(
                    result == 0,
                    true,
                    (value & 0xf) == 0,
                    self.get_flag(registers::Flag::C),
                );
                self.write_byte(address, result);
            }
            OP::IncR8(reg) => {
                let value = self.get_reg8(reg);
                let result = value.wrapping_add(1);

                self.set_all_flags(
                    result == 0,
                    false,
                    (value & 0xf) == 0xf,
                    self.get_flag(registers::Flag::C),
                );
            }
            OP::IncHL => {
                let address = self.get_reg16(registers::Reg16::HL);
                let value = self.read_byte(address);
                let result = value.wrapping_add(1);

                self.set_all_flags(
                    result == 0,
                    false,
                    (value & 0xf) == 0xf,
                    self.get_flag(registers::Flag::C),
                );
                self.write_byte(address, result);
            }
            OP::AdcR8(reg) => {
                let value = self.get_reg8(reg);
                let result = self.a.wrapping_add(value) + self.get_flag(registers::Flag::C) as u8;

                self.set_all_flags(
                    result == 0,
                    false,
                    (self.a & 0xf) + (value & 0xf) + self.get_flag(registers::Flag::C) as u8 > 0xf,
                    (self.a as u16) + (value as u16) + self.get_flag(registers::Flag::C) as u16
                        > 0xff,
                );

                self.a = result;
            }
            OP::AdcR16(reg) => {
                let address = self.get_reg16(reg);
                let value = self.read_byte(address);
                let result = self.a.wrapping_add(value) + self.get_flag(registers::Flag::C) as u8;

                self.set_all_flags(
                    result == 0,
                    false,
                    (self.a & 0xf) + (value & 0xf) + self.get_flag(registers::Flag::C) as u8 > 0xf,
                    (self.a as u16) + (value as u16) + self.get_flag(registers::Flag::C) as u16
                        > 0xff,
                );

                self.a = result;
            }
            OP::SubR8(reg) => {
                let value = self.get_reg8(reg);
                let result = self.a.wrapping_sub(value);

                self.set_all_flags(
                    result == 0,
                    true,
                    (self.a & 0xf) < (value & 0xf),
                    (self.a as u16) < (value as u16),
                );

                self.a = result;
            }
            OP::SubR16(reg) => {
                let address = self.get_reg16(reg);
                let value = self.read_byte(address);
                let result = self.a.wrapping_sub(value);

                self.set_all_flags(
                    result == 0,
                    true,
                    (self.a & 0xf) < (value & 0xf),
                    (self.a as u16) < (value as u16),
                );

                self.a = result;
            }
            OP::SbcR8(reg) => {
                let value = self.get_reg8(reg);
                let result = self.a.wrapping_sub(value) - self.get_flag(registers::Flag::C) as u8;

                self.set_all_flags(
                    result == 0,
                    true,
                    (self.a & 0xf) < (value & 0xf),
                    (self.a as u16) < (value as u16),
                );

                self.a = result;
            }
            OP::SbcR16(reg) => {
                let address = self.get_reg16(reg);
                let value = self.read_byte(address);
                let result = self.a.wrapping_sub(value) - self.get_flag(registers::Flag::C) as u8;

                self.set_all_flags(
                    result == 0,
                    true,
                    (self.a & 0xf) < (value & 0xf),
                    (self.a as u16) < (value as u16),
                );

                self.a = result;
            }
            OP::AddHLR16(reg) => {
                let address = self.get_reg16(reg);
                let value = self.read_byte(address);
                let result = self
                    .get_reg16(registers::Reg16::HL)
                    .wrapping_add(value as u16);

                self.set_all_flags(
                    false,
                    false,
                    (self.get_reg16(registers::Reg16::HL) & 0xfff) + (value as u16 & 0xfff) > 0xfff,
                    (self.get_reg16(registers::Reg16::HL) as u32) + (value as u32) > 0xffff,
                );

                self.set_reg16(registers::Reg16::HL, result);
            }
            OP::DecR16(reg) => {
                let value = self.get_reg16(reg);
                self.set_reg16(reg, value.wrapping_sub(1));
            }
            OP::IncR16(reg) => {
                let value = self.get_reg16(reg);
                self.set_reg16(reg, value.wrapping_add(1));
            }
            OP::RlR8(reg) => {
                let value = self.get_reg8(reg);
                let result = (value << 1) | (self.get_flag(registers::Flag::C) as u8);

                self.set_all_flags(
                    result == 0,
                    false,
                    (value & 0x80) == 0x80,
                    (value & 0x80) == 0x80,
                );

                self.set_reg8(reg, result);
            }
            OP::RlA => {
                let value = self.a;
                let result = (value << 1) | (self.get_flag(registers::Flag::C) as u8);

                self.set_all_flags(
                    result == 0,
                    false,
                    (value & 0x80) == 0x80,
                    (value & 0x80) == 0x80,
                );

                self.a = result;
            }
            OP::RrR8(reg) => {
                let value = self.get_reg8(reg);
                let result = (value >> 1) | ((self.get_flag(registers::Flag::C) as u8) << 7);

                self.set_all_flags(
                    result == 0,
                    false,
                    (value & 0x1) == 0x1,
                    (value & 0x1) == 0x1,
                );

                self.set_reg8(reg, result);
            }
            OP::RrA => {
                let value = self.a;
                let result = (value >> 1) | ((self.get_flag(registers::Flag::C) as u8) << 7);

                self.set_all_flags(
                    result == 0,
                    false,
                    (value & 0x1) == 0x1,
                    (value & 0x1) == 0x1,
                );

                self.a = result;
            }
            OP::LdR8R8(reg, reg2) => {
                let value = self.get_reg8(reg2);
                self.set_reg8(reg, value);
            }
            OP::LdMemR8(reg, reg2) => {
                let address = self.get_reg16(registers::Reg16::HL);
                let value = self.get_reg8(reg2);
                self.write_byte(address, value);
            }
            OP::LdR8Mem(reg, reg2) => {
                let address = self.get_reg16(registers::Reg16::HL);
                let value = self.read_byte(address);
                self.set_reg8(reg, value);
            }
            OP::LdR8Imm(reg, value) => {
                self.set_reg8(reg, value);
            }
            OP::LdR16Imm(reg, value) => {
                self.set_reg16(reg, value);
            }
            OP::LdHLImm(value) => {
                self.set_reg16(registers::Reg16::HL, value as u16);
            }
            OP::LdR16R8(reg1, reg2) => {
                let address = self.get_reg16(reg1);
                let value = self.get_reg8(reg2);
                self.write_byte(address, value);
            }
            OP::LdHLIA => {
                let address = self.get_reg16(registers::Reg16::HL);
                let value = self.a;
                self.write_byte(address, value);
                self.set_reg16(
                    registers::Reg16::HL,
                    self.get_reg16(registers::Reg16::HL).wrapping_add(1),
                );
            }
            OP::LdHLDA => {
                let address = self.get_reg16(registers::Reg16::HL);
                let value = self.a;
                self.write_byte(address, value);
                self.set_reg16(
                    registers::Reg16::HL,
                    self.get_reg16(registers::Reg16::HL).wrapping_sub(1),
                );
            }
            OP::LdAHLI => {
                let address = self.get_reg16(registers::Reg16::HL);
                let value = self.read_byte(address);
                self.a = value;
                self.set_reg16(
                    registers::Reg16::HL,
                    self.get_reg16(registers::Reg16::HL).wrapping_add(1),
                );
            }
            OP::LdAHLD => {
                let address = self.get_reg16(registers::Reg16::HL);
                let value = self.read_byte(address);
                self.a = value;
                self.set_reg16(
                    registers::Reg16::HL,
                    self.get_reg16(registers::Reg16::HL).wrapping_sub(1),
                );
            }
            OP::Jr(value) => {
                self.pc = self.pc.wrapping_add(value as u16);
            }
            OP::JrCond(flag, value) => {
                if self.get_flag(flag) {
                    self.pc = self.pc.wrapping_add(value as u16);
                }
            }
            OP::RetCond(flag) => {
                if self.get_flag(flag) {
                    let address = self.pop_stack();
                    self.pc = address;
                }
            }
            OP::Ret => {
                let address = self.pop_stack();
                self.pc = address;
            }
            OP::Reti => {
                let address = self.pop_stack();
                self.pc = address;
                self.ime = true;
            }
            OP::Rst(value) => {
                self.push_stack(self.pc);
                self.pc = value as u16;
            }
            OP::LdImmSP(value) => {
                self.sp = value;
            }
            OP::LdSPHL => {
                self.sp = self.get_reg16(registers::Reg16::HL);
            }
            OP::PopR16(reg) => {
                let value = self.pop_stack();
                self.set_reg16(reg, value);
            }
            OP::PushR16(reg) => {
                let value = self.get_reg16(reg);
                self.push_stack(value);
            }
            OP::Ccf => {
                self.set_all_flags(false, false, false, !self.get_flag(registers::Flag::C));
            }
            OP::Cpl => {
                self.a = !self.a;
                self.set_all_flags(true, true, true, self.get_flag(registers::Flag::C));
            }
            OP::Daa => {
                let mut value = self.a;
                if self.get_flag(registers::Flag::N) {
                    if self.get_flag(registers::Flag::H) {
                        value = value.wrapping_sub(0x06);
                    }
                    if self.get_flag(registers::Flag::C) {
                        value = value.wrapping_sub(0x60);
                    }
                } else {
                    if self.get_flag(registers::Flag::H) || (value & 0xF) > 9 {
                        value = value.wrapping_add(0x06);
                    }
                    if self.get_flag(registers::Flag::C) || value > 0x9F {
                        value = value.wrapping_add(0x60);
                    }
                }

                self.set_all_flags(
                    value == 0,
                    self.get_flag(registers::Flag::N),
                    false,
                    value > 0xFF,
                );

                self.a = value;
            }
            OP::Di => {
                self.ime = false;
            }
            OP::Ei => {
                self.ime = true;
            }
            OP::Halt => {
                self.halted = true;
            }
            OP::Nop => {}
            OP::Scf => {
                self.set_all_flags(false, false, false, true);
            }
            OP::Stop => {
                self.stopped = true;
            }
            OP::AndR8(reg) => {
                let value = self.get_reg8(reg);
                self.a &= value;
                self.set_all_flags(self.a == 0, false, true, false);
            }
            OP::AndR16(reg) => {
                let value = self.get_reg16(reg);
                self.a &= (value >> 8) as u8;
                self.set_all_flags(self.a == 0, false, true, false);
            }
            OP::XorR8(reg) => {
                let value = self.get_reg8(reg);
                self.a ^= value;
                self.set_all_flags(self.a == 0, false, false, false);
            }
            OP::XorR16(reg) => {
                let value = self.get_reg16(reg);
                self.a ^= (value >> 8) as u8;
                self.set_all_flags(self.a == 0, false, false, false);
            }
            OP::OrR8(reg) => {
                let value = self.get_reg8(reg);
                self.a |= value;
                self.set_all_flags(self.a == 0, false, false, false);
            }
            OP::OrR16(reg) => {
                let value = self.get_reg16(reg);
                self.a |= (value >> 8) as u8;
                self.set_all_flags(self.a == 0, false, false, false);
            }
            OP::CpR8(reg) => {
                let value = self.get_reg8(reg);
                self.set_all_flags(self.a == value, true, self.a < value, false);
            }
            OP::CpR16(reg) => {
                let value = self.get_reg16(reg);
                self.set_all_flags(
                    self.a == (value >> 8) as u8,
                    true,
                    self.a < (value >> 8) as u8,
                    false,
                );
            }
            OP::JPCondImm16(flag) => {
                let value = self.read_imm16();
                if self.get_flag(flag) {
                    self.pc = value;
                }
            }
            OP::CallCondImm16(flag) => {
                let value = self.read_imm16();
                if self.get_flag(flag) {
                    self.push_stack(self.pc);
                    self.pc = value;
                }
            }
            OP::JPImm16 => {
                let value = self.read_imm16();
                self.pc = value;
            }
            OP::AddImm8 => {
                let value = self.read_imm8();
                let (result, overflow) = self.a.overflowing_add(value);
                self.set_all_flags(result == 0, false, false, overflow);
                self.a = result;
            }
            OP::CBPrefix => {
                let opcode = self.read_imm8();
                self.execute_cb(opcode);
            }
            OP::CallImm16 => {
                let value = self.read_imm16();
                self.push_stack(self.pc);
                self.pc = value;
            }
            OP::AdcImm8 => {
                let value = self.read_imm8();
                let carry = if self.get_flag(registers::Flag::C) {
                    1
                } else {
                    0
                };
                let (result, overflow) = self.a.overflowing_add(value);
                let (result, overflow2) = result.overflowing_add(carry);
                self.set_all_flags(result == 0, false, false, overflow || overflow2);
                self.a = result;
            }
            OP::SubImm8 => {
                let value = self.read_imm8();
                let (result, overflow) = self.a.overflowing_sub(value);
                self.set_all_flags(result == 0, true, overflow, false);
                self.a = result;
            }
            OP::SbcImm8 => {
                let value = self.read_imm8();
                let carry = if self.get_flag(registers::Flag::C) {
                    1
                } else {
                    0
                };
                let (result, overflow) = self.a.overflowing_sub(value);
                let (result, overflow2) = result.overflowing_sub(carry);
                self.set_all_flags(result == 0, true, overflow || overflow2, false);
                self.a = result;
            }
            OP::LdIOImm8A => {
                let value = self.read_imm8();
                self.write_io(value as u16, self.a);
            }
            OP::LdIOC => {
                let value = self.read_io(self.c as u16);
                self.a = value;
            }
            OP::AndImm8 => {
                let value = self.read_imm8();
                self.a &= value;
                self.set_all_flags(self.a == 0, false, true, false);
            }
            OP::AddSPImm8 => {
                let value = self.read_imm8();
                let (result, overflow) = self.sp.overflowing_add(value as u16);
                self.set_all_flags(false, false, overflow, overflow);
                self.sp = result;
            }
            OP::LdImm16A => {
                let value = self.read_imm16();
                self.write_mem(value, self.a);
            }
            OP::JPHL => {
                let value = self.get_reg16(registers::Reg16::HL);
                self.pc = value;
            }
            OP::XorImm8 => {
                let value = self.read_imm8();
                self.a ^= value;
                self.set_all_flags(self.a == 0, false, false, false);
            }
            OP::LdAIOImm8 => {
                let value = self.read_imm8();
                self.a = self.read_io(value as u16);
            }
            OP::LdACIO => {
                let value = self.read_io(self.c as u16);
                self.a = value;
            }
            OP::OrImm8 => {
                let value = self.read_imm8();
                self.a |= value;
                self.set_all_flags(self.a == 0, false, false, false);
            }
            OP::LdHLSPImm8 => {
                let value = self.read_imm8();
                let (result, overflow) = self.sp.overflowing_add(value as u16);
                self.set_all_flags(false, false, overflow, overflow);
                self.set_reg16(registers::Reg16::HL, result);
            }
            OP::LdAImm16 => {
                let value = self.read_imm16();
                self.a = self.read_mem(value);
            }
            OP::CpImm8 => {
                let value = self.read_imm8();
                let (result, overflow) = self.a.overflowing_sub(value);
                self.set_all_flags(result == 0, true, overflow, false);
            }
            _ => {
                panic!("Unknown opcode {:?}", opcode);
            }
        }

        self.pc += size as u16;
    }

    fn get_reg8(&self, reg1: registers::Reg8) -> u8 {
        match reg1 {
            registers::Reg8::A => self.a,
            registers::Reg8::B => self.b,
            registers::Reg8::C => self.c,
            registers::Reg8::D => self.d,
            registers::Reg8::E => self.e,
            registers::Reg8::H => self.h,
            registers::Reg8::L => self.l,
        }
    }

    fn get_reg16(&self, reg1: registers::Reg16) -> u16 {
        match reg1 {
            registers::Reg16::AF => {
                ((self.a as u16) << 8)
                    | (self.f.z as u16) << 7
                    | (self.f.n as u16) << 6
                    | (self.f.h as u16) << 5
                    | (self.f.c as u16) << 4
            }
            registers::Reg16::BC => ((self.b as u16) << 8) | (self.c as u16),
            registers::Reg16::DE => ((self.d as u16) << 8) | (self.e as u16),
            registers::Reg16::HL => ((self.h as u16) << 8) | (self.l as u16),
            registers::Reg16::SP => self.sp,
            registers::Reg16::PC => self.pc,
        }
    }

    fn set_reg8(&mut self, reg1: registers::Reg8, value: u8) -> () {
        match reg1 {
            registers::Reg8::A => self.a = value,
            registers::Reg8::B => self.b = value,
            registers::Reg8::C => self.c = value,
            registers::Reg8::D => self.d = value,
            registers::Reg8::E => self.e = value,
            registers::Reg8::H => self.h = value,
            registers::Reg8::L => self.l = value,
        }
    }

    fn set_reg16(&mut self, reg1: registers::Reg16, value: u16) -> () {
        match reg1 {
            registers::Reg16::AF => panic!("Cannot set AF register"),
            registers::Reg16::BC => {
                self.b = ((value & 0xff00) >> 8) as u8;
                self.c = (value & 0x00ff) as u8;
            }
            registers::Reg16::DE => {
                self.d = ((value & 0xff00) >> 8) as u8;
                self.e = (value & 0x00ff) as u8;
            }
            registers::Reg16::HL => {
                self.h = ((value & 0xff00) >> 8) as u8;
                self.l = (value & 0x00ff) as u8;
            }
            registers::Reg16::SP => self.sp = value,
            registers::Reg16::PC => self.pc = value,
        }
    }

    fn get_flag(&self, flag: registers::Flag) -> bool {
        match flag {
            registers::Flag::Z => self.f.z,
            registers::Flag::N => self.f.n,
            registers::Flag::H => self.f.h,
            registers::Flag::C => self.f.c,
            registers::Flag::NZ => !self.f.z,
            registers::Flag::NC => !self.f.c,
        }
    }

    fn set_flag(&mut self, flag: registers::Flag, value: bool) -> () {
        match flag {
            registers::Flag::Z => self.f.z = value,
            registers::Flag::N => self.f.n = value,
            registers::Flag::H => self.f.h = value,
            registers::Flag::C => self.f.c = value,
            registers::Flag::NZ => self.f.z = !value,
            registers::Flag::NC => self.f.c = !value,
        }
    }

    fn set_all_flags(&mut self, z: bool, n: bool, h: bool, c: bool) -> () {
        self.set_flag(registers::Flag::Z, z);
        self.set_flag(registers::Flag::N, n);
        self.set_flag(registers::Flag::H, h);
        self.set_flag(registers::Flag::C, c);
    }

    pub fn load_rom(&mut self, path: &str) -> () {
        let mut file = File::open(path).expect("Failed to open file");
        let mut buffer = Vec::new();

        file.read_to_end(&mut buffer).expect("Failed to read file");

        for (i, byte) in buffer.iter().enumerate() {
            self.memory[i] = *byte;
        }
    }

    fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }

    fn pop_stack(&mut self) -> u16 {
        let value = self.read_byte(self.sp);
        self.set_reg16(registers::Reg16::SP, self.sp + 1);
        value as u16
    }

    fn push_stack(&mut self, value: u16) {
        self.set_reg16(registers::Reg16::SP, self.sp - 1);
        self.write_byte(self.sp, (value >> 8) as u8);
        self.set_reg16(registers::Reg16::SP, self.sp - 1);
        self.write_byte(self.sp, (value & 0x00ff) as u8);
    }

    fn read_imm16(&mut self) -> u16 {
        // read little endian
        let value = (self.read_byte(self.pc + 1) as u16) << 8 | self.read_byte(self.pc) as u16;
        self.set_reg16(registers::Reg16::PC, self.pc + 2);
        println!("read_imm16: {:x}", value);
        value
    }

    fn read_imm8(&mut self) -> u8 {
        let value = self.read_byte(self.pc);
        self.set_reg16(registers::Reg16::PC, self.pc + 1);
        value
    }

    fn write_io(&self, value: u16, a: u8) -> i32 {
        match a {
            _ => panic!("Unknown IO address: {:x}", a),
        }
    }

    fn read_io(&self, c: u16) -> u8 {
        0
    }

    fn write_mem(&self, value: u16, a: u8) -> () {}

    fn read_mem(&self, value: u16) -> u8 {
        0
    }

    fn execute_cb(&self, opcode: u8) -> () {
        match opcode {
            _ => panic!("Unknown CB opcode: {:x}", opcode),
        }
    }
}
