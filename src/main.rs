use crate::cpu::Cpu;

mod cpu;
mod opcodes;
mod registers;

fn main() {
    let mut cpu = Cpu::new();
    cpu.execute();
}
