use crate::instructions::*;
use crate::memory::*;
use crate::registers::*;

pub struct CPU {
    registers: Registers,
    pc: u16,
    sp: u16,
    pub bus: MemoryBus,
    is_halted: bool,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: Registers::new(),
            pc: 0,
            sp: 0,
            bus: MemoryBus::new(),
            is_halted: false,
        }
    }

    pub fn execute(&mut self, instruction: Instruction) -> u16 {
        if self.is_halted {
            return self.pc;
        }

        match instruction {
            Instruction::ADD(target) => match target {
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.add(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                _ => unimplemented!(),
            },
            Instruction::JP(test) => {
                let jump_condition = match test {
                    JumpTest::NotZero => !self.registers.f.zero,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::Always => true,
                };
                self.jump(jump_condition);
                self.pc.wrapping_add(1)
            }
            Instruction::LD(load_type) => {
                match load_type {
                    LoadType::Byte(target, source) => {
                        let source_value = match source {
                            LoadByteSource::A => self.registers.a,
                            LoadByteSource::D8 => self.bus.read_next_byte(&mut self.pc),
                            LoadByteSource::HLI => self.bus.read_byte(self.registers.get_hl()),
                            _ => {
                                unimplemented!()
                            } // TODO - Implement other sources
                        };
                        match target {
                            LoadByteTarget::A => self.registers.a = source_value,
                            LoadByteTarget::B => self.registers.b = source_value,
                            LoadByteTarget::C => self.registers.c = source_value,
                            LoadByteTarget::D => self.registers.d = source_value,
                            LoadByteTarget::E => self.registers.e = source_value,
                            LoadByteTarget::H => self.registers.h = source_value,
                            LoadByteTarget::L => self.registers.l = source_value,
                            LoadByteTarget::HLI => {
                                self.bus.write_byte(self.registers.get_hl(), source_value)
                            }
                            _ => {
                                unimplemented!()
                            }
                        };
                        match source {
                            LoadByteSource::D8 => self.pc.wrapping_add(2),
                            _ => self.pc.wrapping_add(1),
                        }
                    }
                    _ => unimplemented!(),
                }
            }
            Instruction::PUSH(target) => {
                let value = match target {
                    PushTarget::BC => self.registers.get_bc(),
                    PushTarget::DE => self.registers.get_de(),
                    PushTarget::HL => self.registers.get_hl(),
                    PushTarget::AF => self.registers.get_af(),
                    PushTarget::SP => self.sp,
                };
                self.push(value);
                self.pc.wrapping_add(1)
            }
            Instruction::POP(target) => {
                let result = self.pop();
                match target {
                    PopTarget::BC => self.registers.set_bc(result),
                    PopTarget::DE => self.registers.set_de(result),
                    PopTarget::HL => self.registers.set_hl(result),
                    PopTarget::AF => self.registers.set_af(result),
                    PopTarget::SP => self.sp = result,
                };
                self.pc.wrapping_add(1)
            }
            Instruction::CALL(test) => {
                let jump_condition = match test {
                    JumpTest::NotZero => !self.registers.f.zero,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::Always => true,
                };
                self.call(jump_condition)
            }
            Instruction::RET(test) => {
                let jump_condition = match test {
                    JumpTest::NotZero => !self.registers.f.zero,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::Always => true,
                };
                self.ret(jump_condition)
            }
            Instruction::NOP() => self.pc.wrapping_add(1),
            // TODO - Add more instructions
            _ => self.pc,
        }
    }

    fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        new_value
    }

    pub fn step(&mut self) {
        let mut instruction_byte = self.bus.read_byte(self.pc);
        let prefixed = instruction_byte == 0xCB;
        if prefixed {
            instruction_byte = self.bus.read_byte(self.pc.wrapping_add(1));
        }

        let next_pc = if let Some(instruction) = Instruction::from_byte(instruction_byte, prefixed)
        {
            self.execute(instruction)
        } else {
            let description = format!(
                "0x{}{:x}",
                if prefixed { "cb" } else { "" },
                instruction_byte
            );
            panic!("Unkown instruction found for: {}", description);
        };

        self.pc = next_pc;
    }

    fn jump(&self, should_jump: bool) -> u16 {
        if should_jump {
            let low = self.bus.read_byte(self.pc.wrapping_add(1));
            let high = self.bus.read_byte(self.pc.wrapping_add(2));
            ((high as u16) << 8) | (low as u16)
        } else {
            self.pc.wrapping_add(3)
        }
    }

    fn push(&mut self, value: u16) {
        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, ((value & 0xFF00) >> 8) as u8);

        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, (value & 0xFF) as u8);
    }

    fn pop(&mut self) -> u16 {
        let lsb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        let msb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        (msb << 8) | lsb
    }

    fn call(&mut self, should_jump: bool) -> u16 {
        if should_jump {
            let low = self.bus.read_byte(self.pc.wrapping_add(1));
            let high = self.bus.read_byte(self.pc.wrapping_add(2));
            self.push(self.pc.wrapping_add(3));
            ((high as u16) << 8) | (low as u16)
        } else {
            self.pc.wrapping_add(3)
        }
    }

    fn ret(&mut self, should_jump: bool) -> u16 {
        if should_jump {
            self.pop()
        } else {
            self.pc.wrapping_add(1)
        }
    }
}
