pub struct Memory {
    data: [u8; 0x10000], // 65,536 bytes of memory
}

impl Memory {
    pub fn new() -> Self {
        Memory { data: [0; 0x10000] }
    }

    pub fn read(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    pub fn write(&mut self, address: u16, data: u8) {
        self.data[address as usize] = data;
    }

    pub fn load(&mut self, start: u16, data: &[u8]) {
        for (i, &byte) in data.iter().enumerate() {
            self.data[start as usize + i] = byte;
        }
    }
}