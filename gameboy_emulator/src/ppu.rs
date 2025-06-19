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
    fn fill_buffer(&mut self, gameboy: &mut Gameboy) {
        let mut ii = 0;
        let mut pixel_index = 0;
        let mut first_byte = (((0x8000)) as usize);
        
        while pixel_index < 23040 {
            for j in 0..8 {
                let bin_string1 = format!("{:b}", first_byte);
                let bin_string2 = format!("{:b}", first_byte+1);
                let bv1: Vec<char> = bin_string1.chars().collect();
                let bv2: Vec<char> = bin_string2.chars().collect();
                    for i in 0..8 {
                        if bv1[i] == '1' {
                            if bv2[i] == '0' {
                                gameboy.frame_buffer[pixel_index as usize] = 0x8bac0f;
                            }
                            else {
                                gameboy.frame_buffer[pixel_index as usize] = 0x9bbc0f; 
                            }
                        }
                        else {
                            if bv2[i] == '0' {
                                gameboy.frame_buffer[pixel_index as usize] = 0x0f380f; 
                            }
                            else {
                                
                                gameboy.frame_buffer[pixel_index as usize] = 0x306230; 
                            }
                        }
                        //print!("{:X}", gameboy.frame_buffer[pixel_index]);
                        pixel_index+=1;
                        } 
                    first_byte+=2;
                }
                ii+=1;
                }

//        gameboy.frame_buffer = gameboy.memory[0x9800..=0x9BFF];
    }

    // fn pixel_gen(byte_1: u8, byte_2: u8, pixel_index: u32) {
    //     let bv1: BitVec<u8> = BitVec::from_element(byte_1);
    //     let bv2: BitVec<u8> = BitVec::from_element(byte_2);
    //     for i in 0..7 {
    //         if bv1[i] == true {
    //             if bv2[i] == false {
    //                 gameboy.frame_buffer[pixel_index as usize] = 0xFF555555;
    //             }
    //             else {
    //                 gameboy.frame_buffer[pixel_index as usize] = 0xFF000000; 
    //             }
    //         }
    //         else {
    //             if bv2[i] == false {
    //                 gameboy.frame_buffer[pixel_index as usize] = 0xFFFFFFFF; 
    //             }
    //             else {
    //                 gameboy.frame_buffer[pixel_index as usize] = 0xFFAAAAAA; 
    //             }
    //         }
    //     } 

        
    pub fn step(&mut self, gameboy: &mut Gameboy) {
        self.fill_buffer(gameboy);
            
    }

}

fn main(){

}

