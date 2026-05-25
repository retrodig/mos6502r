# mos6502r

MOS 6502 エミュレータ（Rust 実装）。

## ビルド

```sh
cargo build
cargo test
```

## ライブラリとして使う

```rust
use mos6502r::cpu::CPU;
use mos6502r::memory::Memory;
use mos6502r::opcodes::*;  // オペコード定数

let mut cpu = CPU::new();
let mut mem = Memory::new();

// プログラムをメモリにロード
mem.load(0x0200, &[
    LDA_IMM, 0x42,  // LDA #$42
    STA_ZP,  0x00,  // STA $00
    BRK,            // BRK
]);
cpu.set_pc(0x0200);

// 全命令を BRK まで実行
cpu.run(&mut mem);

// または 1 命令ずつ実行
cpu.step(&mut mem);
```

### opcodes

`mos6502r::opcodes` に全オペコードの定数が定義されている。命名規則は `命令_アドレッシングモード`。

| サフィックス | アドレッシングモード | 例 |
|---|---|---|
| `_IMM` | イミディエート `#$NN` | `LDA_IMM` |
| `_ZP` | ゼロページ `$NN` | `STA_ZP` |
| `_ZPX` / `_ZPY` | ゼロページ indexed | `LDY_ZPX` |
| `_ABS` | アブソリュート `$NNNN` | `JMP_ABS` |
| `_ABSX` / `_ABSY` | アブソリュート indexed | `STA_ABSY` |
| `_INDX` / `_INDY` | インダイレクト | `LDA_INDX` |
| `_ACC` | アキュムレータ | `ASL_ACC` |
| なし | インプライド・レラティブ | `TAX`, `BNE`, `BRK` |

### CPU API

| メソッド | 説明 |
|----------|------|
| `CPU::new()` | CPU を初期化（PC=0, SP=0xFF） |
| `set_pc(addr)` | プログラムカウンタを設定 |
| `run(&mut mem)` | BRK に当たるまで全命令実行 |
| `step(&mut mem)` | 1命令だけ実行 |
| `is_halted()` | BRK で停止中か |
| `a()` / `x()` / `y()` | アキュムレータ・インデックスレジスタ |
| `pc()` / `sp()` | プログラムカウンタ・スタックポインタ |
| `carry()` / `zero()` / `negative()` / `overflow()` | ステータスフラグ |

### Memory API

| メソッド | 説明 |
|----------|------|
| `Memory::new()` | 64KB ゼロ初期化メモリを作成 |
| `read(addr)` | 1バイト読み取り |
| `write(addr, data)` | 1バイト書き込み |
| `load(start, &[u8])` | バイト列を連続して書き込み |

## ROM ローダー（バイナリ）

バイナリファイルをメモリ末尾に配置し、リセットベクタ（`$FFFC`/`$FFFD`）から実行を開始する。

```sh
cargo run --bin cpu -- <rom.bin>
cargo run --bin cpu -- --step <rom.bin>
```

- ROM サイズは 1〜32768 バイト（メモリ末尾に配置）
- `--step` を付けると 1命令ずつ実行し、各ステップのレジスタ・フラグ状態を表示する

```
ROM:   game.rom (16384 bytes, loaded at $C000)
Start: $C000
---
[     0] PC=$C000  A=$00 X=$00 Y=$00 SP=$FF  N=0 V=0 Z=0 C=0
[     1] PC=$C002  A=$42 X=$00 Y=$00 SP=$FF  N=0 V=0 Z=0 C=0
...
---
Halted.
A=$42  X=$00  Y=$00  PC=$0000  SP=$FC
N=0  V=0  Z=0  C=0
```

## サンプル

### counter — カウントダウン

10 から 1 までカウントダウンし、結果を `$0300`〜`$0309` に格納する。

```sh
cargo run --example counter
# countdown: 10 9 8 7 6 5 4 3 2 1
```

### fibonacci — フィボナッチ数列

最初の 10 項を計算し、`$0010`〜`$0019` に格納する。

```sh
cargo run --example fibonacci
# fibonacci: 0 1 1 2 3 5 8 13 21 34
```

### checksum — チェックサム

`$0010`〜`$0017` の 8 バイトを合計し、結果を `$0000` に格納する。

```sh
cargo run --example checksum
# data:     [1, 2, 3, 4, 5, 6, 7, 8]
# checksum: $24 (36)
# ok
```

## テスト

```sh
cargo test
```

`tests/cpu_test.rs` に統合テストが 31 件あり、主要な命令・アドレッシングモード・フラグ動作を網羅している。

## 実装済み命令

| グループ | 命令 |
|----------|------|
| ロード・ストア | LDA, LDX, LDY, STA, STX, STY |
| 転送 | TAX, TAY, TXA, TYA, TSX, TXS |
| 算術 | ADC, SBC |
| 論理 | AND, ORA, EOR, BIT |
| シフト・ローテート | ASL, LSR, ROL, ROR |
| 比較 | CMP, CPX, CPY |
| インクリメント | INC, INX, INY, DEC, DEX, DEY |
| ジャンプ | JMP, JSR, RTS, RTI |
| 分岐 | BCC, BCS, BEQ, BMI, BNE, BPL, BVC, BVS, BRA |
| スタック | PHA, PLA, PHP, PLP |
| フラグ操作 | CLC, SEC, CLI, SEI, CLD, SED, CLV |
| その他 | NOP, BRK |