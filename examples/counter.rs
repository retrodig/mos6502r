// 10 から 1 までカウントダウンし、結果を $0300 以降に格納する例。
//
// 6502 プログラム:
//   LDX #$0A      ; カウンタ = 10
//   LDY #$00      ; メモリインデックス
// loop:
//   TXA           ; A = X
//   STA $0300,Y   ; メモリに書き込み
//   INY           ; インデックス先送り（DEX より先に置くことで Z フラグを保護）
//   DEX           ; BNE はこの Z を見る
//   BNE loop
//   BRK

use mos6502r::cpu::CPU;
use mos6502r::memory::Memory;

fn main() {
    let mut cpu = CPU::new();
    let mut mem = Memory::new();

    #[rustfmt::skip]
    let program: &[u8] = &[
        0xA2, 0x0A,              // LDX #$0A
        0xA0, 0x00,              // LDY #$00
        0x8A,                    // TXA        ← loop
        0x99, 0x00, 0x03,        // STA $0300,Y
        0xC8,                    // INY
        0xCA,                    // DEX
        0xD0, 0xF8,              // BNE loop  (Z from DEX)
        0x00,                    // BRK
    ];

    mem.load(0x0200, program);
    cpu.set_pc(0x0200);
    cpu.run(&mut mem);

    print!("countdown:");
    for i in 0..10u16 {
        print!(" {}", mem.read(0x0300 + i));
    }
    println!();
}