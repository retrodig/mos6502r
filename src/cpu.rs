use crate::memory::Memory;
use crate::register::StatusRegister;

#[derive(Debug)]
pub struct CPU {
    a: u8,                  // A Register
    x: u8,                  // X Register
    y: u8,                  // Y Register
    pc: u16,                // Program Counter
    sp: u8,                 // Stack Pointer
    status: StatusRegister, // Status Register
    halted: bool,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            sp: 0xFF, // Initial value of stack pointer is 0xFF
            status: StatusRegister::new(),
            halted: false,
        }
    }

    pub fn step(&mut self, memory: &mut Memory) {
        if !self.halted {
            self.execute(memory);
        }
    }

    pub fn run(&mut self, memory: &mut Memory) {
        while !self.halted {
            self.execute(memory);
        }
    }

    pub fn set_pc(&mut self, addr: u16) { self.pc = addr; }
    pub fn a(&self) -> u8 { self.a }
    pub fn x(&self) -> u8 { self.x }
    pub fn y(&self) -> u8 { self.y }
    pub fn pc(&self) -> u16 { self.pc }
    pub fn sp(&self) -> u8 { self.sp }
    pub fn is_halted(&self) -> bool { self.halted }
    pub fn carry(&self) -> bool { self.status.c }
    pub fn zero(&self) -> bool { self.status.z }
    pub fn negative(&self) -> bool { self.status.n }
    pub fn overflow(&self) -> bool { self.status.v }

    fn execute(&mut self, memory: &mut Memory) {
        let opcode = memory.read(self.pc);
        self.pc += 1;

        match opcode {
            0x00 => {
                // BRK Instruction
                // Push PC+1 and status, set I, jump to IRQ/BRK vector ($FFFE/$FFFF)
                let return_addr = self.pc; // PC already points to next byte after opcode fetch
                memory.write(0x0100 + self.sp as u16, ((return_addr >> 8) & 0xFF) as u8);
                self.sp = self.sp.wrapping_sub(1);
                memory.write(0x0100 + self.sp as u16, (return_addr & 0xFF) as u8);
                self.sp = self.sp.wrapping_sub(1);
                let mut p = self.status.as_byte();
                p |= 0b0011_0000; // Set B and unused bit5 on stack copy
                memory.write(0x0100 + self.sp as u16, p);
                self.sp = self.sp.wrapping_sub(1);
                self.status.i = true;
                let lo = memory.read(0xFFFE) as u16;
                let hi = memory.read(0xFFFF) as u16;
                self.pc = (hi << 8) | lo;
                self.halted = true;
            }
            0x01 => {
                // ORA ($NN,X)
                let addr = self.indirect_x_addr(memory);
                let data = memory.read(addr);
                self.ora(data);
            }
            0x05 => {
                // ORA $NN (Zero Page)
                let addr = self.zero_page_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                self.ora(data);
            }
            0x06 => {
                // ASL $NN (Zero Page)
                let addr = self.zero_page_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                let result = self.asl(data);
                memory.write(addr, result);
            }
            0x08 => {
                // PHP (Push Processor Status)
                memory.write(0x0100 + self.sp as u16, self.status.as_byte());
                self.sp = self.sp.wrapping_sub(1);
            }
            0x09 => {
                // ORA #$NN
                let immediate = memory.read(self.pc);
                self.pc += 1;
                self.ora(immediate);
            }
            0x0A => {
                // ASL A (Accumulator)
                let result = self.asl(self.a);
                self.a = result;
            }
            0x0D => {
                // ORA $NNNN (Absolute)
                let addr = self.absolute_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                self.ora(data);
            }
            0x0E => {
                // ASL $NNNN (Absolute)
                let addr = self.absolute_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                let result = self.asl(data);
                memory.write(addr, result);
            }
            0x10 => {
                // BPL (Branch on Plus)
                let offset = memory.read(self.pc) as i8;
                self.pc += 1;
                if !self.status.n {
                    self.pc = self.pc.wrapping_add(offset as u16);
                }
            }
            0x11 => {
                // ORA ($NN),Y
                let addr = self.indirect_y_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                self.ora(data);
            }
            0x15 => {
                // ORA $NN,X (Zero Page,X)
                let addr = self.zero_page_x_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                self.ora(data);
            }
            0x16 => {
                // ASL $NN,X (Zero Page,X)
                let addr = self.zero_page_x_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                let result = self.asl(data);
                memory.write(addr, result);
            }
            0x18 => {
                // CLC (Clear Carry)
                self.status.c = false;
            }
            0x19 => {
                // ORA $NNNN,Y (Absolute,Y)
                let addr = self.absolute_y_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                self.ora(data);
            }
            0x1D => {
                // ORA $NNNN,X (Absolute,X)
                let addr = self.absolute_x_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                self.ora(data);
            }
            0x1E => {
                // ASL $NNNN,X (Absolute,X)
                let addr = self.absolute_x_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                let result = self.asl(data);
                memory.write(addr, result);
            }
            0x20 => {
                // JSR $NNNN
                let target = self.absolute_addr(memory);
                let return_addr = self.pc + 1; // Address of last operand byte
                memory.write(0x0100 + self.sp as u16, ((return_addr >> 8) & 0xFF) as u8);
                self.sp = self.sp.wrapping_sub(1);
                memory.write(0x0100 + self.sp as u16, (return_addr & 0xFF) as u8);
                self.sp = self.sp.wrapping_sub(1);
                self.pc = target;
            }
            0x21 => {
                // AND ($NN,X)
                let addr = self.indirect_x_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                self.and(data);
            }
            0x24 => {
                // BIT $NN (Zero Page)
                let addr = self.zero_page_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                self.bit(data);
            }
            0x25 => {
                // AND $NN (Zero Page)
                let addr = self.zero_page_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                self.and(data);
            }
            0x26 => {
                // ROL $NN (Zero Page)
                let addr = self.zero_page_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                let result = self.rol(data);
                memory.write(addr, result);
            }
            0x28 => {
                // PLP (Pull Processor Status)
                self.sp = self.sp.wrapping_add(1);
                let status = memory.read(0x0100 + self.sp as u16);
                // ステータスレジスタを復元する処理を追加する必要があります
                self.status = StatusRegister::from_byte(status);
            }
            0x29 => {
                // AND #$NN
                let immediate = memory.read(self.pc);
                self.pc += 1;
                self.and(immediate);
            }
            0x2A => {
                // ROL A
                let result = self.rol(self.a);
                self.a = result;
            }
            0x2C => {
                // BIT $NNNN
                let addr = self.absolute_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                self.bit(data);
            }
            0x2D => {
                // AND $NNNN
                let addr = self.absolute_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                self.and(data);
            }
            0x2E => {
                // ROL $NNNN (Absolute)
                let addr = self.absolute_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                let result = self.rol(data);
                memory.write(addr, result);
            }
            0x30 => {
                // BMI (Branch on Minus)
                let offset = memory.read(self.pc) as i8;
                self.pc += 1;
                if self.status.n {
                    self.pc = self.pc.wrapping_add(offset as u16);
                }
            }
            0x31 => {
                // AND ($NN),Y
                let addr = self.indirect_y_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                self.and(data);
            }
            0x35 => {
                // AND $NN,X
                let addr = self.zero_page_x_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                self.and(data);
            }
            0x36 => {
                // ROL $NN,X
                let addr = self.zero_page_x_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                let result = self.rol(data);
                memory.write(addr, result);
            }
            0x38 => {
                // SEC (Set Carry Flag)
                self.status.c = true;
            }
            0x39 => {
                // AND $NNNN,Y
                let addr = self.absolute_y_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                self.and(data);
            }
            0x3D => {
                // AND $NNNN,X
                let addr = self.absolute_x_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                self.and(data);
            }
            0x3E => {
                // ROL $NNNN,X
                let addr = self.absolute_x_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                let result = self.rol(data);
                memory.write(addr, result);
            }
            0x40 => {
                // RTI (Return from Interrupt)
                self.sp = self.sp.wrapping_add(1);
                let status = memory.read(0x0100 + self.sp as u16);
                self.status = StatusRegister::from_byte(status);
                self.sp = self.sp.wrapping_add(1);
                let lo = memory.read(0x0100 + self.sp as u16) as u16;
                self.sp = self.sp.wrapping_add(1);
                let hi = memory.read(0x0100 + self.sp as u16) as u16;
                self.pc = (hi << 8) | lo;
            }
            0x41 => {
                // EOR ($NN,X)
                let addr = self.indirect_x_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                self.eor(data);
            }
            0x45 => {
                // EOR $NN (Zero Page)
                let addr = self.zero_page_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                self.eor(data);
            }
            0x46 => {
                // LSR $NN (Zero Page)
                let addr = self.zero_page_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                let result = self.lsr(data);
                memory.write(addr, result);
            }
            0x48 => {
                // PHA (Push Accumulator)
                memory.write(0x0100 + self.sp as u16, self.a);
                self.sp = self.sp.wrapping_sub(1);
            }
            0x49 => {
                // EOR #$NN
                let immediate = memory.read(self.pc);
                self.pc += 1;
                self.eor(immediate);
            }
            0x4A => {
                // LSR A (Accumulator)
                let result = self.lsr(self.a);
                self.a = result;
            }
            0x4C => {
                // JMP $NNNN
                let addr = self.absolute_addr(memory);
                self.pc = addr;
            }
            0x4D => {
                // EOR $NNNN
                let addr = self.absolute_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                self.eor(data);
            }
            0x50 => {
                // BVC (Branch on Overflow Clear)
                let offset = memory.read(self.pc) as i8;
                self.pc += 1;
                if !self.status.v {
                    self.pc = self.pc.wrapping_add(offset as u16);
                }
            }
            0x51 => {
                // EOR ($NN),Y
                let addr = self.indirect_y_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                self.eor(data);
            }
            0x55 => {
                // EOR $NN,X (Zero Page,X)
                let addr = self.zero_page_x_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                self.eor(data);
            }
            0x56 => {
                // LSR $NN,X (Zero Page,X)
                let addr = self.zero_page_x_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                let result = self.lsr(data);
                memory.write(addr, result);
            }
            0x58 => {
                // CLI (Clear Interrupt Disable)
                self.status.i = false;
            }
            0x59 => {
                // EOR $NNNN,Y
                let addr = self.absolute_y_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                self.eor(data);
            }
            0x5D => {
                // EOR $NNNN,X
                let addr = self.absolute_x_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                self.eor(data);
            }
            0x5E => {
                // LSR $NNNN,X
                let addr = self.absolute_x_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                let result = self.lsr(data);
                memory.write(addr, result);
            }
            0x60 => {
                // RTS (Return from Subroutine)
                self.sp = self.sp.wrapping_add(1);
                let lo = memory.read(0x0100 + self.sp as u16) as u16;
                self.sp = self.sp.wrapping_add(1);
                let hi = memory.read(0x0100 + self.sp as u16) as u16;
                self.pc = ((hi << 8) | lo).wrapping_add(1);
            }
            0x61 => {
                // ADC ($NN,X)
                let addr = self.indirect_x_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                self.adc(data);
            }
            0x65 => {
                // ADC $NN (Zero Page)
                let addr = self.zero_page_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                self.adc(data);
            }
            0x66 => {
                // ROR $NN (Zero Page)
                let addr = self.zero_page_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                let result = self.ror(data);
                memory.write(addr, result);
            }
            0x68 => {
                // PLA (Pull Accumulator)
                self.sp = self.sp.wrapping_add(1);
                self.a = memory.read(0x0100 + self.sp as u16);
            }
            0x69 => {
                // ADC #$NN
                let immediate = memory.read(self.pc);
                self.pc += 1;
                self.adc(immediate);
            }
            0x6A => {
                // ROR A (Accumulator)
                let result = self.ror(self.a);
                self.a = result;
            }
            0x6C => {
                // JMP ($NNNN)
                let addr = self.indirect_addr(memory);
                self.pc = addr;
            }
            0x6D => {
                // ADC $NNNN
                let addr = self.absolute_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                self.adc(data);
            }
            0x6E => {
                // ROR $NNNN
                let addr = self.absolute_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                let result = self.ror(data);
                memory.write(addr, result);
            }
            0x70 => {
                // BVS (Branch on Overflow Set)
                let offset = memory.read(self.pc) as i8;
                self.pc += 1;
                if self.status.v {
                    self.pc = self.pc.wrapping_add(offset as u16);
                }
            }
            0x71 => {
                // ADC ($NN),Y
                let addr = self.indirect_y_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                self.adc(data);
            }
            0x75 => {
                // ADC $NN,X (Zero Page,X)
                let addr = self.zero_page_x_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                self.adc(data);
            }
            0x76 => {
                // ROR $NN,X (Zero Page,X)
                let addr = self.zero_page_x_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                let result = self.ror(data);
                memory.write(addr, result);
            }
            0x78 => {
                // SEI (Set Interrupt Disable)
                self.status.i = true;
            }
            0x79 => {
                // ADC $NNNN,Y
                let addr = self.absolute_y_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                self.adc(data);
            }
            0x7D => {
                // ADC $NNNN,X
                let addr = self.absolute_x_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                self.adc(data);
            }
            0x7E => {
                // ROR $NNNN,X
                let addr = self.absolute_x_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                let result = self.ror(data);
                memory.write(addr, result);
            }
            0x80 => {
                // BRA (Branch Always)
                let offset = memory.read(self.pc) as i8;
                self.pc += 1;
                self.pc = self.pc.wrapping_add(offset as u16);
            }
            0x81 => {
                // STA ($NN,X)
                let addr = self.indirect_x_addr(memory);
                self.pc += 1;
                memory.write(addr, self.a);
            }
            0x84 => {
                // STY $NN (Zero Page)
                let addr = self.zero_page_addr(memory);
                self.pc += 1;
                memory.write(addr, self.y);
            }
            0x85 => {
                // STA $NN (Zero Page)
                let addr = self.zero_page_addr(memory);
                self.pc += 1;
                memory.write(addr, self.a);
            }
            0x86 => {
                // STX $NN (Zero Page)
                let addr = self.zero_page_addr(memory);
                self.pc += 1;
                memory.write(addr, self.x);
            }
            0x88 => {
                // DEY (Decrement Y Register)
                self.y = self.y.wrapping_sub(1);
                self.status.z = self.y == 0;
                self.status.n = self.y & 0x80 != 0;
            }
            0x89 => {
                // BIT $NNNN
                let addr = self.absolute_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                self.bit(data);
            }
            0x8A => {
                // TXA (Transfer X Register to Accumulator)
                self.a = self.x;
                self.status.z = self.a == 0;
                self.status.n = self.a & 0x80 != 0;
            }
            0x8C => {
                // STY $NNNN
                let addr = self.absolute_addr(memory);
                self.pc += 2;
                memory.write(addr, self.y);
            }
            0x8D => {
                // STA $NNNN
                let addr = self.absolute_addr(memory);
                self.pc += 2;
                memory.write(addr, self.a);
            }
            0x8E => {
                // STX $NNNN
                let addr = self.absolute_addr(memory);
                self.pc += 2;
                memory.write(addr, self.x);
            }
            0x90 => {
                // BCC (Branch on Carry Clear)
                let offset = memory.read(self.pc) as i8;
                self.pc += 1;
                if !self.status.c {
                    self.pc = self.pc.wrapping_add(offset as u16);
                }
            }
            0x91 => {
                // STA ($NN),Y
                let addr = self.indirect_y_addr(memory);
                self.pc += 1;
                memory.write(addr, self.a);
            }
            0x94 => {
                // STY $NN,X (Zero Page,X)
                let addr = self.zero_page_x_addr(memory);
                self.pc += 1;
                memory.write(addr, self.y);
            }
            0x95 => {
                // STA $NN,X (Zero Page,X)
                let addr = self.zero_page_x_addr(memory);
                self.pc += 1;
                memory.write(addr, self.a);
            }
            0x96 => {
                // STX $NN,Y (Zero Page,Y)
                let addr = self.zero_page_y_addr(memory);
                self.pc += 1;
                memory.write(addr, self.x);
            }
            0x98 => {
                // TYA (Transfer Y Register to Accumulator)
                self.a = self.y;
                self.status.z = self.a == 0;
                self.status.n = self.a & 0x80 != 0;
            }
            0x99 => {
                // STA $NNNN,Y
                let addr = self.absolute_y_addr(memory);
                self.pc += 2;
                memory.write(addr, self.a);
            }
            0x9A => {
                // TXS (Transfer X Register to Stack Pointer)
                self.sp = self.x;
            }
            0x9D => {
                // STA $NNNN,X
                let addr = self.absolute_x_addr(memory);
                self.pc += 2;
                memory.write(addr, self.a);
            }
            0xA0 => {
                // LDY #$NN
                let immediate = memory.read(self.pc);
                self.pc += 1;
                self.y = immediate;
                self.status.z = self.y == 0;
                self.status.n = self.y & 0x80 != 0;
            }
            0xA1 => {
                // LDA ($NN,X)
                let addr = self.indirect_x_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                self.lda(data);
            }
            0xA2 => {
                // LDX #$NN
                let immediate = memory.read(self.pc);
                self.pc += 1;
                self.x = immediate;
                self.status.z = self.x == 0;
                self.status.n = self.x & 0x80 != 0;
            }
            0xA4 => {
                // LDY $NN (Zero Page)
                let addr = self.zero_page_addr(memory);
                self.pc += 1;
                self.y = memory.read(addr);
                self.status.z = self.y == 0;
                self.status.n = self.y & 0x80 != 0;
            }
            0xA5 => {
                // LDA $NN (Zero Page)
                let addr = self.zero_page_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                self.lda(data);
            }
            0xA6 => {
                // LDX $NN (Zero Page)
                let addr = self.zero_page_addr(memory);
                self.pc += 1;
                self.x = memory.read(addr);
                self.status.z = self.x == 0;
                self.status.n = self.x & 0x80 != 0;
            }
            0xA8 => {
                // TAY (Transfer Accumulator to Y Register)
                self.y = self.a;
                self.status.z = self.y == 0;
                self.status.n = self.y & 0x80 != 0;
            }
            0xA9 => {
                // LDA #$NN
                let immediate = memory.read(self.pc);
                self.pc += 1;
                self.lda(immediate);
            }
            0xAA => {
                // TAX (Transfer Accumulator to X Register)
                self.x = self.a;
                self.status.z = self.x == 0;
                self.status.n = self.x & 0x80 != 0;
            }
            0xAC => {
                // LDY $NNNN
                let addr = self.absolute_addr(memory);
                self.pc += 2;
                self.y = memory.read(addr);
                self.status.z = self.y == 0;
                self.status.n = self.y & 0x80 != 0;
            }
            0xAD => {
                // LDA $NNNN
                let addr = self.absolute_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                self.lda(data);
            }
            0xAE => {
                // LDX $NNNN
                let addr = self.absolute_addr(memory);
                self.pc += 2;
                self.x = memory.read(addr);
                self.status.z = self.x == 0;
                self.status.n = self.x & 0x80 != 0;
            }
            0xB0 => {
                // BCS (Branch on Carry Set)
                let offset = memory.read(self.pc) as i8;
                self.pc += 1;
                if self.status.c {
                    self.pc = self.pc.wrapping_add(offset as u16);
                }
            }
            0xB1 => {
                // LDA ($NN),Y
                let addr = self.indirect_y_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                self.lda(data);
            }
            0xB4 => {
                // LDY $NN,X (Zero Page,X)
                let addr = self.zero_page_x_addr(memory);
                self.pc += 1;
                self.y = memory.read(addr);
                self.status.z = self.y == 0;
                self.status.n = self.y & 0x80 != 0;
            }
            0xB5 => {
                // LDA $NN,X (Zero Page,X)
                let addr = self.zero_page_x_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                self.lda(data);
            }
            0xB6 => {
                // LDX $NN,Y (Zero Page,Y)
                let addr = self.zero_page_y_addr(memory);
                self.pc += 1;
                self.x = memory.read(addr);
                self.status.z = self.x == 0;
                self.status.n = self.x & 0x80 != 0;
            }
            0xB8 => {
                // CLV (Clear Overflow Flag)
                self.status.v = false;
            }
            0xB9 => {
                // LDA $NNNN,Y
                let addr = self.absolute_y_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                self.lda(data);
            }
            0xBA => {
                // TSX (Transfer Stack Pointer to X Register)
                self.x = self.sp;
                self.status.z = self.x == 0;
                self.status.n = self.x & 0x80 != 0;
            }
            0xBC => {
                // LDY $NNNN,X
                let addr = self.absolute_x_addr(memory);
                self.pc += 2;
                self.y = memory.read(addr);
                self.status.z = self.y == 0;
                self.status.n = self.y & 0x80 != 0;
            }
            0xBD => {
                // LDA $NNNN,X
                let addr = self.absolute_x_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                self.lda(data);
            }
            0xBE => {
                // LDX $NNNN,Y
                let addr = self.absolute_y_addr(memory);
                self.pc += 2;
                self.x = memory.read(addr);
                self.status.z = self.x == 0;
                self.status.n = self.x & 0x80 != 0;
            }
            0xC0 => {
                // CPY #$NN
                let immediate = memory.read(self.pc);
                self.pc += 1;
                let result = self.y.wrapping_sub(immediate);
                self.status.c = self.y >= immediate;
                self.status.z = result == 0;
                self.status.n = result & 0x80 != 0;
            }
            0xC1 => {
                // CMP ($NN,X)
                let addr = self.indirect_x_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                self.cmp(data);
            }
            0xC4 => {
                // CPY $NN (Zero Page)
                let addr = self.zero_page_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                let result = self.y.wrapping_sub(data);
                self.status.c = self.y >= data;
                self.status.z = result == 0;
                self.status.n = result & 0x80 != 0;
            }
            0xC5 => {
                // CMP $NN (Zero Page)
                let addr = self.zero_page_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                self.cmp(data);
            }
            0xC6 => {
                // DEC $NN (Zero Page)
                let addr = self.zero_page_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                let result = self.dec(data);
                memory.write(addr, result);
            }
            0xC8 => {
                // INY (Increment Y Register)
                self.y = self.y.wrapping_add(1);
                self.status.z = self.y == 0;
                self.status.n = self.y & 0x80 != 0;
            }
            0xC9 => {
                // CMP #$NN
                let immediate = memory.read(self.pc);
                self.pc += 1;
                self.cmp(immediate);
            }
            0xCA => {
                // DEX (Decrement X Register)
                self.x = self.x.wrapping_sub(1);
                self.status.z = self.x == 0;
                self.status.n = self.x & 0x80 != 0;
            }
            0xCC => {
                // CPY $NNNN
                let addr = self.absolute_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                let result = self.y.wrapping_sub(data);
                self.status.c = self.y >= data;
                self.status.z = result == 0;
                self.status.n = result & 0x80 != 0;
            }
            0xCD => {
                // CMP $NNNN
                let addr = self.absolute_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                self.cmp(data);
            }
            0xCE => {
                // DEC $NNNN
                let addr = self.absolute_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                let result = self.dec(data);
                memory.write(addr, result);
            }
            0xD0 => {
                // BNE (Branch on Not Equal)
                let offset = memory.read(self.pc) as i8;
                self.pc += 1;
                if !self.status.z {
                    self.pc = self.pc.wrapping_add(offset as u16);
                }
            }
            0xD1 => {
                // CMP ($NN),Y
                let addr = self.indirect_y_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                self.cmp(data);
            }
            0xD5 => {
                // CMP $NN,X (Zero Page,X)
                let addr = self.zero_page_x_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                self.cmp(data);
            }
            0xD6 => {
                // DEC $NN,X (Zero Page,X)
                let addr = self.zero_page_x_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                let result = self.dec(data);
                memory.write(addr, result);
            }
            0xD8 => {
                // CLD (Clear Decimal Mode)
                self.status.d = false;
            }
            0xD9 => {
                // CMP $NNNN,Y
                let addr = self.absolute_y_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                self.cmp(data);
            }
            0xDD => {
                // CMP $NNNN,X
                let addr = self.absolute_x_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                self.cmp(data);
            }
            0xDE => {
                // DEC $NNNN,X
                let addr = self.absolute_x_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                let result = self.dec(data);
                memory.write(addr, result);
            }
            0xE0 => {
                // CPX #$NN
                let immediate = memory.read(self.pc);
                self.pc += 1;
                let result = self.x.wrapping_sub(immediate);
                self.status.c = self.x >= immediate;
                self.status.z = result == 0;
                self.status.n = result & 0x80 != 0;
            }
            0xE1 => {
                // SBC ($NN,X)
                let addr = self.indirect_x_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                self.sbc(data);
            }
            0xE4 => {
                // CPX $NN (Zero Page)
                let addr = self.zero_page_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                let result = self.x.wrapping_sub(data);
                self.status.c = self.x >= data;
                self.status.z = result == 0;
                self.status.n = result & 0x80 != 0;
            }
            0xE5 => {
                // SBC $NN (Zero Page)
                let addr = self.zero_page_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                self.sbc(data);
            }
            0xE6 => {
                // INC $NN (Zero Page)
                let addr = self.zero_page_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                let result = self.inc(data);
                memory.write(addr, result);
            }
            0xE8 => {
                // INX (Increment X Register)
                self.x = self.x.wrapping_add(1);
                self.status.z = self.x == 0;
                self.status.n = self.x & 0x80 != 0;
            }
            0xE9 => {
                // SBC #$NN
                let immediate = memory.read(self.pc);
                self.pc += 1;
                self.sbc(immediate);
            }
            0xEA => {
                // NOP (No Operation)
            }
            0xEC => {
                // CPX $NNNN
                let addr = self.absolute_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                let result = self.x.wrapping_sub(data);
                self.status.c = self.x >= data;
                self.status.z = result == 0;
                self.status.n = result & 0x80 != 0;
            }
            0xED => {
                // SBC $NNNN
                let addr = self.absolute_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                self.sbc(data);
            }
            0xEE => {
                // INC $NNNN
                let addr = self.absolute_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                let result = self.inc(data);
                memory.write(addr, result);
            }
            0xF0 => {
                // BEQ (Branch on Equal)
                let offset = memory.read(self.pc) as i8;
                self.pc += 1;
                if self.status.z {
                    self.pc = self.pc.wrapping_add(offset as u16);
                }
            }
            0xF1 => {
                // SBC ($NN),Y
                let addr = self.indirect_y_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                self.sbc(data);
            }
            0xF5 => {
                // SBC $NN,X (Zero Page,X)
                let addr = self.zero_page_x_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                self.sbc(data);
            }
            0xF6 => {
                // INC $NN,X (Zero Page,X)
                let addr = self.zero_page_x_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                let result = self.inc(data);
                memory.write(addr, result);
            }
            0xF8 => {
                // SED (Set Decimal Mode)
                self.status.d = true;
            }
            0xF9 => {
                // SBC $NNNN,Y
                let addr = self.absolute_y_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                self.sbc(data);
            }
            0xFD => {
                // SBC $NNNN,X
                let addr = self.absolute_x_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                self.sbc(data);
            }
            0xFE => {
                // INC $NNNN,X
                let addr = self.absolute_x_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                let result = self.inc(data);
                memory.write(addr, result);
            }
            0x4E => {
                // LSR $NNNN
                let addr = self.absolute_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                let result = self.lsr(data);
                memory.write(addr, result);
            }
            _ => panic!("Unimplemented instruction code: {:02X}", opcode),
        }
    }

    // --- Address acquisition functions for various addressing modes ---
    fn zero_page_addr(&self, memory: &Memory) -> u16 {
        memory.read(self.pc) as u16
    }

    fn zero_page_x_addr(&self, memory: &Memory) -> u16 {
        (memory.read(self.pc) as u16 + self.x as u16) & 0xFF
    }

    fn zero_page_y_addr(&self, memory: &Memory) -> u16 {
        (memory.read(self.pc) as u16 + self.y as u16) & 0xFF
    }

    fn absolute_addr(&self, memory: &Memory) -> u16 {
        let lo = memory.read(self.pc) as u16;
        let hi = memory.read(self.pc + 1) as u16;
        (hi << 8) | lo
    }

    fn absolute_x_addr(&self, memory: &Memory) -> u16 {
        self.absolute_addr(memory) + self.x as u16
    }

    fn absolute_y_addr(&self, memory: &Memory) -> u16 {
        self.absolute_addr(memory) + self.y as u16
    }

    fn indirect_addr(&self, memory: &Memory) -> u16 {
        // JMP ($addr) with 6502 page-wrap bug on high-byte fetch
        let ptr_lo = memory.read(self.pc) as u16;
        let ptr_hi = memory.read(self.pc + 1) as u16;
        let ptr = (ptr_hi << 8) | ptr_lo;
        let lo = memory.read(ptr) as u16;
        let hi_addr = (ptr & 0xFF00) | ((ptr + 1) & 0x00FF);
        let hi = memory.read(hi_addr) as u16;
        (hi << 8) | lo
    }

    fn indirect_x_addr(&self, memory: &Memory) -> u16 {
        let addr = (memory.read(self.pc) as u16 + self.x as u16) & 0xFF;
        let lo = memory.read(addr) as u16;
        let hi = memory.read((addr + 1) & 0xFF) as u16;
        (hi << 8) | lo
    }

    fn indirect_y_addr(&self, memory: &Memory) -> u16 {
        let zp = memory.read(self.pc) as u16;
        let lo = memory.read(zp) as u16;
        let hi = memory.read((zp + 1) & 0xFF) as u16;
        let base = (hi << 8) | lo;
        base.wrapping_add(self.y as u16)
    }

    // --- Implementation of various instructions ---
    fn lda(&mut self, value: u8) {
        self.a = value;
        self.status.z = self.a == 0;
        self.status.n = self.a & 0x80 != 0;
    }

    fn adc(&mut self, value: u8) {
        let sum = self.a as u16 + value as u16 + self.status.c as u16;
        self.status.c = sum > 0xFF;
        self.status.v = ((self.a ^ value) & 0x80 == 0) && ((self.a as u16 ^ sum) & 0x80 != 0);
        self.a = (sum & 0xFF) as u8;
        self.status.z = self.a == 0;
        self.status.n = self.a & 0x80 != 0;
    }

    fn ora(&mut self, value: u8) {
        self.a |= value;
        self.status.z = self.a == 0;
        self.status.n = self.a & 0x80 != 0;
    }

    fn eor(&mut self, value: u8) {
        self.a ^= value;
        self.status.z = self.a == 0;
        self.status.n = self.a & 0x80 != 0;
    }

    fn and(&mut self, value: u8) {
        self.a &= value;
        self.status.z = self.a == 0;
        self.status.n = self.a & 0x80 != 0;
    }

    fn bit(&mut self, value: u8) {
        let result = self.a & value;
        self.status.z = result == 0;
        self.status.n = (value & 0x80) != 0; // N = M7 [4]
        self.status.v = (value & 0x40) != 0; // V = M6 [4]
    }

    fn lsr(&mut self, value: u8) -> u8 {
        self.status.c = (value & 0x01) != 0;
        let result = value >> 1;
        self.status.z = result == 0;
        self.status.n = (result & 0x80) != 0;
        result
    }

    fn asl(&mut self, value: u8) -> u8 {
        self.status.c = (value & 0x80) != 0;
        let result = value << 1;
        self.status.z = result == 0;
        self.status.n = (result & 0x80) != 0;
        result
    }

    fn rol(&mut self, value: u8) -> u8 {
        let old_carry = self.status.c;
        self.status.c = (value & 0x80) != 0;
        let result = (value << 1) | (old_carry as u8);
        self.status.z = result == 0;
        self.status.n = (result & 0x80) != 0;
        result
    }

    fn ror(&mut self, value: u8) -> u8 {
        let old_carry = self.status.c;
        self.status.c = (value & 0x01) != 0;
        let result = (value >> 1) | ((old_carry as u8) << 7);
        self.status.z = result == 0;
        self.status.n = (result & 0x80) != 0;
        result
    }

    fn cmp(&mut self, value: u8) {
        let result = self.a as i16 - value as i16;
        self.status.c = result >= 0;
        self.status.z = self.a == value;
        self.status.n = (result & 0x80) != 0;
    }

    fn dec(&mut self, value: u8) -> u8 {
        let result = value.wrapping_sub(1);
        self.status.z = result == 0;
        self.status.n = result & 0x80 != 0;
        result
    }

    fn inc(&mut self, value: u8) -> u8 {
        let result = value.wrapping_add(1);
        self.status.z = result == 0;
        self.status.n = result & 0x80 != 0;
        result
    }

    fn sbc(&mut self, value: u8) {
        let result = self.a as i16 - value as i16 - (!self.status.c as i16);
        self.status.c = result >= 0;
        self.status.v = ((self.a ^ value) & 0x80 != 0) && ((self.a ^ result as u8) & 0x80 != 0);
        self.a = (result & 0xFF) as u8;
        self.status.z = self.a == 0;
        self.status.n = (result & 0x80) != 0;
    }
}
