// フィボナッチ数列の最初の10項を計算し、$0010〜$0019 に格納する例。
//
// 6502 プログラム:
//   $00 = prev, $01 = curr, $02 = next (ゼロページ作業領域)
//   $0010/$0011 に 0, 1 を初期値として書き込み
//   Y = 2 (出力インデックス), X = 8 (残りループ回数)
// loop:
//   CLC
//   LDA $00 : ADC $01 → $02 (next = prev + curr)
//   LDA $01 → $00   (prev = curr)
//   LDA $02 → $01   (curr = next)
//   STA $0010,Y
//   INY : DEX : BNE loop
//   BRK

use mos6502r::cpu::CPU;
use mos6502r::memory::Memory;

fn main() {
    let mut cpu = CPU::new();
    let mut mem = Memory::new();

    #[rustfmt::skip]
    let program: &[u8] = &[
        0xA9, 0x00, 0x85, 0x00, 0x85, 0x10,  // LDA #0; STA $00; STA $10
        0xA9, 0x01, 0x85, 0x01, 0x85, 0x11,  // LDA #1; STA $01; STA $11
        0xA0, 0x02,                            // LDY #2
        0xA2, 0x08,                            // LDX #8
        0x18,                                  // CLC          ← loop
        0xA5, 0x00, 0x65, 0x01, 0x85, 0x02,  // LDA $00; ADC $01; STA $02
        0xA5, 0x01, 0x85, 0x00,               // LDA $01; STA $00
        0xA5, 0x02, 0x85, 0x01,               // LDA $02; STA $01
        0x99, 0x10, 0x00,                      // STA $0010,Y
        0xC8,                                  // INY
        0xCA,                                  // DEX
        0xD0, 0xEA,                            // BNE loop
        0x00,                                  // BRK
    ];

    mem.load(0x0200, program);
    cpu.set_pc(0x0200);
    cpu.run(&mut mem);

    print!("fibonacci:");
    for i in 0..10u16 {
        print!(" {}", mem.read(0x0010 + i));
    }
    println!();
}