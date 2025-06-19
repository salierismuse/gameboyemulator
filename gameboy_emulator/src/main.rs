mod cpu;
mod ppu;
use ppu::{Ppu, PpuMode};
use cpu::{Cpu, Gameboy};
use std::fs::read;
use minifb::{Key, Window, WindowOptions};

fn main() {

    let mut gameboy = Gameboy {
        cpu: Cpu::new(),
        memory: [0; 65536],
        frame_buffer: [0; (160 * 144)],
    };
    let rom_data = std::fs::read(r"C:\Users\Isaac\OneDrive\Desktop\gameboygames\tetris.gb").expect("failed to read rom file");
    
    let mut window = Window::new(
        "gameboy emulator",
        160, 144, // gameboy resolution
        WindowOptions::default(),
    ).unwrap();
    
    let mut ppu = Ppu {
        scanline: 0,
        vram: [0; 8192],
        mode: PpuMode::OAM,
        cycle: 0,
        scx: 0,
        lcdc: 0,
        lyc: 0,
        scy: 0,
        wx: 0,
        wy: 0,
        stat: 0,
        bgp: 0,
        ly: 0,
    };

    gameboy.cpu.pc = 0x0000;  
    gameboy.cpu.halted = false;  
    

    
    gameboy.load_rom(&rom_data);


    while window.is_open() {
        gameboy.step(); 
        ppu.step(&mut gameboy);
        // let pixels = gameboy.get_frame_buffer();
        window.update_with_buffer(gameboy.get_frame_buffer(), 160, 144).unwrap();
    }
    // run a few steps
    // while true {
    //     println!("pc: {:04X}, a: {:02X}", gameboy.cpu.pc, gameboy.cpu.a);
    //     gameboy.step();
    // }
}