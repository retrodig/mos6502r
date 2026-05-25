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
use mos6502r::opcodes::*;

fn main() {
    let mut cpu = CPU::new();
    let mut mem = Memory::new();

    #[rustfmt::skip]
    let program: &[u8] = &[
        LDX_IMM, 0x0A,           // LDX #$0A
        LDY_IMM, 0x00,           // LDY #$00
        TXA,                     //            ← loop
        STA_ABSY, 0x00, 0x03,    // STA $0300,Y
        INY,
        DEX,
        BNE, 0xF8,               // BNE loop
        BRK,
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