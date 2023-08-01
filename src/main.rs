use crate::cpu::Cpu;

mod cpu;
mod opcodes;
mod registers;

fn main() {
    let mut cpu = Cpu::new();

    cpu.load_rom("roms/cpu_instrs.gb");

    loop {
        cpu.execute();
    }
}
