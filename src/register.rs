#[derive(Debug)]
pub struct StatusRegister {
    pub c: bool, // Carry Flag
    pub z: bool, // Zero Flag
    pub i: bool, // Interrupt Disable Flag
    pub d: bool, // Decimal Mode Flag
    pub b: bool, // Break Command Flag
    pub v: bool, // Overflow Flag
    pub n: bool, // Negative Flag
}

impl StatusRegister {
    pub fn new() -> Self {
        StatusRegister {
            c: false,
            z: false,
            i: false,
            d: false,
            b: false,
            v: false,
            n: false,
        }
    }

    pub fn as_byte(&self) -> u8 {
        let mut result = 0u8;
        if self.c {
            result |= 0b0000_0001;
        }
        if self.z {
            result |= 0b0000_0010;
        }
        if self.i {
            result |= 0b0000_0100;
        }
        if self.d {
            result |= 0b0000_1000;
        }
        if self.b {
            result |= 0b0001_0000;
        }
        // bit 5は常に1
        result |= 0b0010_0000;
        if self.v {
            result |= 0b0100_0000;
        }
        if self.n {
            result |= 0b1000_0000;
        }
        result
    }

    pub fn from_byte(byte: u8) -> Self {
        StatusRegister {
            c: (byte & 0b0000_0001) != 0,
            z: (byte & 0b0000_0010) != 0,
            i: (byte & 0b0000_0100) != 0,
            d: (byte & 0b0000_1000) != 0,
            b: (byte & 0b0001_0000) != 0,
            // bit 5は無視
            v: (byte & 0b0100_0000) != 0,
            n: (byte & 0b1000_0000) != 0,
        }
    }
}
