// LDA
pub const LDA_INDX: u8 = 0xA1;
pub const LDA_ZP:   u8 = 0xA5;
pub const LDA_IMM:  u8 = 0xA9;
pub const LDA_ABS:  u8 = 0xAD;
pub const LDA_INDY: u8 = 0xB1;
pub const LDA_ZPX:  u8 = 0xB5;
pub const LDA_ABSY: u8 = 0xB9;
pub const LDA_ABSX: u8 = 0xBD;

// LDX
pub const LDX_IMM:  u8 = 0xA2;
pub const LDX_ZP:   u8 = 0xA6;
pub const LDX_ABS:  u8 = 0xAE;
pub const LDX_ZPY:  u8 = 0xB6;
pub const LDX_ABSY: u8 = 0xBE;

// LDY
pub const LDY_IMM:  u8 = 0xA0;
pub const LDY_ZP:   u8 = 0xA4;
pub const LDY_ABS:  u8 = 0xAC;
pub const LDY_ZPX:  u8 = 0xB4;
pub const LDY_ABSX: u8 = 0xBC;

// STA
pub const STA_INDX: u8 = 0x81;
pub const STA_ZP:   u8 = 0x85;
pub const STA_ABS:  u8 = 0x8D;
pub const STA_INDY: u8 = 0x91;
pub const STA_ZPX:  u8 = 0x95;
pub const STA_ABSY: u8 = 0x99;
pub const STA_ABSX: u8 = 0x9D;

// STX
pub const STX_ZP:   u8 = 0x86;
pub const STX_ABS:  u8 = 0x8E;
pub const STX_ZPY:  u8 = 0x96;

// STY
pub const STY_ZP:   u8 = 0x84;
pub const STY_ABS:  u8 = 0x8C;
pub const STY_ZPX:  u8 = 0x94;

// Transfer
pub const TAX: u8 = 0xAA;
pub const TAY: u8 = 0xA8;
pub const TXA: u8 = 0x8A;
pub const TYA: u8 = 0x98;
pub const TSX: u8 = 0xBA;
pub const TXS: u8 = 0x9A;

// ADC
pub const ADC_INDX: u8 = 0x61;
pub const ADC_ZP:   u8 = 0x65;
pub const ADC_IMM:  u8 = 0x69;
pub const ADC_ABS:  u8 = 0x6D;
pub const ADC_INDY: u8 = 0x71;
pub const ADC_ZPX:  u8 = 0x75;
pub const ADC_ABSY: u8 = 0x79;
pub const ADC_ABSX: u8 = 0x7D;

// SBC
pub const SBC_INDX: u8 = 0xE1;
pub const SBC_ZP:   u8 = 0xE5;
pub const SBC_IMM:  u8 = 0xE9;
pub const SBC_ABS:  u8 = 0xED;
pub const SBC_INDY: u8 = 0xF1;
pub const SBC_ZPX:  u8 = 0xF5;
pub const SBC_ABSY: u8 = 0xF9;
pub const SBC_ABSX: u8 = 0xFD;

// AND
pub const AND_INDX: u8 = 0x21;
pub const AND_ZP:   u8 = 0x25;
pub const AND_IMM:  u8 = 0x29;
pub const AND_ABS:  u8 = 0x2D;
pub const AND_INDY: u8 = 0x31;
pub const AND_ZPX:  u8 = 0x35;
pub const AND_ABSY: u8 = 0x39;
pub const AND_ABSX: u8 = 0x3D;

// ORA
pub const ORA_INDX: u8 = 0x01;
pub const ORA_ZP:   u8 = 0x05;
pub const ORA_IMM:  u8 = 0x09;
pub const ORA_ABS:  u8 = 0x0D;
pub const ORA_INDY: u8 = 0x11;
pub const ORA_ZPX:  u8 = 0x15;
pub const ORA_ABSY: u8 = 0x19;
pub const ORA_ABSX: u8 = 0x1D;

// EOR
pub const EOR_INDX: u8 = 0x41;
pub const EOR_ZP:   u8 = 0x45;
pub const EOR_IMM:  u8 = 0x49;
pub const EOR_ABS:  u8 = 0x4D;
pub const EOR_INDY: u8 = 0x51;
pub const EOR_ZPX:  u8 = 0x55;
pub const EOR_ABSY: u8 = 0x59;
pub const EOR_ABSX: u8 = 0x5D;

// BIT
pub const BIT_ZP:   u8 = 0x24;
pub const BIT_ABS:  u8 = 0x2C;
pub const BIT_IMM:  u8 = 0x89;

// ASL
pub const ASL_ZP:   u8 = 0x06;
pub const ASL_ACC:  u8 = 0x0A;
pub const ASL_ABS:  u8 = 0x0E;
pub const ASL_ZPX:  u8 = 0x16;
pub const ASL_ABSX: u8 = 0x1E;

// LSR
pub const LSR_ZP:   u8 = 0x46;
pub const LSR_ACC:  u8 = 0x4A;
pub const LSR_ABS:  u8 = 0x4E;
pub const LSR_ZPX:  u8 = 0x56;
pub const LSR_ABSX: u8 = 0x5E;

// ROL
pub const ROL_ZP:   u8 = 0x26;
pub const ROL_ACC:  u8 = 0x2A;
pub const ROL_ABS:  u8 = 0x2E;
pub const ROL_ZPX:  u8 = 0x36;
pub const ROL_ABSX: u8 = 0x3E;

// ROR
pub const ROR_ZP:   u8 = 0x66;
pub const ROR_ACC:  u8 = 0x6A;
pub const ROR_ABS:  u8 = 0x6E;
pub const ROR_ZPX:  u8 = 0x76;
pub const ROR_ABSX: u8 = 0x7E;

// CMP
pub const CMP_INDX: u8 = 0xC1;
pub const CMP_ZP:   u8 = 0xC5;
pub const CMP_IMM:  u8 = 0xC9;
pub const CMP_ABS:  u8 = 0xCD;
pub const CMP_INDY: u8 = 0xD1;
pub const CMP_ZPX:  u8 = 0xD5;
pub const CMP_ABSY: u8 = 0xD9;
pub const CMP_ABSX: u8 = 0xDD;

// CPX
pub const CPX_IMM:  u8 = 0xE0;
pub const CPX_ZP:   u8 = 0xE4;
pub const CPX_ABS:  u8 = 0xEC;

// CPY
pub const CPY_IMM:  u8 = 0xC0;
pub const CPY_ZP:   u8 = 0xC4;
pub const CPY_ABS:  u8 = 0xCC;

// INC / DEC (memory)
pub const INC_ZP:   u8 = 0xE6;
pub const INC_ABS:  u8 = 0xEE;
pub const INC_ZPX:  u8 = 0xF6;
pub const INC_ABSX: u8 = 0xFE;
pub const DEC_ZP:   u8 = 0xC6;
pub const DEC_ABS:  u8 = 0xCE;
pub const DEC_ZPX:  u8 = 0xD6;
pub const DEC_ABSX: u8 = 0xDE;

// INC / DEC (register)
pub const INX: u8 = 0xE8;
pub const INY: u8 = 0xC8;
pub const DEX: u8 = 0xCA;
pub const DEY: u8 = 0x88;

// Jump / subroutine
pub const JMP_ABS: u8 = 0x4C;
pub const JMP_IND: u8 = 0x6C;
pub const JSR:     u8 = 0x20;
pub const RTS:     u8 = 0x60;
pub const RTI:     u8 = 0x40;

// Branch
pub const BCC: u8 = 0x90;
pub const BCS: u8 = 0xB0;
pub const BEQ: u8 = 0xF0;
pub const BMI: u8 = 0x30;
pub const BNE: u8 = 0xD0;
pub const BPL: u8 = 0x10;
pub const BVC: u8 = 0x50;
pub const BVS: u8 = 0x70;
pub const BRA: u8 = 0x80;

// Stack
pub const PHA: u8 = 0x48;
pub const PLA: u8 = 0x68;
pub const PHP: u8 = 0x08;
pub const PLP: u8 = 0x28;

// Flag
pub const CLC: u8 = 0x18;
pub const SEC: u8 = 0x38;
pub const CLI: u8 = 0x58;
pub const SEI: u8 = 0x78;
pub const CLD: u8 = 0xD8;
pub const SED: u8 = 0xF8;
pub const CLV: u8 = 0xB8;

// Misc
pub const NOP: u8 = 0xEA;
pub const BRK: u8 = 0x00;