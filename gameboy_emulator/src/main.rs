mod cpu;
use cpu::{Cpu, Gameboy};

fn main() {
    let mut gb = Gameboy {
        cpu: Cpu::new(),
        memory: [0; 65536],
    };

    gb.cpu.b = 0x44;
    gb.cpu.a = 0x42;
    gb.memory[0x0000] = 0x3E;
    gb.memory[0x0001] = 4;
    println!("A: {:02X}", gb.cpu.a);
    gb.step();
    println!("A: {:02X}", gb.cpu.a);
}
