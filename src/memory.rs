pub struct MemoryBus {
    memory: [u8; 0xFFFF],
}

impl MemoryBus {
    pub fn new() -> MemoryBus {
        MemoryBus {
            memory: [0; 0xFFFF],
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn read_next_byte(&self, address: &mut u16) -> u8 {
        let value = self.read_byte(*address);
        *address += 1;
        value
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }
}
