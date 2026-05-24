use mos6502r::cpu::CPU;
use mos6502r::memory::Memory;

fn setup(start: u16, program: &[u8]) -> (CPU, Memory) {
    let mut cpu = CPU::new();
    let mut mem = Memory::new();
    mem.load(start, program);
    cpu.set_pc(start);
    (cpu, mem)
}

// ─── LDA ───────────────────────────────────────────────────────────────────

#[test]
fn lda_immediate_sets_a() {
    let (mut cpu, mut mem) = setup(0x0200, &[0xA9, 0x42, 0x00]);
    cpu.run(&mut mem);
    assert_eq!(cpu.a(), 0x42);
}

#[test]
fn lda_immediate_sets_zero_flag() {
    let (mut cpu, mut mem) = setup(0x0200, &[0xA9, 0x00, 0x00]);
    cpu.run(&mut mem);
    assert!(cpu.zero());
    assert!(!cpu.negative());
}

#[test]
fn lda_immediate_sets_negative_flag() {
    let (mut cpu, mut mem) = setup(0x0200, &[0xA9, 0x80, 0x00]);
    cpu.run(&mut mem);
    assert!(cpu.negative());
    assert!(!cpu.zero());
}

#[test]
fn lda_zero_page() {
    let (mut cpu, mut mem) = setup(0x0200, &[0xA5, 0x10, 0x00]);
    mem.write(0x0010, 0x55);
    cpu.run(&mut mem);
    assert_eq!(cpu.a(), 0x55);
}

#[test]
fn lda_absolute() {
    let (mut cpu, mut mem) = setup(0x0200, &[0xAD, 0x00, 0x03, 0x00]);
    mem.write(0x0300, 0xAB);
    cpu.run(&mut mem);
    assert_eq!(cpu.a(), 0xAB);
}

// ─── LDX / LDY ─────────────────────────────────────────────────────────────

#[test]
fn ldx_immediate() {
    let (mut cpu, mut mem) = setup(0x0200, &[0xA2, 0x07, 0x00]);
    cpu.run(&mut mem);
    assert_eq!(cpu.x(), 0x07);
}

#[test]
fn ldy_immediate() {
    let (mut cpu, mut mem) = setup(0x0200, &[0xA0, 0x03, 0x00]);
    cpu.run(&mut mem);
    assert_eq!(cpu.y(), 0x03);
}

// ─── STA / STX / STY ───────────────────────────────────────────────────────

#[test]
fn sta_zero_page() {
    // LDA #$FF, STA $20, BRK
    let (mut cpu, mut mem) = setup(0x0200, &[0xA9, 0xFF, 0x85, 0x20, 0x00]);
    cpu.run(&mut mem);
    assert_eq!(mem.read(0x0020), 0xFF);
}

#[test]
fn sta_absolute() {
    // LDA #$AB, STA $0300, BRK
    let (mut cpu, mut mem) = setup(0x0200, &[0xA9, 0xAB, 0x8D, 0x00, 0x03, 0x00]);
    cpu.run(&mut mem);
    assert_eq!(mem.read(0x0300), 0xAB);
}

// ─── Transfers ─────────────────────────────────────────────────────────────

#[test]
fn tax_transfers_a_to_x() {
    // LDA #$42, TAX, BRK
    let (mut cpu, mut mem) = setup(0x0200, &[0xA9, 0x42, 0xAA, 0x00]);
    cpu.run(&mut mem);
    assert_eq!(cpu.x(), 0x42);
}

#[test]
fn tay_transfers_a_to_y() {
    // LDA #$42, TAY, BRK
    let (mut cpu, mut mem) = setup(0x0200, &[0xA9, 0x42, 0xA8, 0x00]);
    cpu.run(&mut mem);
    assert_eq!(cpu.y(), 0x42);
}

#[test]
fn txa_transfers_x_to_a() {
    // LDX #$10, TXA, BRK
    let (mut cpu, mut mem) = setup(0x0200, &[0xA2, 0x10, 0x8A, 0x00]);
    cpu.run(&mut mem);
    assert_eq!(cpu.a(), 0x10);
}

// ─── ADC ───────────────────────────────────────────────────────────────────

#[test]
fn adc_no_carry() {
    // LDA #$10, ADC #$20, BRK
    let (mut cpu, mut mem) = setup(0x0200, &[0xA9, 0x10, 0x69, 0x20, 0x00]);
    cpu.run(&mut mem);
    assert_eq!(cpu.a(), 0x30);
    assert!(!cpu.carry());
}

#[test]
fn adc_produces_carry() {
    // LDA #$FF, ADC #$01, BRK
    let (mut cpu, mut mem) = setup(0x0200, &[0xA9, 0xFF, 0x69, 0x01, 0x00]);
    cpu.run(&mut mem);
    assert_eq!(cpu.a(), 0x00);
    assert!(cpu.carry());
    assert!(cpu.zero());
}

#[test]
fn adc_signed_overflow() {
    // 0x50 + 0x50 = 0xA0 → signed overflow (positive + positive = negative)
    // LDA #$50, ADC #$50, BRK
    let (mut cpu, mut mem) = setup(0x0200, &[0xA9, 0x50, 0x69, 0x50, 0x00]);
    cpu.run(&mut mem);
    assert_eq!(cpu.a(), 0xA0);
    assert!(cpu.overflow());
    assert!(cpu.negative());
}

// ─── SBC ───────────────────────────────────────────────────────────────────

#[test]
fn sbc_no_borrow() {
    // LDA #$50, SEC, SBC #$10, BRK  (C=1 means no borrow)
    let (mut cpu, mut mem) = setup(0x0200, &[0xA9, 0x50, 0x38, 0xE9, 0x10, 0x00]);
    cpu.run(&mut mem);
    assert_eq!(cpu.a(), 0x40);
    assert!(cpu.carry()); // no borrow
}

#[test]
fn sbc_with_borrow() {
    // LDA #$10, CLC, SBC #$01, BRK  (C=0 means borrow-in of 1)
    let (mut cpu, mut mem) = setup(0x0200, &[0xA9, 0x10, 0x18, 0xE9, 0x01, 0x00]);
    cpu.run(&mut mem);
    assert_eq!(cpu.a(), 0x0E); // 0x10 - 0x01 - 1 = 0x0E
}

#[test]
fn sbc_signed_overflow() {
    // 0xD0 (-48) - 0x70 (112) = -160 → overflows signed byte
    // LDA #$D0, SEC, SBC #$70, BRK
    let (mut cpu, mut mem) = setup(0x0200, &[0xA9, 0xD0, 0x38, 0xE9, 0x70, 0x00]);
    cpu.run(&mut mem);
    assert!(cpu.overflow());
}

// ─── CMP ───────────────────────────────────────────────────────────────────

#[test]
fn cmp_equal_sets_zero_and_carry() {
    // LDA #$42, CMP #$42, BRK
    let (mut cpu, mut mem) = setup(0x0200, &[0xA9, 0x42, 0xC9, 0x42, 0x00]);
    cpu.run(&mut mem);
    assert!(cpu.zero());
    assert!(cpu.carry());
    assert!(!cpu.negative());
}

#[test]
fn cmp_greater_sets_carry() {
    // LDA #$50, CMP #$20, BRK
    let (mut cpu, mut mem) = setup(0x0200, &[0xA9, 0x50, 0xC9, 0x20, 0x00]);
    cpu.run(&mut mem);
    assert!(!cpu.zero());
    assert!(cpu.carry());
}

#[test]
fn cmp_less_clears_carry() {
    // LDA #$10, CMP #$20, BRK
    let (mut cpu, mut mem) = setup(0x0200, &[0xA9, 0x10, 0xC9, 0x20, 0x00]);
    cpu.run(&mut mem);
    assert!(!cpu.carry());
    assert!(cpu.negative());
}

// ─── Branches ──────────────────────────────────────────────────────────────

#[test]
fn beq_branches_when_zero() {
    // LDA #$00 → Z=1, BEQ +1 (skip NOP), LDA #$FF, BRK
    // skip the LDA #$FF by branching over it (2 bytes)
    let (mut cpu, mut mem) = setup(0x0200, &[
        0xA9, 0x00, // LDA #$00
        0xF0, 0x02, // BEQ +2  → jumps over next LDA
        0xA9, 0xFF, // LDA #$FF  (skipped)
        0x00,       // BRK
    ]);
    cpu.run(&mut mem);
    assert_eq!(cpu.a(), 0x00);
}

#[test]
fn bne_branches_when_not_zero() {
    // LDA #$01 → Z=0, BNE +2 (skip LDA #$FF), BRK
    let (mut cpu, mut mem) = setup(0x0200, &[
        0xA9, 0x01, // LDA #$01
        0xD0, 0x02, // BNE +2
        0xA9, 0xFF, // LDA #$FF  (skipped)
        0x00,       // BRK
    ]);
    cpu.run(&mut mem);
    assert_eq!(cpu.a(), 0x01);
}

// ─── Stack ─────────────────────────────────────────────────────────────────

#[test]
fn pha_pla_roundtrip() {
    // LDA #$AB, PHA, LDA #$00, PLA, BRK
    let (mut cpu, mut mem) = setup(0x0200, &[0xA9, 0xAB, 0x48, 0xA9, 0x00, 0x68, 0x00]);
    cpu.run(&mut mem);
    assert_eq!(cpu.a(), 0xAB);
}

#[test]
fn pha_decrements_sp() {
    let (mut cpu, mut mem) = setup(0x0200, &[0xA9, 0x01, 0x48, 0x00]);
    cpu.step(&mut mem); // LDA #$01
    let sp_before = cpu.sp();
    cpu.step(&mut mem); // PHA
    assert_eq!(cpu.sp(), sp_before.wrapping_sub(1));
}

// ─── JSR / RTS ─────────────────────────────────────────────────────────────

#[test]
fn jsr_rts_returns_correctly() {
    // 0x0200: JSR $0210
    // 0x0203: LDA #$BB
    // 0x0205: BRK
    // 0x0210: LDA #$AA
    // 0x0212: RTS
    let mut mem = Memory::new();
    mem.load(0x0200, &[0x20, 0x10, 0x02]); // JSR $0210
    mem.load(0x0203, &[0xA9, 0xBB, 0x00]); // LDA #$BB, BRK
    mem.load(0x0210, &[0xA9, 0xAA, 0x60]); // LDA #$AA, RTS
    let mut cpu = CPU::new();
    cpu.set_pc(0x0200);
    cpu.run(&mut mem);
    assert_eq!(cpu.a(), 0xBB); // subroutine ran, then returned, then LDA #$BB
}

// ─── Bit shifts ────────────────────────────────────────────────────────────

#[test]
fn asl_accumulator() {
    // LDA #$41, ASL A, BRK
    let (mut cpu, mut mem) = setup(0x0200, &[0xA9, 0x41, 0x0A, 0x00]);
    cpu.run(&mut mem);
    assert_eq!(cpu.a(), 0x82);
    assert!(!cpu.carry());
}

#[test]
fn asl_sets_carry_from_bit7() {
    // LDA #$80, ASL A, BRK
    let (mut cpu, mut mem) = setup(0x0200, &[0xA9, 0x80, 0x0A, 0x00]);
    cpu.run(&mut mem);
    assert_eq!(cpu.a(), 0x00);
    assert!(cpu.carry());
    assert!(cpu.zero());
}

#[test]
fn lsr_accumulator() {
    // LDA #$82, LSR A, BRK
    let (mut cpu, mut mem) = setup(0x0200, &[0xA9, 0x82, 0x4A, 0x00]);
    cpu.run(&mut mem);
    assert_eq!(cpu.a(), 0x41);
    assert!(!cpu.carry());
}

// ─── BRK halts ─────────────────────────────────────────────────────────────

#[test]
fn brk_halts_cpu() {
    let (mut cpu, mut mem) = setup(0x0200, &[0x00]);
    cpu.step(&mut mem);
    assert!(cpu.is_halted());
}

#[test]
fn step_does_nothing_when_halted() {
    let (mut cpu, mut mem) = setup(0x0200, &[0x00, 0xA9, 0xFF]);
    cpu.run(&mut mem); // halts at BRK
    cpu.step(&mut mem); // should not execute LDA #$FF
    assert_eq!(cpu.a(), 0x00);
}