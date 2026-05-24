// $0010〜$0017 の8バイトを合計し、結果を $0000 に格納する例。
//
// 6502 プログラム:
//   LDA #$00 : LDX #8 : LDY #0
// loop:
//   CLC : ADC $0010,Y
//   INY : DEX : BNE loop
//   STA $00
//   BRK

use mos6502r::cpu::CPU;
use mos6502r::memory::Memory;

fn main() {
    let mut cpu = CPU::new();
    let mut mem = Memory::new();

    // チェックサムをとるデータ
    let data: &[u8] = &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
    mem.load(0x0010, data);

    #[rustfmt::skip]
    let program: &[u8] = &[
        0xA9, 0x00,              // LDA #$00
        0xA2, 0x08,              // LDX #8
        0xA0, 0x00,              // LDY #0
        0x18,                    // CLC          ← loop
        0x79, 0x10, 0x00,        // ADC $0010,Y
        0xC8,                    // INY
        0xCA,                    // DEX
        0xD0, 0xF8,              // BNE loop
        0x85, 0x00,              // STA $00
        0x00,                    // BRK
    ];

    mem.load(0x0200, program);
    cpu.set_pc(0x0200);
    cpu.run(&mut mem);

    let expected: u8 = data.iter().fold(0u8, |acc, &b| acc.wrapping_add(b));
    let result = mem.read(0x0000);
    println!("data:     {:?}", data);
    println!("checksum: ${:02X} ({})", result, result);
    assert_eq!(result, expected, "checksum mismatch");
    println!("ok");
}