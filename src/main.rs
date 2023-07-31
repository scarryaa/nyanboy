use crate::cpu::Cpu;
use crate::memory::Memory;

mod cpu;
mod memory;
mod opcodes;
mod registers;

fn main() {
    let mem = memory::Memory::new();
}
