mod cpu;
use cpu::{Cpu, Gameboy};

fn main() {
    let mut gb = Gameboy {
        cpu: Cpu::new(),
        memory: [0; 65536],
    };

    // set up stack pointer (important for call/ret!)
    gb.cpu.sp = 0xFFFE;  // stack starts at top of memory
    gb.cpu.halted = false;  // enable cpu

    // load the "game" into memory
    gb.memory[0x0100] = 0xCD; gb.memory[0x0101] = 0x10; gb.memory[0x0102] = 0x01; // call 0x0110
    gb.memory[0x0103] = 0x76; // halt

    // function at 0x0110:
    gb.memory[0x0110] = 0x3E; gb.memory[0x0111] = 0x42; // ld a,0x42
    gb.memory[0x0112] = 0xC9; // ret

    // start execution
    gb.cpu.pc = 0x0100;

    // run the program
    gb.step(); // call 0x0110
    println!("after call: pc = {:04X}, sp = {:04X}", gb.cpu.pc, gb.cpu.sp);
    
    gb.step(); // ld a,0x42
    println!("after load: a = {:02X}", gb.cpu.a);
    
    gb.step(); // ret
    println!("after ret: pc = {:04X}, sp = {:04X}", gb.cpu.pc, gb.cpu.sp);
    
    gb.step(); // halt
    println!("final: a = {:02X}, halted = {}", gb.cpu.a, gb.cpu.halted);
}