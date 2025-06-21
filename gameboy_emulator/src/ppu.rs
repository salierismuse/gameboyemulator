use crate::cpu::{Cpu, Gameboy};
use bitvec::prelude::*;
// read in vram
// load relevant bytes into the frame_buffer
// display

//scroll registers determinte viewports placement
pub enum PpuMode {
    OAM,
    Drawing,
    H_Blank,
    V_Blank,
}

pub struct Ppu {
    pub scanline: u8,
    pub vram: [u8; 8192],
    pub mode: PpuMode,
    pub cycle: u16,
    pub scx: u8, pub scy: u8, pub wx: u8, pub wy: u8, 
    pub lcdc: u8, pub stat: u8, pub bgp: u8, pub ly: u8, 
    pub lyc: u8,

}

impl Ppu {
    pub fn fill_buffer(&mut self, gameboy: &mut Gameboy, pixel_index: u8){
        for scanline in 0..144 {
            for tile in 0..20 {
                // fetch both bytes
                // process them, send them to buffer
                let cur_id = gameboy.memory[0x9800 + (tile) + (32 * (scanline/8))];
                let cur_addr = 0x8000 + (cur_id * 16) + (2 * (scanline%8));
                let byte_1 = gameboy.memory[addr];
                let byte_2 = gameboy.memory[addr+1];
            }
        }
    }
    pub fn step(&mut self, gameboy: &mut Gameboy) {
        pixel_index = 0;
        self.fill_buffer(gameboy, 0);
            
    }

}

fn main(){

}

