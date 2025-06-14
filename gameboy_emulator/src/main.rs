mod cpu;
use cpu::{Cpu, Gameboy};

fn main() {
    let mut gb = Gameboy {
        cpu: Cpu::new(),
        memory: [0; 65536],
    };

        // load the "game" into memory
    gb.memory[0x0100] = 0x3E; gb.memory[0x0101] = 0xFF;
    gb.memory[0x0102] = 0xFA; gb.memory[0x0103] = 0x00; gb.memory[0x0104] = 0x80;
    gb.memory[0x0105] = 0x3E; gb.memory[0x0106] = 0x00;
    gb.memory[0x0107] = 0xC3; gb.memory[0x0108] = 0x00; gb.memory[0x0109] = 0x01;

    // start execution at gameboy boot address
    gb.cpu.pc = 0x0100;

    // step 1: LD A, 0xFF

    gb.step(); // your cpu loads 0xFF into register A
    println!("A = {:02X}", gb.cpu.a); // outputs: A = FF

    // step 2: LD A, (0x8000) - load from video memory
    gb.step(); // your cpu reads from memory[0x8000] into A
    println!("A = {:02X}", gb.cpu.a); // outputs: A = 00 (vram starts empty)

    // step 3: LD A, 0x00  
    gb.step(); // loads 0x00 into A
    println!("A = {:02X}", gb.cpu.a); // outputs: A = 00

    // step 4: JP 0x0100 - infinite loop!
    gb.step(); // jumps back to start
    println!("PC = {:04X}", gb.cpu.pc); // outputs: PC = 0100
}

