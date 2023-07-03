mod cpu;
mod instructions;
mod memory;
mod registers;

fn main() {
    let mut cpu = cpu::CPU::new();
    cpu::CPU::step(&mut cpu);
}
