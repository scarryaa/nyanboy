pub struct Memory {
    pub memory: [u8; 0xFFFF],
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            memory: [0; 0xFFFF],
        }
    }
}
