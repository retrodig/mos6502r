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
use mos6502r::opcodes::*;

fn main() {
    let mut cpu = CPU::new();
    let mut mem = Memory::new();

    #[rustfmt::skip]
    let program: &[u8] = &[
        LDA_IMM, 0x00, STA_ZP, 0x00, STA_ZP, 0x10,  // prev=0, output[0]=0
        LDA_IMM, 0x01, STA_ZP, 0x01, STA_ZP, 0x11,  // curr=1, output[1]=1
        LDY_IMM, 0x02,                                // Y = 2
        LDX_IMM, 0x08,                                // X = 8 (残り8項)
        CLC,                                          //        ← loop
        LDA_ZP, 0x00, ADC_ZP, 0x01, STA_ZP, 0x02,   // next = prev + curr
        LDA_ZP, 0x01, STA_ZP, 0x00,                  // prev = curr
        LDA_ZP, 0x02, STA_ZP, 0x01,                  // curr = next
        STA_ABSY, 0x10, 0x00,                         // output[Y] = curr
        INY,
        DEX,
        BNE, 0xEA,                                    // BNE loop
        BRK,
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