mod cpu;
use cpu::{Cpu, Gameboy};

fn main() {
    let mut gameboy = Gameboy {
        cpu: Cpu::new(),
        memory: [0; 65536],
    };

    gameboy.memory[0x0100] = 0x3E; // ld a,0x42
    gameboy.memory[0x0101] = 0x42;
    gameboy.memory[0x0102] = 0x00; // nop
    
    // run a few steps
    for _ in 0..10 {
        println!("pc: {:04X}, a: {:02X}", gameboy.cpu.pc, gameboy.cpu.a);
        gameboy.step();
    }

}