use crate::cpu::Cpu;

mod cpu;
mod opcodes;
mod registers;

fn main() {
    let mut cpu = Cpu::new();

    cpu.load_rom("roms/04-op r,imm.gb");
    println!("Loaded ROM");

    loop {
        cpu.execute();
    }
}
