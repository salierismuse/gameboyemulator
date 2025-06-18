mod cpu;
use cpu::{Cpu, Gameboy};
use std::fs::read;

fn main() {
    let mut gameboy = Gameboy {
        cpu: Cpu::new(),
        memory: [0; 65536],
    };
    
    gameboy.cpu.pc = 0x0000;  
    gameboy.cpu.halted = false;  
    

    let rom_data = std::fs::read(r"").expect("failed to read rom file");
    gameboy.load_rom(&rom_data);

    // run a few steps
    while true {
        println!("pc: {:04X}, a: {:02X}", gameboy.cpu.pc, gameboy.cpu.a);
        gameboy.step();
    }
}