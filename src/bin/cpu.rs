use std::env;
use std::fs;
use std::process;

use mos6502r::cpu::CPU;
use mos6502r::memory::Memory;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut step_mode = false;
    let mut rom_path: Option<&str> = None;

    for arg in &args[1..] {
        match arg.as_str() {
            "--step" => step_mode = true,
            other => rom_path = Some(other),
        }
    }

    let rom_path = match rom_path {
        Some(p) => p,
        None => {
            eprintln!("Usage: {} [--step] <rom.bin>", args[0]);
            process::exit(1);
        }
    };

    let rom = match fs::read(rom_path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error: {}: {}", rom_path, e);
            process::exit(1);
        }
    };

    if rom.is_empty() || rom.len() > 0x8000 {
        eprintln!("Error: ROM must be 1–32768 bytes (got {})", rom.len());
        process::exit(1);
    }

    let mut mem = Memory::new();

    // ROMをメモリ末尾に配置（$10000 - rom.len() 〜 $FFFF）
    let load_addr = (0x10000 - rom.len()) as u16;
    mem.load(load_addr, &rom);

    // リセットベクタ ($FFFC/$FFFD) から開始アドレスを取得
    let pc = (mem.read(0xFFFC) as u16) | ((mem.read(0xFFFD) as u16) << 8);

    let mut cpu = CPU::new();
    cpu.set_pc(pc);

    println!("ROM:   {} ({} bytes, loaded at ${:04X})", rom_path, rom.len(), load_addr);
    println!("Start: ${:04X}", pc);
    println!("---");

    if step_mode {
        let mut step = 0u64;
        while !cpu.is_halted() {
            let pc_before = cpu.pc();
            cpu.step(&mut mem);
            println!(
                "[{:>6}] PC=${:04X}  A=${:02X} X=${:02X} Y=${:02X} SP=${:02X}  N={} V={} Z={} C={}",
                step,
                pc_before,
                cpu.a(), cpu.x(), cpu.y(), cpu.sp(),
                cpu.negative() as u8, cpu.overflow() as u8,
                cpu.zero() as u8, cpu.carry() as u8,
            );
            step += 1;
        }
    } else {
        cpu.run(&mut mem);
    }

    println!("---");
    println!("Halted. ({} instructions)", if step_mode { "see above" } else { "?" });
    println!("A=${:02X}  X=${:02X}  Y=${:02X}  PC=${:04X}  SP=${:02X}",
             cpu.a(), cpu.x(), cpu.y(), cpu.pc(), cpu.sp());
    println!("N={}  V={}  Z={}  C={}",
             cpu.negative() as u8, cpu.overflow() as u8,
             cpu.zero() as u8, cpu.carry() as u8);
}