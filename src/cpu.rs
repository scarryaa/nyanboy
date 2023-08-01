use crate::opcodes::OP;
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
        }
    }

    pub fn execute(&mut self) {
        let (opcode, size, duration) =
            OP::from_bytes(&self.memory[self.pc as usize..]).expect("Unknown opcode");

        println!("{:?}", opcode);
        println!("Size: {}", size);
        println!("Duration: {}", duration);
        println!("PC: {}", self.pc);

        match opcode {
            OP::Adc(reg1, reg2) => {
                let value = self.get_reg8(reg1) + self.get_reg8(reg2);
                let carry = (self.get_reg8(reg1) as u16) + (self.get_reg8(reg2) as u16) > 0xFF;
                let half_carry = (self.get_reg8(reg1) & 0xF) + (self.get_reg8(reg2) & 0xF) > 0xF;

                self.set_reg8(reg1, value);
                self.set_all_flags(value == 0, false, half_carry, carry);
            }
            OP::AdcImm(reg, imm) => {
                let value = self.get_reg8(reg) + imm;
                let carry = (self.get_reg8(reg) as u16) + (imm as u16) > 0xFF;
                let half_carry = (self.get_reg8(reg) & 0xF) + (imm & 0xF) > 0xF;

                self.set_reg8(reg, value);
                self.set_all_flags(value == 0, false, half_carry, carry);
            }
            OP::Add(reg1, reg2) => {
                let value = self.get_reg8(reg1) + self.get_reg8(reg2);
                self.set_reg8(reg1, value);
            }
            OP::AddImm(reg, imm) => {
                let value = self.get_reg8(reg) + imm;
                self.set_reg8(reg, value);
            }
            OP::Inc(reg) => {
                let value = self.get_reg8(reg) + 1;
                self.set_reg8(reg, value);
            }
            _ => {
                panic!("Unknown opcode {:?}", opcode);
            }
        }

        self.pc += size as u16;
    }

    fn get_reg8(&self, reg1: crate::registers::Reg8) -> u8 {
        match reg1 {
            crate::registers::Reg8::A => self.a,
            crate::registers::Reg8::B => self.b,
            crate::registers::Reg8::C => self.c,
            crate::registers::Reg8::D => self.d,
            crate::registers::Reg8::E => self.e,
            crate::registers::Reg8::H => self.h,
            crate::registers::Reg8::L => self.l,
        }
    }

    fn set_reg8(&mut self, reg1: crate::registers::Reg8, value: u8) -> () {
        match reg1 {
            crate::registers::Reg8::A => self.a = value,
            crate::registers::Reg8::B => self.b = value,
            crate::registers::Reg8::C => self.c = value,
            crate::registers::Reg8::D => self.d = value,
            crate::registers::Reg8::E => self.e = value,
            crate::registers::Reg8::H => self.h = value,
            crate::registers::Reg8::L => self.l = value,
        }
    }

    fn set_flag(&mut self, flag: crate::registers::Flag, value: bool) -> () {
        match flag {
            crate::registers::Flag::Z => self.f.z = value,
            crate::registers::Flag::N => self.f.n = value,
            crate::registers::Flag::H => self.f.h = value,
            crate::registers::Flag::C => self.f.c = value,
            crate::registers::Flag::NZ => self.f.z = !value,
            crate::registers::Flag::NC => self.f.c = !value,
        }
    }

    fn set_all_flags(&mut self, z: bool, n: bool, h: bool, c: bool) -> () {
        self.set_flag(crate::registers::Flag::Z, z);
        self.set_flag(crate::registers::Flag::N, n);
        self.set_flag(crate::registers::Flag::H, h);
        self.set_flag(crate::registers::Flag::C, c);
    }

    pub fn load_rom(&mut self, path: &str) -> () {
        let mut file = File::open(path).expect("Failed to open file");
        let mut buffer = Vec::new();

        file.read_to_end(&mut buffer).expect("Failed to read file");

        for (i, byte) in buffer.iter().enumerate() {
            self.memory[i] = *byte;
        }
    }
}
