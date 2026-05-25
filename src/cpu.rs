use crate::memory::Memory;
use crate::opcodes::*;
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
            BRK => {
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
            ORA_INDX => {
                // ORA ($NN,X)
                let addr = self.indirect_x_addr(memory);
                let data = memory.read(addr);
                self.ora(data);
            }
            ORA_ZP => {
                // ORA $NN (Zero Page)
                let addr = self.zero_page_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                self.ora(data);
            }
            ASL_ZP => {
                // ASL $NN (Zero Page)
                let addr = self.zero_page_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                let result = self.asl(data);
                memory.write(addr, result);
            }
            PHP => {
                // PHP (Push Processor Status)
                memory.write(0x0100 + self.sp as u16, self.status.as_byte());
                self.sp = self.sp.wrapping_sub(1);
            }
            ORA_IMM => {
                // ORA #$NN
                let immediate = memory.read(self.pc);
                self.pc += 1;
                self.ora(immediate);
            }
            ASL_ACC => {
                // ASL A (Accumulator)
                let result = self.asl(self.a);
                self.a = result;
            }
            ORA_ABS => {
                // ORA $NNNN (Absolute)
                let addr = self.absolute_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                self.ora(data);
            }
            ASL_ABS => {
                // ASL $NNNN (Absolute)
                let addr = self.absolute_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                let result = self.asl(data);
                memory.write(addr, result);
            }
            BPL => {
                // BPL (Branch on Plus)
                let offset = memory.read(self.pc) as i8;
                self.pc += 1;
                if !self.status.n {
                    self.pc = self.pc.wrapping_add(offset as u16);
                }
            }
            ORA_INDY => {
                // ORA ($NN),Y
                let addr = self.indirect_y_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                self.ora(data);
            }
            ORA_ZPX => {
                // ORA $NN,X (Zero Page,X)
                let addr = self.zero_page_x_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                self.ora(data);
            }
            ASL_ZPX => {
                // ASL $NN,X (Zero Page,X)
                let addr = self.zero_page_x_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                let result = self.asl(data);
                memory.write(addr, result);
            }
            CLC => {
                // CLC (Clear Carry)
                self.status.c = false;
            }
            ORA_ABSY => {
                // ORA $NNNN,Y (Absolute,Y)
                let addr = self.absolute_y_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                self.ora(data);
            }
            ORA_ABSX => {
                // ORA $NNNN,X (Absolute,X)
                let addr = self.absolute_x_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                self.ora(data);
            }
            ASL_ABSX => {
                // ASL $NNNN,X (Absolute,X)
                let addr = self.absolute_x_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                let result = self.asl(data);
                memory.write(addr, result);
            }
            JSR => {
                // JSR $NNNN
                let target = self.absolute_addr(memory);
                let return_addr = self.pc + 1; // Address of last operand byte
                memory.write(0x0100 + self.sp as u16, ((return_addr >> 8) & 0xFF) as u8);
                self.sp = self.sp.wrapping_sub(1);
                memory.write(0x0100 + self.sp as u16, (return_addr & 0xFF) as u8);
                self.sp = self.sp.wrapping_sub(1);
                self.pc = target;
            }
            AND_INDX => {
                // AND ($NN,X)
                let addr = self.indirect_x_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                self.and(data);
            }
            BIT_ZP => {
                // BIT $NN (Zero Page)
                let addr = self.zero_page_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                self.bit(data);
            }
            AND_ZP => {
                // AND $NN (Zero Page)
                let addr = self.zero_page_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                self.and(data);
            }
            ROL_ZP => {
                // ROL $NN (Zero Page)
                let addr = self.zero_page_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                let result = self.rol(data);
                memory.write(addr, result);
            }
            PLP => {
                // PLP (Pull Processor Status)
                self.sp = self.sp.wrapping_add(1);
                let status = memory.read(0x0100 + self.sp as u16);
                // ステータスレジスタを復元する処理を追加する必要があります
                self.status = StatusRegister::from_byte(status);
            }
            AND_IMM => {
                // AND #$NN
                let immediate = memory.read(self.pc);
                self.pc += 1;
                self.and(immediate);
            }
            ROL_ACC => {
                // ROL A
                let result = self.rol(self.a);
                self.a = result;
            }
            BIT_ABS => {
                // BIT $NNNN
                let addr = self.absolute_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                self.bit(data);
            }
            AND_ABS => {
                // AND $NNNN
                let addr = self.absolute_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                self.and(data);
            }
            ROL_ABS => {
                // ROL $NNNN (Absolute)
                let addr = self.absolute_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                let result = self.rol(data);
                memory.write(addr, result);
            }
            BMI => {
                // BMI (Branch on Minus)
                let offset = memory.read(self.pc) as i8;
                self.pc += 1;
                if self.status.n {
                    self.pc = self.pc.wrapping_add(offset as u16);
                }
            }
            AND_INDY => {
                // AND ($NN),Y
                let addr = self.indirect_y_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                self.and(data);
            }
            AND_ZPX => {
                // AND $NN,X
                let addr = self.zero_page_x_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                self.and(data);
            }
            ROL_ZPX => {
                // ROL $NN,X
                let addr = self.zero_page_x_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                let result = self.rol(data);
                memory.write(addr, result);
            }
            SEC => {
                // SEC (Set Carry Flag)
                self.status.c = true;
            }
            AND_ABSY => {
                // AND $NNNN,Y
                let addr = self.absolute_y_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                self.and(data);
            }
            AND_ABSX => {
                // AND $NNNN,X
                let addr = self.absolute_x_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                self.and(data);
            }
            ROL_ABSX => {
                // ROL $NNNN,X
                let addr = self.absolute_x_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                let result = self.rol(data);
                memory.write(addr, result);
            }
            RTI => {
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
            EOR_INDX => {
                // EOR ($NN,X)
                let addr = self.indirect_x_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                self.eor(data);
            }
            EOR_ZP => {
                // EOR $NN (Zero Page)
                let addr = self.zero_page_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                self.eor(data);
            }
            LSR_ZP => {
                // LSR $NN (Zero Page)
                let addr = self.zero_page_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                let result = self.lsr(data);
                memory.write(addr, result);
            }
            PHA => {
                // PHA (Push Accumulator)
                memory.write(0x0100 + self.sp as u16, self.a);
                self.sp = self.sp.wrapping_sub(1);
            }
            EOR_IMM => {
                // EOR #$NN
                let immediate = memory.read(self.pc);
                self.pc += 1;
                self.eor(immediate);
            }
            LSR_ACC => {
                // LSR A (Accumulator)
                let result = self.lsr(self.a);
                self.a = result;
            }
            JMP_ABS => {
                // JMP $NNNN
                let addr = self.absolute_addr(memory);
                self.pc = addr;
            }
            EOR_ABS => {
                // EOR $NNNN
                let addr = self.absolute_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                self.eor(data);
            }
            BVC => {
                // BVC (Branch on Overflow Clear)
                let offset = memory.read(self.pc) as i8;
                self.pc += 1;
                if !self.status.v {
                    self.pc = self.pc.wrapping_add(offset as u16);
                }
            }
            EOR_INDY => {
                // EOR ($NN),Y
                let addr = self.indirect_y_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                self.eor(data);
            }
            EOR_ZPX => {
                // EOR $NN,X (Zero Page,X)
                let addr = self.zero_page_x_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                self.eor(data);
            }
            LSR_ZPX => {
                // LSR $NN,X (Zero Page,X)
                let addr = self.zero_page_x_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                let result = self.lsr(data);
                memory.write(addr, result);
            }
            CLI => {
                // CLI (Clear Interrupt Disable)
                self.status.i = false;
            }
            EOR_ABSY => {
                // EOR $NNNN,Y
                let addr = self.absolute_y_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                self.eor(data);
            }
            EOR_ABSX => {
                // EOR $NNNN,X
                let addr = self.absolute_x_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                self.eor(data);
            }
            LSR_ABSX => {
                // LSR $NNNN,X
                let addr = self.absolute_x_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                let result = self.lsr(data);
                memory.write(addr, result);
            }
            RTS => {
                // RTS (Return from Subroutine)
                self.sp = self.sp.wrapping_add(1);
                let lo = memory.read(0x0100 + self.sp as u16) as u16;
                self.sp = self.sp.wrapping_add(1);
                let hi = memory.read(0x0100 + self.sp as u16) as u16;
                self.pc = ((hi << 8) | lo).wrapping_add(1);
            }
            ADC_INDX => {
                // ADC ($NN,X)
                let addr = self.indirect_x_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                self.adc(data);
            }
            ADC_ZP => {
                // ADC $NN (Zero Page)
                let addr = self.zero_page_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                self.adc(data);
            }
            ROR_ZP => {
                // ROR $NN (Zero Page)
                let addr = self.zero_page_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                let result = self.ror(data);
                memory.write(addr, result);
            }
            PLA => {
                // PLA (Pull Accumulator)
                self.sp = self.sp.wrapping_add(1);
                self.a = memory.read(0x0100 + self.sp as u16);
            }
            ADC_IMM => {
                // ADC #$NN
                let immediate = memory.read(self.pc);
                self.pc += 1;
                self.adc(immediate);
            }
            ROR_ACC => {
                // ROR A (Accumulator)
                let result = self.ror(self.a);
                self.a = result;
            }
            JMP_IND => {
                // JMP ($NNNN)
                let addr = self.indirect_addr(memory);
                self.pc = addr;
            }
            ADC_ABS => {
                // ADC $NNNN
                let addr = self.absolute_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                self.adc(data);
            }
            ROR_ABS => {
                // ROR $NNNN
                let addr = self.absolute_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                let result = self.ror(data);
                memory.write(addr, result);
            }
            BVS => {
                // BVS (Branch on Overflow Set)
                let offset = memory.read(self.pc) as i8;
                self.pc += 1;
                if self.status.v {
                    self.pc = self.pc.wrapping_add(offset as u16);
                }
            }
            ADC_INDY => {
                // ADC ($NN),Y
                let addr = self.indirect_y_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                self.adc(data);
            }
            ADC_ZPX => {
                // ADC $NN,X (Zero Page,X)
                let addr = self.zero_page_x_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                self.adc(data);
            }
            ROR_ZPX => {
                // ROR $NN,X (Zero Page,X)
                let addr = self.zero_page_x_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                let result = self.ror(data);
                memory.write(addr, result);
            }
            SEI => {
                // SEI (Set Interrupt Disable)
                self.status.i = true;
            }
            ADC_ABSY => {
                // ADC $NNNN,Y
                let addr = self.absolute_y_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                self.adc(data);
            }
            ADC_ABSX => {
                // ADC $NNNN,X
                let addr = self.absolute_x_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                self.adc(data);
            }
            ROR_ABSX => {
                // ROR $NNNN,X
                let addr = self.absolute_x_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                let result = self.ror(data);
                memory.write(addr, result);
            }
            BRA => {
                // BRA (Branch Always)
                let offset = memory.read(self.pc) as i8;
                self.pc += 1;
                self.pc = self.pc.wrapping_add(offset as u16);
            }
            STA_INDX => {
                // STA ($NN,X)
                let addr = self.indirect_x_addr(memory);
                self.pc += 1;
                memory.write(addr, self.a);
            }
            STY_ZP => {
                // STY $NN (Zero Page)
                let addr = self.zero_page_addr(memory);
                self.pc += 1;
                memory.write(addr, self.y);
            }
            STA_ZP => {
                // STA $NN (Zero Page)
                let addr = self.zero_page_addr(memory);
                self.pc += 1;
                memory.write(addr, self.a);
            }
            STX_ZP => {
                // STX $NN (Zero Page)
                let addr = self.zero_page_addr(memory);
                self.pc += 1;
                memory.write(addr, self.x);
            }
            DEY => {
                // DEY (Decrement Y Register)
                self.y = self.y.wrapping_sub(1);
                self.status.z = self.y == 0;
                self.status.n = self.y & 0x80 != 0;
            }
            BIT_IMM => {
                // BIT $NNNN
                let addr = self.absolute_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                self.bit(data);
            }
            TXA => {
                // TXA (Transfer X Register to Accumulator)
                self.a = self.x;
                self.status.z = self.a == 0;
                self.status.n = self.a & 0x80 != 0;
            }
            STY_ABS => {
                // STY $NNNN
                let addr = self.absolute_addr(memory);
                self.pc += 2;
                memory.write(addr, self.y);
            }
            STA_ABS => {
                // STA $NNNN
                let addr = self.absolute_addr(memory);
                self.pc += 2;
                memory.write(addr, self.a);
            }
            STX_ABS => {
                // STX $NNNN
                let addr = self.absolute_addr(memory);
                self.pc += 2;
                memory.write(addr, self.x);
            }
            BCC => {
                // BCC (Branch on Carry Clear)
                let offset = memory.read(self.pc) as i8;
                self.pc += 1;
                if !self.status.c {
                    self.pc = self.pc.wrapping_add(offset as u16);
                }
            }
            STA_INDY => {
                // STA ($NN),Y
                let addr = self.indirect_y_addr(memory);
                self.pc += 1;
                memory.write(addr, self.a);
            }
            STY_ZPX => {
                // STY $NN,X (Zero Page,X)
                let addr = self.zero_page_x_addr(memory);
                self.pc += 1;
                memory.write(addr, self.y);
            }
            STA_ZPX => {
                // STA $NN,X (Zero Page,X)
                let addr = self.zero_page_x_addr(memory);
                self.pc += 1;
                memory.write(addr, self.a);
            }
            STX_ZPY => {
                // STX $NN,Y (Zero Page,Y)
                let addr = self.zero_page_y_addr(memory);
                self.pc += 1;
                memory.write(addr, self.x);
            }
            TYA => {
                // TYA (Transfer Y Register to Accumulator)
                self.a = self.y;
                self.status.z = self.a == 0;
                self.status.n = self.a & 0x80 != 0;
            }
            STA_ABSY => {
                // STA $NNNN,Y
                let addr = self.absolute_y_addr(memory);
                self.pc += 2;
                memory.write(addr, self.a);
            }
            TXS => {
                // TXS (Transfer X Register to Stack Pointer)
                self.sp = self.x;
            }
            STA_ABSX => {
                // STA $NNNN,X
                let addr = self.absolute_x_addr(memory);
                self.pc += 2;
                memory.write(addr, self.a);
            }
            LDY_IMM => {
                // LDY #$NN
                let immediate = memory.read(self.pc);
                self.pc += 1;
                self.y = immediate;
                self.status.z = self.y == 0;
                self.status.n = self.y & 0x80 != 0;
            }
            LDA_INDX => {
                // LDA ($NN,X)
                let addr = self.indirect_x_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                self.lda(data);
            }
            LDX_IMM => {
                // LDX #$NN
                let immediate = memory.read(self.pc);
                self.pc += 1;
                self.x = immediate;
                self.status.z = self.x == 0;
                self.status.n = self.x & 0x80 != 0;
            }
            LDY_ZP => {
                // LDY $NN (Zero Page)
                let addr = self.zero_page_addr(memory);
                self.pc += 1;
                self.y = memory.read(addr);
                self.status.z = self.y == 0;
                self.status.n = self.y & 0x80 != 0;
            }
            LDA_ZP => {
                // LDA $NN (Zero Page)
                let addr = self.zero_page_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                self.lda(data);
            }
            LDX_ZP => {
                // LDX $NN (Zero Page)
                let addr = self.zero_page_addr(memory);
                self.pc += 1;
                self.x = memory.read(addr);
                self.status.z = self.x == 0;
                self.status.n = self.x & 0x80 != 0;
            }
            TAY => {
                // TAY (Transfer Accumulator to Y Register)
                self.y = self.a;
                self.status.z = self.y == 0;
                self.status.n = self.y & 0x80 != 0;
            }
            LDA_IMM => {
                // LDA #$NN
                let immediate = memory.read(self.pc);
                self.pc += 1;
                self.lda(immediate);
            }
            TAX => {
                // TAX (Transfer Accumulator to X Register)
                self.x = self.a;
                self.status.z = self.x == 0;
                self.status.n = self.x & 0x80 != 0;
            }
            LDY_ABS => {
                // LDY $NNNN
                let addr = self.absolute_addr(memory);
                self.pc += 2;
                self.y = memory.read(addr);
                self.status.z = self.y == 0;
                self.status.n = self.y & 0x80 != 0;
            }
            LDA_ABS => {
                // LDA $NNNN
                let addr = self.absolute_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                self.lda(data);
            }
            LDX_ABS => {
                // LDX $NNNN
                let addr = self.absolute_addr(memory);
                self.pc += 2;
                self.x = memory.read(addr);
                self.status.z = self.x == 0;
                self.status.n = self.x & 0x80 != 0;
            }
            BCS => {
                // BCS (Branch on Carry Set)
                let offset = memory.read(self.pc) as i8;
                self.pc += 1;
                if self.status.c {
                    self.pc = self.pc.wrapping_add(offset as u16);
                }
            }
            LDA_INDY => {
                // LDA ($NN),Y
                let addr = self.indirect_y_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                self.lda(data);
            }
            LDY_ZPX => {
                // LDY $NN,X (Zero Page,X)
                let addr = self.zero_page_x_addr(memory);
                self.pc += 1;
                self.y = memory.read(addr);
                self.status.z = self.y == 0;
                self.status.n = self.y & 0x80 != 0;
            }
            LDA_ZPX => {
                // LDA $NN,X (Zero Page,X)
                let addr = self.zero_page_x_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                self.lda(data);
            }
            LDX_ZPY => {
                // LDX $NN,Y (Zero Page,Y)
                let addr = self.zero_page_y_addr(memory);
                self.pc += 1;
                self.x = memory.read(addr);
                self.status.z = self.x == 0;
                self.status.n = self.x & 0x80 != 0;
            }
            CLV => {
                // CLV (Clear Overflow Flag)
                self.status.v = false;
            }
            LDA_ABSY => {
                // LDA $NNNN,Y
                let addr = self.absolute_y_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                self.lda(data);
            }
            TSX => {
                // TSX (Transfer Stack Pointer to X Register)
                self.x = self.sp;
                self.status.z = self.x == 0;
                self.status.n = self.x & 0x80 != 0;
            }
            LDY_ABSX => {
                // LDY $NNNN,X
                let addr = self.absolute_x_addr(memory);
                self.pc += 2;
                self.y = memory.read(addr);
                self.status.z = self.y == 0;
                self.status.n = self.y & 0x80 != 0;
            }
            LDA_ABSX => {
                // LDA $NNNN,X
                let addr = self.absolute_x_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                self.lda(data);
            }
            LDX_ABSY => {
                // LDX $NNNN,Y
                let addr = self.absolute_y_addr(memory);
                self.pc += 2;
                self.x = memory.read(addr);
                self.status.z = self.x == 0;
                self.status.n = self.x & 0x80 != 0;
            }
            CPY_IMM => {
                // CPY #$NN
                let immediate = memory.read(self.pc);
                self.pc += 1;
                let result = self.y.wrapping_sub(immediate);
                self.status.c = self.y >= immediate;
                self.status.z = result == 0;
                self.status.n = result & 0x80 != 0;
            }
            CMP_INDX => {
                // CMP ($NN,X)
                let addr = self.indirect_x_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                self.cmp(data);
            }
            CPY_ZP => {
                // CPY $NN (Zero Page)
                let addr = self.zero_page_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                let result = self.y.wrapping_sub(data);
                self.status.c = self.y >= data;
                self.status.z = result == 0;
                self.status.n = result & 0x80 != 0;
            }
            CMP_ZP => {
                // CMP $NN (Zero Page)
                let addr = self.zero_page_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                self.cmp(data);
            }
            DEC_ZP => {
                // DEC $NN (Zero Page)
                let addr = self.zero_page_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                let result = self.dec(data);
                memory.write(addr, result);
            }
            INY => {
                // INY (Increment Y Register)
                self.y = self.y.wrapping_add(1);
                self.status.z = self.y == 0;
                self.status.n = self.y & 0x80 != 0;
            }
            CMP_IMM => {
                // CMP #$NN
                let immediate = memory.read(self.pc);
                self.pc += 1;
                self.cmp(immediate);
            }
            DEX => {
                // DEX (Decrement X Register)
                self.x = self.x.wrapping_sub(1);
                self.status.z = self.x == 0;
                self.status.n = self.x & 0x80 != 0;
            }
            CPY_ABS => {
                // CPY $NNNN
                let addr = self.absolute_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                let result = self.y.wrapping_sub(data);
                self.status.c = self.y >= data;
                self.status.z = result == 0;
                self.status.n = result & 0x80 != 0;
            }
            CMP_ABS => {
                // CMP $NNNN
                let addr = self.absolute_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                self.cmp(data);
            }
            DEC_ABS => {
                // DEC $NNNN
                let addr = self.absolute_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                let result = self.dec(data);
                memory.write(addr, result);
            }
            BNE => {
                // BNE (Branch on Not Equal)
                let offset = memory.read(self.pc) as i8;
                self.pc += 1;
                if !self.status.z {
                    self.pc = self.pc.wrapping_add(offset as u16);
                }
            }
            CMP_INDY => {
                // CMP ($NN),Y
                let addr = self.indirect_y_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                self.cmp(data);
            }
            CMP_ZPX => {
                // CMP $NN,X (Zero Page,X)
                let addr = self.zero_page_x_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                self.cmp(data);
            }
            DEC_ZPX => {
                // DEC $NN,X (Zero Page,X)
                let addr = self.zero_page_x_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                let result = self.dec(data);
                memory.write(addr, result);
            }
            CLD => {
                // CLD (Clear Decimal Mode)
                self.status.d = false;
            }
            CMP_ABSY => {
                // CMP $NNNN,Y
                let addr = self.absolute_y_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                self.cmp(data);
            }
            CMP_ABSX => {
                // CMP $NNNN,X
                let addr = self.absolute_x_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                self.cmp(data);
            }
            DEC_ABSX => {
                // DEC $NNNN,X
                let addr = self.absolute_x_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                let result = self.dec(data);
                memory.write(addr, result);
            }
            CPX_IMM => {
                // CPX #$NN
                let immediate = memory.read(self.pc);
                self.pc += 1;
                let result = self.x.wrapping_sub(immediate);
                self.status.c = self.x >= immediate;
                self.status.z = result == 0;
                self.status.n = result & 0x80 != 0;
            }
            SBC_INDX => {
                // SBC ($NN,X)
                let addr = self.indirect_x_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                self.sbc(data);
            }
            CPX_ZP => {
                // CPX $NN (Zero Page)
                let addr = self.zero_page_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                let result = self.x.wrapping_sub(data);
                self.status.c = self.x >= data;
                self.status.z = result == 0;
                self.status.n = result & 0x80 != 0;
            }
            SBC_ZP => {
                // SBC $NN (Zero Page)
                let addr = self.zero_page_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                self.sbc(data);
            }
            INC_ZP => {
                // INC $NN (Zero Page)
                let addr = self.zero_page_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                let result = self.inc(data);
                memory.write(addr, result);
            }
            INX => {
                // INX (Increment X Register)
                self.x = self.x.wrapping_add(1);
                self.status.z = self.x == 0;
                self.status.n = self.x & 0x80 != 0;
            }
            SBC_IMM => {
                // SBC #$NN
                let immediate = memory.read(self.pc);
                self.pc += 1;
                self.sbc(immediate);
            }
            NOP => {
                // NOP (No Operation)
            }
            CPX_ABS => {
                // CPX $NNNN
                let addr = self.absolute_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                let result = self.x.wrapping_sub(data);
                self.status.c = self.x >= data;
                self.status.z = result == 0;
                self.status.n = result & 0x80 != 0;
            }
            SBC_ABS => {
                // SBC $NNNN
                let addr = self.absolute_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                self.sbc(data);
            }
            INC_ABS => {
                // INC $NNNN
                let addr = self.absolute_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                let result = self.inc(data);
                memory.write(addr, result);
            }
            BEQ => {
                // BEQ (Branch on Equal)
                let offset = memory.read(self.pc) as i8;
                self.pc += 1;
                if self.status.z {
                    self.pc = self.pc.wrapping_add(offset as u16);
                }
            }
            SBC_INDY => {
                // SBC ($NN),Y
                let addr = self.indirect_y_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                self.sbc(data);
            }
            SBC_ZPX => {
                // SBC $NN,X (Zero Page,X)
                let addr = self.zero_page_x_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                self.sbc(data);
            }
            INC_ZPX => {
                // INC $NN,X (Zero Page,X)
                let addr = self.zero_page_x_addr(memory);
                self.pc += 1;
                let data = memory.read(addr);
                let result = self.inc(data);
                memory.write(addr, result);
            }
            SED => {
                // SED (Set Decimal Mode)
                self.status.d = true;
            }
            SBC_ABSY => {
                // SBC $NNNN,Y
                let addr = self.absolute_y_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                self.sbc(data);
            }
            SBC_ABSX => {
                // SBC $NNNN,X
                let addr = self.absolute_x_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                self.sbc(data);
            }
            INC_ABSX => {
                // INC $NNNN,X
                let addr = self.absolute_x_addr(memory);
                self.pc += 2;
                let data = memory.read(addr);
                let result = self.inc(data);
                memory.write(addr, result);
            }
            LSR_ABS => {
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
