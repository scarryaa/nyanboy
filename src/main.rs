use crate::registers::Registers;
mod registers;

fn main() {
    // print all the registers
    let registers = Registers::new();
    println!("AF: {:04X}", registers.get_af());
    println!("BC: {:04X}", registers.get_bc());
    println!("DE: {:04X}", registers.get_de());
    println!("HL: {:04X}", registers.get_hl());
    println!("SP: {:04X}", registers.sp);
    println!("PC: {:04X}", registers.pc);
}
