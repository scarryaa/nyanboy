use crate::opcodes::OP;

pub struct Cpu {
    a: u8,
    f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
    memory: [u8; 65536],
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            a: 0,
            f: 0,
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

    pub fn get_a(&self) -> u8 {
        self.a
    }

    pub fn set_a(&mut self, value: u8) {
        self.a = value;
    }

    pub fn execute(&mut self) {
        let (opcode, size, duration) =
            OP::from_bytes(&self.memory[self.pc as usize..]).expect("Unknown opcode");

        match opcode {
            OP::Nop => {}
            _ => {
                panic!("Unknown opcode: 0x")
            }
        }
    }
}
