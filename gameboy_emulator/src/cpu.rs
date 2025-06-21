use minifb::{Key, Window, WindowOptions};
const ZERO_FLAG: u8 = 0b10000000;
const CARRY_FLAG: u8 = 0b00010000;
// just make a blanket flag resetter function when you can pass ignore in for flags you dont touch lol.


//test

//what
pub struct Cpu {
    // 8-bit registers
    pub a: u8,  // accumulator
    pub b: u8, pub c: u8,
    pub d: u8, pub e: u8,
    pub h: u8, pub l: u8,
    
    // 16-bit registers
    pub sp: u16,  // stack pointer
    pub pc: u16,  // program counter
    
    // flags register
    pub f: u8,

    // bools
    pub halted: bool,

    pub ime: bool,
}

pub struct Gameboy {
    pub cpu: Cpu,
    pub memory: [u8; 65536],
    pub frame_buffer: [u32; 160 * 144],
}

impl Gameboy {
    pub fn step(&mut self) {
        if self.cpu.halted{
            let if_flag = self.memory[0xFF0F];
            let ie_flag = self.memory[0xFFFF];
            if if_flag & ie_flag != 0 {
                self.cpu.halted = false;
            }
            else {
                return
            }
        }
        // attain opcode
        let opcode = self.fetch_byte();
        // match and execute
        self.execute_opcode(opcode);
    }

    fn fetch_byte(&mut self) -> u8 {
        let opcode = self.memory[self.cpu.pc as usize];
        // proper wrapping for overflows
        self.cpu.pc = self.cpu.pc.wrapping_add(1);
        opcode
    }

    pub fn load_rom(&mut self, rom_data: &[u8]){
        for (i, &byte) in rom_data.iter().enumerate() {
            if i < 0x8000 {
                self.memory[i] = byte;
            }
        }
    }

    fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x < 160 && y < 144 {
            self.frame_buffer[y * 160 + x] = color;
        }
    }

    pub fn get_frame_buffer(&self) -> &[u32] {
        &self.frame_buffer
    }

fn execute_opcode(&mut self, opcode: u8){
    match opcode {
        //loads - immediate
        0x3E => self.ld_x_n('a'),  // ld a,n
        0x06 => self.ld_x_n('b'),  // ld b,n
        0x0E => self.ld_x_n('c'),  // ld c,n
        0x16 => self.ld_x_n('d'),  // ld d,n
        0x1E => self.ld_x_n('e'),  // ld e,n
        0x26 => self.ld_x_n('h'),  // ld h,n
        0x2E => self.ld_x_n('l'),  // ld l,n

        //loads - register to register
        0x7F => self.ld_r_r('a', 'a'),  // ld a,a
        0x78 => self.ld_r_r('a', 'b'),  // ld a,b
        0x79 => self.ld_r_r('a', 'c'),  // ld a,c
        0x7A => self.ld_r_r('a', 'd'),  // ld a,d
        0x7B => self.ld_r_r('a', 'e'),  // ld a,e
        0x7C => self.ld_r_r('a', 'h'),  // ld a,h
        0x7D => self.ld_r_r('a', 'l'),  // ld a,l
        0x47 => self.ld_r_r('b', 'a'),  // ld b,a
        0x40 => self.ld_r_r('b', 'b'),  // ld b,b
        0x41 => self.ld_r_r('b', 'c'),  // ld b,c
        0x42 => self.ld_r_r('b', 'd'),  // ld b,d
        0x43 => self.ld_r_r('b', 'e'),  // ld b,e
        0x44 => self.ld_r_r('b', 'h'),  // ld b,h
        0x45 => self.ld_r_r('b', 'l'),  // ld b,l
        0x4F => self.ld_r_r('c', 'a'),  // ld c,a
        0x48 => self.ld_r_r('c', 'b'),  // ld c,b
        0x49 => self.ld_r_r('c', 'c'),  // ld c,c
        0x4A => self.ld_r_r('c', 'd'),  // ld c,d
        0x4B => self.ld_r_r('c', 'e'),  // ld c,e
        0x4C => self.ld_r_r('c', 'h'),  // ld c,h
        0x4D => self.ld_r_r('c', 'l'),  // ld c,l
        0x57 => self.ld_r_r('d', 'a'),  // ld d,a
        0x50 => self.ld_r_r('d', 'b'),  // ld d,b
        0x51 => self.ld_r_r('d', 'c'),  // ld d,c
        0x52 => self.ld_r_r('d', 'd'),  // ld d,d
        0x53 => self.ld_r_r('d', 'e'),  // ld d,e
        0x54 => self.ld_r_r('d', 'h'),  // ld d,h
        0x55 => self.ld_r_r('d', 'l'),  // ld d,l
        0x5F => self.ld_r_r('e', 'a'),  // ld e,a
        0x58 => self.ld_r_r('e', 'b'),  // ld e,b
        0x59 => self.ld_r_r('e', 'c'),  // ld e,c
        0x5A => self.ld_r_r('e', 'd'),  // ld e,d
        0x5B => self.ld_r_r('e', 'e'),  // ld e,e
        0x5C => self.ld_r_r('e', 'h'),  // ld e,h
        0x5D => self.ld_r_r('e', 'l'),  // ld e,l
        0x67 => self.ld_r_r('h', 'a'),  // ld h,a
        0x60 => self.ld_r_r('h', 'b'),  // ld h,b
        0x61 => self.ld_r_r('h', 'c'),  // ld h,c
        0x62 => self.ld_r_r('h', 'd'),  // ld h,d
        0x63 => self.ld_r_r('h', 'e'),  // ld h,e
        0x64 => self.ld_r_r('h', 'h'),  // ld h,h
        0x65 => self.ld_r_r('h', 'l'),  // ld h,l
        0x6F => self.ld_r_r('l', 'a'),  // ld l,a
        0x68 => self.ld_r_r('l', 'b'),  // ld l,b
        0x69 => self.ld_r_r('l', 'c'),  // ld l,c
        0x6A => self.ld_r_r('l', 'd'),  // ld l,d
        0x6B => self.ld_r_r('l', 'e'),  // ld l,e
        0x6C => self.ld_r_r('l', 'h'),  // ld l,h
        0x6D => self.ld_r_r('l', 'l'),  // ld l,l

        0xE0 => self.ldh_n_a(),
        0xF0 => self.ldh_a_n(),

        //ADD - all variants
        0x87 => self.add_a_x('a'),  // add a,a
        0x80 => self.add_a_x('b'),  // add a,b  
        0x81 => self.add_a_x('c'),  // add a,c
        0x82 => self.add_a_x('d'),  // add a,d
        0x83 => self.add_a_x('e'),  // add a,e
        0x84 => self.add_a_x('h'),  // add a,h
        0x85 => self.add_a_x('l'),  // add a,l
        0x86 => self.add_a_hl(),    // add a,(hl)
        0xC6 => self.add_a_n(),     // add a,n

        //SUB - all variants  
        0x97 => self.sub_a_x('a'),  // sub a,a
        0x90 => self.sub_a_x('b'),  // sub a,b
        0x91 => self.sub_a_x('c'),  // sub a,c
        0x92 => self.sub_a_x('d'),  // sub a,d
        0x93 => self.sub_a_x('e'),  // sub a,e
        0x94 => self.sub_a_x('h'),  // sub a,h
        0x95 => self.sub_a_x('l'),  // sub a,l
        0x96 => self.sub_a_hl(),    // sub a,(hl)
        0xD6 => self.sub_a_n(),     // sub a,n

        // adc 
        0x8F => self.adc_a_x('a'),  // adc a,a
        0x88 => self.adc_a_x('b'),  // adc a,b
        0x89 => self.adc_a_x('c'),  // adc a,c
        0x8A => self.adc_a_x('d'),  // adc a,d
        0x8B => self.adc_a_x('e'),  // adc a,e
        0x8C => self.adc_a_x('h'),  // adc a,h
        0x8D => self.adc_a_x('l'),  // adc a,l
        0x8E => self.adc_a_hl(),    // adc a,(hl)
        0xCE => self.adc_a_n(),     // adc a,n

        // sbc
        0x9F => self.sbc_a_x('a'),  // sbc a,a
        0x98 => self.sbc_a_x('b'),  // sbc a,b
        0x99 => self.sbc_a_x('c'),  // sbc a,c
        0x9A => self.sbc_a_x('d'),  // sbc a,d
        0x9B => self.sbc_a_x('e'),  // sbc a,e
        0x9C => self.sbc_a_x('h'),  // sbc a,h
        0x9D => self.sbc_a_x('l'),  // sbc a,l
        0x9E => self.sbc_a_hl(),    // sbc a,(hl)
        0xDE => self.sbc_a_n(),     // sbc a,n

        //XOR - all variants
        0xAF => self.xor_r('a'),    // xor a,a 
        0xA8 => self.xor_r('b'),    // xor a,b
        0xA9 => self.xor_r('c'),    // xor a,c
        0xAA => self.xor_r('d'),    // xor a,d
        0xAB => self.xor_r('e'),    // xor a,e
        0xAC => self.xor_r('h'),    // xor a,h
        0xAD => self.xor_r('l'),    // xor a,l
        0xAE => self.xor_hl(),      // xor a,(hl)
        0xEE => self.xor_n(),       // xor a,n

        //OR - all variants
        0xB7 => self.or_r('a'),     // or a,a
        0xB0 => self.or_r('b'),     // or a,b
        0xB1 => self.or_r('c'),     // or a,c
        0xB2 => self.or_r('d'),     // or a,d
        0xB3 => self.or_r('e'),     // or a,e
        0xB4 => self.or_r('h'),     // or a,h
        0xB5 => self.or_r('l'),     // or a,l
        0xB6 => self.or_hl(),       // or a,(hl)
        0xF6 => self.or_n(),        // or a,n

        //CP (compare) - all variants
        0xBF => self.cp_r_n('a'),   // cp a,a
        0xB8 => self.cp_r_n('b'),   // cp a,b
        0xB9 => self.cp_r_n('c'),   // cp a,c
        0xBA => self.cp_r_n('d'),   // cp a,d
        0xBB => self.cp_r_n('e'),   // cp a,e
        0xBC => self.cp_r_n('h'),   // cp a,h
        0xBD => self.cp_r_n('l'),   // cp a,l
        //0xBE => self.cp_hl(),       // cp a,(hl)
        0xFE => self.cp_n(),        // cp a,n

        // PUSH operations 
        0xF5 => self.push_nn("af"), // push af
        0xC5 => self.push_nn("bc"), // push bc
        0xD5 => self.push_nn("de"), // push de
        0xE5 => self.push_nn("hl"), // push hl

        //CALL
        0xCD => self.call_nn(),
        0xC4 => self.call_cc_nn("nz"),
        0xCC => self.call_cc_nn("z"),
        0xD4 => self.call_cc_nn("nc"),
        0xDC => self.call_cc_nn("c"),

        //rets
        0xC9 => self.ret(),
        0xC0 => self.ret_cc("nz"),
        0xC8 => self.ret_cc("z"),
        0xD0 => self.ret_cc(("nc")),
        0xD8 => self.ret_cc("c"),
        0xD9 => self.reti(),

        //loads - memory

        0x7E => self.ld_r_hl('a'),    // ld a,(hl)
        0x46 => self.ld_r_hl('b'),    // ld b,(hl) 
        0x4E => self.ld_r_hl('c'),    // ld c,(hl)
        0x56 => self.ld_r_hl('d'),    // ld d,(hl)
        0x5E => self.ld_r_hl('e'),    // ld e,(hl)
        0x66 => self.ld_r_hl('h'),    // ld h,(hl)
        0x6E => self.ld_r_hl('l'),    // ld l,(hl)

        0x77 => self.ld_hl_r('a'),    // ld (hl),a
        0x70 => self.ld_hl_r('b'),    // ld (hl),b
        0x71 => self.ld_hl_r('c'),    // ld (hl),c
        0x72 => self.ld_hl_r('d'),    // ld (hl),d
        0x73 => self.ld_hl_r('e'),    // ld (hl),e
        0x74 => self.ld_hl_r('h'),    // ld (hl),h
        0x75 => self.ld_hl_r('l'),    // ld (hl),l

        0x36 => self.ld_hl_n(),       // ld (hl),n
        // 0xBE => self.cp_hl(),         // cp a,(hl)
        // 0xFE => self.cp_n(),          // cp a,n

        0x32 => self.ldd_hl_a(),

        0xFA => self.ld_a_nn(),
        0x21 => self.ld_r_nn("hl"),
        0x01 => self.ld_r_nn("bc"),
        0x11 => self.ld_r_nn("de"),
        0x31 => self.ld_r_nn("sp"),

        //jumps
        0xC3 => self.jp_nn(),
        0xC2 => self.jp_nz_nn(),
        0xCA => self.jp_z_nn(),
        0xD2 => self.jp_nc_nn(),
        0xDA => self.jp_c_nn(),
        0xE9 => self.jp_hl(),
        0x18 => self.jr_n(),
        0x20 => self.jr_cc_n("nz"),
        0x28 => self.jr_cc_n("z"),
        0x30 => self.jr_cc_n("nc"),
        0x38 => self.jr_cc_n("c"),

        //AND
        0xA7 => self.and_a_gen('a'),
        0xA0 => self.and_a_gen('b'),
        0xA1 => self.and_a_gen('c'),
        0xA2 => self.and_a_gen('d'),
        0xA3 => self.and_a_gen('e'),
        0xA4 => self.and_a_gen('h'),
        0xA5 => self.and_a_gen('l'),
        0xA6 => self.and_hl(),
        0xE6 => self.and_n(),

        //NOP
        0x00 => {},

        //HALT
        0x76 => self.halt(),

       // 8-bit increment
        0x04 => self.inc_r_n('b'),
        0x0C => self.inc_r_n('c'),
        0x14 => self.inc_r_n('d'),
        0x1C => self.inc_r_n('e'),
        0x24 => self.inc_r_n('h'),
        0x2C => self.inc_r_n('l'),
        0x3C => self.inc_r_n('a'),

        // 8-bit decrement  
        0x05 => self.dec_r_n('b'),
        0x0D => self.dec_r_n('c'),
        0x15 => self.dec_r_n('d'),
        0x1D => self.dec_r_n('e'),
        0x25 => self.dec_r_n('h'),
        0x2D => self.dec_r_n('l'),
        0x3D => self.dec_r_n('a'),

        // 16-bit increment
        0x03 => self.inc_r_nn("bc"),
        0x13 => self.inc_r_nn("de"),
        0x23 => self.inc_r_nn("hl"),
        0x33 => self.inc_r_nn("sp"),

        // 16-bit decrement
        0x0B => self.dec_r_nn("bc"),
        0x1B => self.dec_r_nn("de"),
        0x2B => self.dec_r_nn("hl"),
        0x3B => self.dec_r_nn("sp"),

        // rst
        0xFF => self.rst(0x38),

        // pop
        0xF1 => self.pop_r_r("af"),
        0xC1 => self.pop_r_r("bc"),
        0xD1 => self.pop_r_r("de"), 
        0xE1 => self.pop_r_r("hl"),

        0x07 => self.rlc_r("a"),    // rotate a left circular
        0x0F => self.rrc_r("a"),    // rotate a right circular  
        0x17 => self.rl_r("a"),     // rotate a left through carry
        0x1F => self.rr_r("a"),     // rotate a right through carry

        //interrupts
        0xF3 => self.di(),

        //cb exit
        0xCB => {
            let val = self.fetch_byte();
            self.execute_cb_opcode(val);
        }
       
       _ => unimplemented!("opcode 0x{:02X} not implemented yet", opcode),
   }
}


fn execute_cb_opcode(&mut self, opcode: u8){
        let operation = opcode >> 6;
        let bit_num = (opcode >> 3) & 0x07;
        let reg_index = opcode & 0x07;

        let reg_char = match reg_index {
        0 => 'b', 1 => 'c', 2 => 'd', 3 => 'e',
        4 => 'h', 5 => 'l', 6 => '?', 7 => 'a', 
        _ => unreachable!()
    };
    match operation {
        0 => {
            let reg_str = if reg_index == 6 { "hl" } else { &reg_char.to_string() };
            match bit_num {
                0 => self.rlc_r(reg_str),
                1 => self.rrc_r(reg_str), 
                2 => self.rl_r(reg_str),
                3 => self.rr_r(reg_str),
                4 => self.sla_r(reg_str),
                5 => self.sra_r(reg_str),
//                6 => self.swap_r(reg_str),  // you'll need this one
//                7 => self.srl_r(reg_str),
                _ => unreachable!()
            }
        }
        1 => self.bit_b_r(bit_num, reg_char),  // BIT
        2 => self.res_b_r(bit_num, reg_char),  // RES  
        3 => self.set_b_r(bit_num, reg_char),  // SET
        _ => unimplemented!("not a bit operation")
    }
}

    //misc reg helpers
    fn get_reg(&mut self, reg: char) -> &mut u8
    {
    let curr_reg = match reg {
            'a' => &mut self.cpu.a,
            'b' => &mut self.cpu.b,
            'c' => &mut self.cpu.c,
            'd' => &mut self.cpu.d,
            'e' => &mut self.cpu.e,
            'h' => &mut self.cpu.h,
            'l' => &mut self.cpu.l,
            _ => unimplemented!("fail")
        };
        return curr_reg
    }

    // flag helpers
    fn set_z_flag(&mut self){
        self.cpu.f |= 0b10000000;
    }
    fn unset_z_flag(&mut self){
        self.cpu.f &= !0b10000000;
    }
    fn unset_n_flag(&mut self){
        self.cpu.f &= !0b01000000;
    }
    fn set_n_flag(&mut self){
        self.cpu.f |= 0b01000000;
    }
    fn set_h_flag(&mut self){
        self.cpu.f |= 0b00100000;
    }
    fn unset_h_flag(&mut self){
        self.cpu.f &= !0b00100000;
    }
    fn set_c_flag(&mut self){
        self.cpu.f |= 0b00010000;
    }
    fn unset_c_flag(&mut self){
        self.cpu.f &= !0b00010000;
    }

    fn z_flag_bool(&mut self) -> bool{
        self.cpu.f & 0b10000000 != 0
    }
    fn n_flag_bool(&mut self) -> bool{
        self.cpu.f & 0b01000000 != 0
    }
    fn h_flag_bool(&mut self) -> bool{
        self.cpu.f & 0b00100000 != 0
    }
    fn c_flag_bool(&mut self) -> bool {
        self.cpu.f & 0b00010000 != 0
    }

    fn set_flags(&mut self, z: char, n: char, h: char, c: char){
        match z {
            'i' => {},
            'y' => self.set_z_flag(),
            'n' => self.unset_z_flag(),
            _ => unimplemented!("fail!")
        }
        match n {
            'i' => {},
            'y' => self.set_n_flag(),
            'n' => self.unset_n_flag(),
            _ => unimplemented!("fail!")
        }
        match h {
            'i' => {},
            'y' => self.set_h_flag(),
            'n' => self.unset_h_flag(),
            _ => unimplemented!("fail!")
        }
        match c {
            'i' => {},
            'y' => self.set_c_flag(),
            'n' => self.unset_c_flag(),
            _ => unimplemented!("fail!")
        }
    }
    
    // rst
    fn rst(&mut self, addr: u8) {

        self.cpu.sp = self.cpu.sp.wrapping_sub(1);
        self.memory[self.cpu.sp as usize] = (self.cpu.pc >> 8) as u8;  // high byte
        self.cpu.sp = self.cpu.sp.wrapping_sub(1);
        self.memory[self.cpu.sp as usize] = self.cpu.pc as u8;         // low byte
        self.cpu.pc = addr as u16;
    }

    // rotate
    fn rlc_r(&mut self, reg: &str) {
        let (old_bit_7, val) = if reg == "hl"{
            let hl = (self.cpu.h as u16) << 8 | self.cpu.l as u16;
            let mut val = self.memory[hl as usize];
            let old_bit_7 = val & 0b10000000;
            val = val.rotate_left(1);
            (old_bit_7, val)

        }
        else { 
            let curr_reg = self.get_reg(reg.chars().next().unwrap());
            let old_bit_7 = *curr_reg & 0b10000000;
            *curr_reg = (*curr_reg).rotate_left(1);
            (old_bit_7, *curr_reg)
        };
         
        self.set_flags(if val == 0 {'y'} else {'n'}, 'n', 'n', if old_bit_7 != 0 {'y'} else {'n'})
    }

    fn rrc_r(&mut self, reg: &str) {
        let (old_bit_0, val) = if reg == "hl"{
            let hl = (self.cpu.h as u16) << 8 | self.cpu.l as u16;
            let mut val = self.memory[hl as usize];
            let old_bit_0 = val & 0b00000001;
            val = val.rotate_right(1);
            (old_bit_0, val)

        }
        else { 
            let curr_reg = self.get_reg(reg.chars().next().unwrap());
            let old_bit_0 = *curr_reg & 0b00000001;
            *curr_reg = (*curr_reg).rotate_right(1);
            (old_bit_0, *curr_reg)
        };
         
        self.set_flags(if val == 0 {'y'} else {'n'}, 'n', 'n', if old_bit_0 != 0 {'y'} else {'n'})
    }

    fn rl_r(&mut self, reg: &str) {
        let old_carry = self.c_flag_bool();
        let (old_bit_7, val) = if reg == "hl"{
            let hl = (self.cpu.h as u16) << 8 | self.cpu.l as u16;
            let mut val = self.memory[hl as usize];
            let old_bit_7 = val & 0b10000000;
            val = val << 1;
            if old_carry {
                val |= 0b00000001;
            }
            self.memory[hl as usize] = val;
            (old_bit_7, val)
        } else {
            let curr_reg = self.get_reg(reg.chars().next().unwrap());
            let old_bit_7 = (*curr_reg & 0b10000000);
            *curr_reg = *curr_reg << 1;
            if old_carry {
                *curr_reg |= 0b00000001;
            }
            (old_bit_7, *curr_reg)
        };
    self.set_flags(if val == 0 {'y'} else {'n'}, 'n', 'n', if old_bit_7 != 0 {'y'} else {'n'});
    }

    fn rr_r(&mut self, reg: &str) {
        let old_carry = self.c_flag_bool();
        let (old_bit_0, val) = if reg == "hl"{
            let hl = (self.cpu.h as u16) << 8 | self.cpu.l as u16;
            let mut val = self.memory[hl as usize];
            let old_bit_0 = val & 0b00000001;
            val = val >> 1;
            if old_carry {
                val |= 0b10000000
            }
            self.memory[hl as usize] = val;
            (old_bit_0, val)
        }
        else {
            let curr_reg = self.get_reg(reg.chars().next().unwrap());
            let old_bit_0 = *curr_reg & 0b00000001;
            *curr_reg = (*curr_reg) >> 1;
            if old_carry {
                *curr_reg |= 0b10000000
            }
            (old_bit_0, *curr_reg)
        };

    self.set_flags(if val == 0 {'y'} else {'n'}, 'n', 'n', if old_bit_0 != 0 {'y'} else {'n'} );
    }

    //shift

    fn sla_r(&mut self, reg: &str) {
        let (bit_7, val) = if reg == "hl" {
            let hl = (self.cpu.h as u16) << 8 | self.cpu.l as u16;
            let mut val = self.memory[hl as usize];
            let bit_7 = val & 0b10000000;
            val = (val) << 1;
            self.memory[hl as usize] = val;
            (bit_7, val)
        }
        else {
            let curr_reg = self.get_reg(reg.chars().next().unwrap());
            let bit_7 = *curr_reg & 0b10000000;
            *curr_reg = (*curr_reg) << 1;
            let val = *curr_reg;
            (bit_7, val)
        };
         
        self.set_flags(if val == 0 {'y'} else {'n'}, 'n', 'n', if bit_7 != 0 {'y'} else {'n'})
    }

    fn sra_r(&mut self, reg: &str) {
        let (bit_0, val) = if reg == "hl" { 
            let hl = (self.cpu.h as u16) << 8 | self.cpu.l as u16;
            let mut val = self.memory[hl as usize];
            let bit_7 = val & 0b10000000;
            let bit_0 = val & 0b00000001;
            val = val >> 1;
            val |= bit_7;
            self.memory[hl as usize] = val;
            (bit_0, val)
        }
        else {
            let curr_reg = self.get_reg(reg.chars().next().unwrap());
            let bit_7 = *curr_reg & 0b10000000;
            let bit_0 = *curr_reg & 0b00000001;
            *curr_reg = (*curr_reg) >> 1;
            *curr_reg |= bit_7;
            let val = *curr_reg;
            (bit_0, val)
        };
        self.set_flags(if val == 0 {'y'} else {'n'}, 'n', 'n', if bit_0 != 0 {'y'} else {'n'})
    }

    fn srl_r(&mut self, reg: &str) {
        let (bit_0, val) = if reg == "hl"
            {
                let hl = (self.cpu.h as u16) << 8 | self.cpu.l as u16;
                let mut val = self.memory[hl as usize];
                let bit_0 = val & 0b10000000;
                val = (val) >> 1;
                self.memory[hl as usize] = val; 
                (bit_0, val)
            }
        else {
            let curr_reg = self.get_reg(reg.chars().next().unwrap());
            let bit_0 = *curr_reg & 0b00000001;
            *curr_reg = (*curr_reg) >> 1;
            let val = *curr_reg;
            (bit_0, val)
        };
        self.set_flags(if val == 0 {'y'} else {'n'}, 'n', 'n', if bit_0 != 0 {'y'} else {'n'})
    }


    // call

    fn call_loader(&mut self, byte: u16){
        self.cpu.sp = self.cpu.sp.wrapping_sub(1);
        self.memory[self.cpu.sp as usize] = (self.cpu.pc >> 8) as u8;
        self.cpu.sp = self.cpu.sp.wrapping_sub(1);
        self.memory[self.cpu.sp as usize] = (self.cpu.pc as u8);
        self.cpu.pc = byte;
    }

    fn call_nn(&mut self) {
        let low_byte = self.fetch_byte();
        let high_byte = self.fetch_byte();
        let total_byte = (high_byte as u16) << 8  | low_byte as u16;
        self.call_loader(total_byte);
    }

    fn call_cc_nn(&mut self, flag: &str){
        let low_byte = self.fetch_byte();
        let high_byte = self.fetch_byte();
        let total_byte = (high_byte as u16) << 8  | low_byte as u16;
        match flag {
            "z" => {if self.z_flag_bool() { self.call_loader(total_byte);}}
            "nz" => {if !self.z_flag_bool() { self.call_loader(total_byte);}}
            "c" => {if self.c_flag_bool() { self.call_loader(total_byte);}}
            "nc" => if !self.c_flag_bool() { self.call_loader(total_byte);}
            _ => unimplemented!("fail")
        }
    }

    // returns
    fn ret(&mut self) {
        let val1 = self.memory[self.cpu.sp as usize];
        self.cpu.sp = self.cpu.sp.wrapping_add(1);
        let val2 = self.memory[self.cpu.sp as usize];
        self.cpu.sp = self.cpu.sp.wrapping_add(1);
        let byte = (val2 as u16) << 8 | val1 as u16;
        self.cpu.pc = byte;
    }

    fn ret_cc(&mut self, flag: &str){
        match flag {
            "z" => if self.z_flag_bool() {self.ret()},
            "nz" => if !self.z_flag_bool() { self.ret()},
            "c" => if self.c_flag_bool() { self.ret();},
            "nc" => if !self.c_flag_bool() { self.ret();},
            _ => unimplemented!("fail")
        }
    }

    fn reti(&mut self) {
        self.ret();
        self.cpu.ime = true;
    }

    //pops
    fn pop_r_r(&mut self, regs: &str){
        match regs {
            "af" => {self.cpu.f = self.memory[self.cpu.sp as usize]; self.cpu.sp +=1; self.cpu.a = self.memory[self.cpu.sp as usize]; self.cpu.sp += 1},
            "bc" => {self.cpu.c = self.memory[self.cpu.sp as usize]; self.cpu.sp +=1; self.cpu.b = self.memory[self.cpu.sp as usize]; self.cpu.sp +=1},
            "de" => {self.cpu.e = self.memory[self.cpu.sp as usize]; self.cpu.sp +=1; self.cpu.d = self.memory[self.cpu.sp as usize]; self.cpu.sp += 1},
            "hl" => {self.cpu.l = self.memory[self.cpu.sp as usize]; self.cpu.sp +=1; self.cpu.h = self.memory[self.cpu.sp as usize]; self.cpu.sp +=1},
            _ => unimplemented!("error")

        }
    }

    //pushes
    fn push_nn(&mut self, regs: &str) {
        match regs {
            "af" => { self.cpu.sp = self.cpu.sp.wrapping_sub(1); self.memory[self.cpu.sp as usize] = self.cpu.a; self.cpu.sp = self.cpu.sp.wrapping_sub(1); self.memory[self.cpu.sp as usize] = self.cpu.f },
            "bc" => { self.cpu.sp = self.cpu.sp.wrapping_sub(1); self.memory[self.cpu.sp as usize] = self.cpu.b; self.cpu.sp = self.cpu.sp.wrapping_sub(1); self.memory[self.cpu.sp as usize] = self.cpu.c },
            "de" => { self.cpu.sp = self.cpu.sp.wrapping_sub(1); self.memory[self.cpu.sp as usize] = self.cpu.d; self.cpu.sp = self.cpu.sp.wrapping_sub(1); self.memory[self.cpu.sp as usize] = self.cpu.e },
            "hl" => { self.cpu.sp = self.cpu.sp.wrapping_sub(1); self.memory[self.cpu.sp as usize] = self.cpu.h; self.cpu.sp = self.cpu.sp.wrapping_sub(1); self.memory[self.cpu.sp as usize] = self.cpu.l },
            _ => unimplemented!("error")

        }
    }

    // increment
    fn inc_r_n(&mut self, reg: char) {
        let curr_reg = self.get_reg(reg);
        let old_value = *curr_reg;
        *curr_reg  = curr_reg.wrapping_add(1);
        let result = *curr_reg;
         
        if result == 0 {
            self.set_z_flag();
        }
        else {
            self.unset_z_flag();
        }
        self.unset_n_flag();
        if (old_value & 0x0F) == 0x0F {
            self.set_h_flag();
        }
        else {
            self.unset_h_flag();
        }
    }

    fn inc_r_nn(&mut self, reg_pair: &str) {
        match reg_pair {
            "bc" => {
                let bc = (self.cpu.b as u16) << 8 | self.cpu.c as u16;
                let result = bc.wrapping_add(1);
                self.cpu.b = (result >> 8) as u8;
                self.cpu.c = result as u8;
            }
            "de" => {
                let de = (self.cpu.d as u16) << 8 | self.cpu.e as u16;
                let result = de.wrapping_add(1);
                self.cpu.d = (result >> 8) as u8;
                self.cpu.e = result as u8;
            }
            "hl" => {
                let hl = (self.cpu.h as u16) << 8 | self.cpu.l as u16;
                let result = hl.wrapping_add(1);
                self.cpu.h = (result >> 8) as u8;
                self.cpu.l = result as u8;
            }
            "sp" => {
                self.cpu.sp = self.cpu.sp.wrapping_add(1);
            }
            _ => unimplemented!("fail")
        }
    }




    //dec
    fn dec_r_n(&mut self, reg: char)
    {
        let curr_reg = self.get_reg(reg);
        let old_value = *curr_reg;
        *curr_reg  = curr_reg.wrapping_sub(1);
        let result = *curr_reg;
         
        if result == 0 {
            self.set_z_flag();
        }
        else {
            self.unset_z_flag();
        }
        self.set_n_flag();
        if (old_value & 0x0F) == 0x00 {
            self.set_h_flag();
        }
        else {
            self.unset_h_flag();
        }
    }

    fn dec_r_nn(&mut self, reg_pair: &str) {
        match reg_pair {
            "bc" => {
                let bc = (self.cpu.b as u16) << 8 | self.cpu.c as u16;
                let result = bc.wrapping_sub(1);
                self.cpu.b = (result >> 8) as u8;
                self.cpu.c = result as u8;
            }
            "de" => {
                let de = (self.cpu.d as u16) << 8 | self.cpu.e as u16;
                let result = de.wrapping_sub(1);
                self.cpu.d = (result >> 8) as u8;
                self.cpu.e = result as u8;
            }
            "hl" => {
                let hl = (self.cpu.h as u16) << 8 | self.cpu.l as u16;
                let result = hl.wrapping_sub(1);
                self.cpu.h = (result >> 8) as u8;
                self.cpu.l = result as u8;
            }
            "sp" => {
                self.cpu.sp = self.cpu.sp.wrapping_sub(1);
            }
            _ => unimplemented!("fail")
        }
    }

    //halt
    fn halt(&mut self){
        self.cpu.halted = true;
    }

    // and

    fn and_a_gen(&mut self, reg: char){
        let curr_reg = match reg {
            'a' => self.cpu.a,
            'b' => self.cpu.b,
            'c' => self.cpu.c,
            'd' => self.cpu.d,
            'e' => self.cpu.e,
            'h' => self.cpu.h,
            'l' => self.cpu.l,
            _ => unimplemented!("fail")
        };
        self.cpu.a = self.cpu.a & curr_reg;
        if self.cpu.a == 0 {
            self.set_z_flag();
        }
        else {
            self.unset_z_flag();
        }
        self.unset_n_flag();
        self.set_h_flag();
        self.unset_c_flag();
    }

    fn and_n(&mut self){
        let value = self.fetch_byte();
        self.cpu.a = self.cpu.a & value;
        if self.cpu.a == 0 {
            self.set_z_flag();
        }
        else {
            self.unset_z_flag();
        }
        self.unset_n_flag();
        self.set_h_flag();
        self.unset_c_flag();
    }

    fn and_hl(&mut self){
        let low_byte = self.cpu.l;
        let high_byte = self.cpu.h;
        let total_byte = (high_byte as u16) << 8 | (low_byte as u16);
        self.cpu.a = self.cpu.a & self.memory[total_byte as usize];
        if self.cpu.a == 0 {
            self.set_z_flag();
        }
        else {
            self.unset_z_flag();
        }
        self.unset_n_flag();
        self.set_h_flag();
        self.unset_c_flag();
    }

    // interrupts
    fn di(&mut self) {
        self.cpu.ime = false;  
    }

    // bit stuff
    fn bit_b_r(&mut self, bit: u8, reg: char) {
        self.unset_n_flag();
        self.set_h_flag();
        let curr_reg = self.get_reg(reg);
        let val = *curr_reg;
         
        let mask = 1 << bit;
        if (val & mask == 0)
        {
            self.set_z_flag();
        }
        else
        {
            self.unset_z_flag();
        }
    }

    fn set_b_r(&mut self, bit: u8, reg: char) {
        let curr_reg = self.get_reg(reg);
        let mask = 1 << bit;
        *curr_reg |= mask;
    }

    fn res_b_r(&mut self, bit: u8, reg: char) {
        let curr_reg = self.get_reg(reg);
        let mask = 1 << bit;
        *curr_reg &= !mask;
    }

    // lds

    fn ldh_a_n(&mut self) {
        let inc = self.fetch_byte() as u16;
        let addr = (inc + 0xFF00) as u16;
        self.cpu.a = self.memory[addr as usize];
    }

    fn ldh_n_a(&mut self) {
        let val = self.cpu.a;
        let inc = self.fetch_byte() as u16;
        let addr = (inc + 0xFF00) as u16;
        self.memory[addr as usize] = val;
    }

    fn ldd_hl_a(&mut self) {
        self.ld_hl_r('a');
        self.dec_r_nn("hl");
    }

    //ld x, n, immediate
    fn ld_x_n(&mut self, reg: char)
    {
        let val = self.fetch_byte();
            let curr_reg = match reg {
            'a' => &mut self.cpu.a,
            'b' => &mut self.cpu.b,
            'c' => &mut self.cpu.c,
            'd' => &mut self.cpu.d,
            'e' => &mut self.cpu.e,
            'h' => &mut self.cpu.h,
            'l' => &mut self.cpu.l,
            _ => unimplemented!("fail")
        };

        *curr_reg = val;
    }

    //ld, reg to reg
    fn ld_r_r(&mut self, reg1: char, reg2: char){
          let curr_reg2 = match reg2 {
            'a' => self.cpu.a,
            'b' => self.cpu.b,
            'c' => self.cpu.c,
            'd' => self.cpu.d,
            'e' => self.cpu.e,
            'h' => self.cpu.h,
            'l' => self.cpu.l,
            _ => unimplemented!("fail")
        };
            let curr_reg1 = match reg1 {
            'a' => &mut self.cpu.a,
            'b' => &mut self.cpu.b,
            'c' => &mut self.cpu.c,
            'd' => &mut self.cpu.d,
            'e' => &mut self.cpu.e,
            'h' => &mut self.cpu.h,
            'l' => &mut self.cpu.l,
            _ => unimplemented!("fail")
        };
        *curr_reg1 = curr_reg2;
    }

    fn ld_a_n(&mut self) {
        self.cpu.a = self.memory[self.cpu.pc as usize];
        self.cpu.pc = self.cpu.pc.wrapping_add(1);
    
    }

    fn ld_a_nn(&mut self) {
        let low_byte = self.memory[self.cpu.pc as usize];
        self.cpu.pc = self.cpu.pc.wrapping_add(1);
        let high_byte = self.memory[self.cpu.pc as usize];
        self.cpu.pc = self.cpu.pc.wrapping_add(1);
        let total_byte = (high_byte as u16) << 8 | (low_byte as u16);
        self.cpu.a = self.memory[total_byte as usize];
    }

    fn ld_hl_r(&mut self, reg: char) {
        let total_byte = (self.cpu.h as u16) << 8 | (self.cpu.l as u16);
        let curr_reg = self.get_reg(reg);
        let val = *curr_reg;
         
        self.memory[total_byte as usize] = val;
    }   

    fn ld_r_hl(&mut self, reg: char) {
        let total_byte = (self.cpu.h as u16) << 8 | (self.cpu.l as u16);
        let val = self.memory[total_byte as usize];
        let curr_reg = self.get_reg(reg);
        *curr_reg = val;
    }   

    fn ld_hl_n(&mut self) {
        let total_byte = (self.cpu.h as u16) << 8 | (self.cpu.l as u16);
        self.memory[total_byte as usize] = self.fetch_byte();
    }

    fn ld_r_nn(&mut self, reg_pair: &str) {
        match reg_pair {
            "bc" => {
                self.cpu.c = self.fetch_byte();
                self.cpu.b = self.fetch_byte();
            }
            "de" => {
                self.cpu.e = self.fetch_byte();
                self.cpu.d = self.fetch_byte();
            }
            "hl" => {
                self.cpu.l = self.fetch_byte();
                self.cpu.h = self.fetch_byte();
            }
            "sp" => {
                let low_byte = self.fetch_byte();
                let high_byte = self.fetch_byte();
                let total_byte = ((high_byte as u16) << 8 | low_byte as u16);
                self.cpu.sp = total_byte;
            }
            _ => unimplemented!("fail")
    }
}

    // jumps

    fn jr_cc_n(&mut self, fl: &str) {
        let offset = self.fetch_byte() as i8;
        match fl {
            "z" => {if self.z_flag_bool() {self.cpu.pc = self.cpu.pc.wrapping_add(offset as u16)}},
            "nz" => {if self.z_flag_bool() == false {self.cpu.pc = self.cpu.pc.wrapping_add(offset as u16)}},
            "c" => {if self.c_flag_bool() {self.cpu.pc=self.cpu.pc.wrapping_add(offset as u16)}},
            "nc" => {if self.c_flag_bool() == false {self.cpu.pc = self.cpu.pc.wrapping_add(offset as u16)}}
            _ => unimplemented!("fail")
        }
    }

    fn jr_n(&mut self) {
        let offset = self.fetch_byte() as i8;
        self.cpu.pc = self.cpu.pc.wrapping_add(offset as u16)
    }

    fn jp_hl(&mut self) {
        let total_byte = (self.cpu.h as u16) << 8 | (self.cpu.l as u16);
        self.cpu.pc = total_byte;
    }

    fn jp_nn(&mut self) {
        let low_byte = self.memory[self.cpu.pc as usize];
        self.cpu.pc = self.cpu.pc.wrapping_add(1);
        let high_byte = self.memory[self.cpu.pc as usize];
        self.cpu.pc = self.cpu.pc.wrapping_add(1);
        let total_byte = (high_byte as u16) << 8 | (low_byte as u16);
        self.cpu.pc = total_byte;
    }

    fn jp_nz_nn(&mut self) {
        let low_byte = self.memory[self.cpu.pc as usize];
        self.cpu.pc = self.cpu.pc.wrapping_add(1);
        let high_byte = self.memory[self.cpu.pc as usize];
        self.cpu.pc = self.cpu.pc.wrapping_add(1);
        let total_byte = (high_byte as u16) << 8 | (low_byte as u16);
        if self.cpu.f & 0b10000000 == 0 {
            self.cpu.pc = total_byte;
        }
    }

    fn jp_z_nn(&mut self){
        let low_byte = self.memory[self.cpu.pc as usize];
        self.cpu.pc = self.cpu.pc.wrapping_add(1);
        let high_byte = self.memory[self.cpu.pc as usize];
        self.cpu.pc = self.cpu.pc.wrapping_add(1);
        let total_byte = (high_byte as u16) << 8 | (low_byte as u16);
        if self.cpu.f & 0b10000000 != 0 {
            self.cpu.pc = total_byte;
        }
    }

    fn jp_nc_nn(&mut self) {
        let low_byte = self.memory[self.cpu.pc as usize];
        self.cpu.pc = self.cpu.pc.wrapping_add(1);
        let high_byte = self.memory[self.cpu.pc as usize];
        self.cpu.pc = self.cpu.pc.wrapping_add(1);
        let total_byte = (high_byte as u16) << 8 | (low_byte as u16);
        if self.cpu.f & 0b00010000 == 0 {
            self.cpu.pc = total_byte;
        }  
    }

    fn jp_c_nn(&mut self) {
        let low_byte = self.memory[self.cpu.pc as usize];
        self.cpu.pc = self.cpu.pc.wrapping_add(1);
        let high_byte = self.memory[self.cpu.pc as usize];
        self.cpu.pc = self.cpu.pc.wrapping_add(1);
        let total_byte = (high_byte as u16) << 8 | (low_byte as u16);
        if self.cpu.f & 0b00010000 != 0 {
            self.cpu.pc = total_byte;
        }  
    }

    // add A, n
    fn add_a_x(&mut self, reg: char) {
        let old_value = self.cpu.a;
        let curr_reg = self.get_reg(reg);
        let val = *curr_reg;
         
        let (result, overflow) = self.cpu.a.overflowing_add(val);
        self.cpu.a = result;
        self.unset_n_flag();
        if self.cpu.a == 0{
            self.set_z_flag();
        }
        else {
            self.unset_z_flag();
        }

        if (old_value & 0x0F) + (val & 0x0F) > 0x0F {
            self.set_h_flag();
        }
        else {
            self.unset_h_flag();
        }

        if overflow {
            self.set_c_flag();
        }
        else {
            self.unset_c_flag();
        }
    }

    fn add_a_hl(&mut self) {
        let address = (self.cpu.h as u16) << 8 | self.cpu.l as u16;
        let old_value = self.cpu.a;
        let val = self.memory[address as usize];
        let (result, overflow) = self.cpu.a.overflowing_add(val);
        self.cpu.a = result;
        self.unset_n_flag();
        if self.cpu.a == 0{
            self.set_z_flag();
        }
        else {
            self.unset_z_flag();
        }

        if (old_value & 0x0F) + (val & 0x0F) > 0x0F {
            self.set_h_flag();
        }
        else {
            self.unset_h_flag();
        }

        if overflow {
            self.set_c_flag();
        }
        else {
            self.unset_c_flag();
        }
    }

    fn add_a_n(&mut self) {
        let val = self.fetch_byte();
        let old_value = self.cpu.a;
        let (result, overflow) = self.cpu.a.overflowing_add(val);
        self.cpu.a = result;
        self.unset_n_flag();
        if self.cpu.a == 0{
            self.set_z_flag();
        }
        else {
            self.unset_z_flag();
        }

        if (old_value & 0x0F) + (val & 0x0F) > 0x0F {
            self.set_h_flag();
        }
        else {
            self.unset_h_flag();
        }

        if overflow {
            self.set_c_flag();
        }
        else {
            self.unset_c_flag();
        }
    }

    // potential edge case where adding the flag bit overflows val and we crash
    fn adc_a_x(&mut self, reg: char) {
        let old_value = self.cpu.a;
        let curr_reg = self.get_reg(reg);
        let val = *curr_reg;
         
        let (result1, overflow1) = self.cpu.a.overflowing_add(val);
        let (result, overflow2) = result1.overflowing_add(self.c_flag_bool() as u8);
        let overflow = (overflow1 || overflow2);

        self.cpu.a = result;
        self.unset_n_flag();
        if self.cpu.a == 0{
            self.set_z_flag();
        }
        else {
            self.unset_z_flag();
        }

        if (old_value & 0x0F) + (val & 0x0F) + self.c_flag_bool() as u8 > 0x0F {
            self.set_h_flag();
        }
        else {
            self.unset_h_flag();
        }

        if overflow {
            self.set_c_flag();
        }
        else {
            self.unset_c_flag();
        }
    }

    fn adc_a_hl(&mut self) {
        let address = (self.cpu.h as u16) << 8 | self.cpu.l as u16;
        let old_value = self.cpu.a;
        let val = self.memory[address as usize];
        let (result1, overflow1) = self.cpu.a.overflowing_add(val);
        let (result, overflow2) = result1.overflowing_add(self.c_flag_bool() as u8);
        let overflow = (overflow1 || overflow2);
        self.cpu.a = result;
        self.unset_n_flag();
        if self.cpu.a == 0{
            self.set_z_flag();
        }
        else {
            self.unset_z_flag();
        }

        if (old_value & 0x0F) + (val & 0x0F) + self.c_flag_bool() as u8 > 0x0F {
            self.set_h_flag();
        }
        else {
            self.unset_h_flag();
        }

        if overflow {
            self.set_c_flag();
        }
        else {
            self.unset_c_flag();
        }
    }

    fn adc_a_n(&mut self) {
        let val = self.fetch_byte();
        let old_value = self.cpu.a;
        let (result1, overflow1) = self.cpu.a.overflowing_add(val);
        let (result, overflow2) = result1.overflowing_add(self.c_flag_bool() as u8);
        let overflow = overflow1 || overflow2;
        self.cpu.a = result;
        self.unset_n_flag();
        if self.cpu.a == 0{
            self.set_z_flag();
        }
        else {
            self.unset_z_flag();
        }

        if (old_value & 0x0F) + (val & 0x0F) + self.c_flag_bool() as u8 > 0x0F {
            self.set_h_flag();
        }
        else {
            self.unset_h_flag();
        }

        if overflow {
            self.set_c_flag();
        }
        else {
            self.unset_c_flag();
        }
    }


    //sub functions

    fn sub_a_x(&mut self, reg: char) {
        let old_value = self.cpu.a;
        let curr_reg = self.get_reg(reg);
        let val = *curr_reg;
         
        let (result, overflow) = self.cpu.a.overflowing_sub(val);
        self.cpu.a = result;
        self.set_n_flag();
        if self.cpu.a == 0{
            self.set_z_flag();
        }
        else {
            self.unset_z_flag();
        }

        if (old_value & 0x0F) < (val & 0x0F) {
            self.set_h_flag();
        }
        else {
            self.unset_h_flag();
        }

        if overflow {
            self.set_c_flag();
        }
        else {
            self.unset_c_flag();
        }
    }

    fn sub_a_hl(&mut self) {
        let address = (self.cpu.h as u16) << 8 | self.cpu.l as u16;
        let old_value = self.cpu.a;
        let val = self.memory[address as usize];
        let (result, overflow) = self.cpu.a.overflowing_sub(val);
        self.cpu.a = result;
        self.set_n_flag();
        if self.cpu.a == 0{
            self.set_z_flag();
        }
        else {
            self.unset_z_flag();
        }

        if (old_value & 0x0F) < (val & 0x0F) {
            self.set_h_flag();
        }
        else {
            self.unset_h_flag();
        }

        if overflow {
            self.set_c_flag();
        }
        else {
            self.unset_c_flag();
        }
    }

    fn sub_a_n(&mut self) {
        let old_value = self.cpu.a;
        let val = self.fetch_byte();
        let (result, overflow) = self.cpu.a.overflowing_sub(val);
        self.cpu.a = result;
        self.set_n_flag();
        if self.cpu.a == 0{
            self.set_z_flag();
        }
        else {
            self.unset_z_flag();
        }

        if (old_value & 0x0F) < (val & 0x0F) {
            self.set_h_flag();
        }
        else {
            self.unset_h_flag();
        }

        if overflow {
            self.set_c_flag();
        }
        else {
            self.unset_c_flag();
        }

    }

    fn sbc_a_x(&mut self, reg: char) {
        let old_value = self.cpu.a;
        let curr_reg = self.get_reg(reg);
        let val = *curr_reg;
         
        let (result1, overflow1) = self.cpu.a.overflowing_sub(val);
        let (result, overflow2) = result1.overflowing_sub(self.c_flag_bool() as u8);
        let overflow = overflow1 || overflow2;
        self.cpu.a = result;
        self.set_n_flag();
        if self.cpu.a == 0{
            self.set_z_flag();
        }
        else {
            self.unset_z_flag();
        }

        if (old_value & 0x0F) < (val & 0x0F) + (self.c_flag_bool() as u8) {
            self.set_h_flag();
        }
        else {
            self.unset_h_flag();
        }

        if overflow {
            self.set_c_flag();
        }
        else {
            self.unset_c_flag();
        }
    }

    fn sbc_a_hl(&mut self) {
        let address = (self.cpu.h as u16) << 8 | self.cpu.l as u16;
        let old_value = self.cpu.a;
        let val = self.memory[address as usize];
        let (result1, overflow1) = self.cpu.a.overflowing_sub(val);
        let (result, overflow2) = result1.overflowing_sub(self.c_flag_bool() as u8);
        let overflow = overflow1 || overflow2;
        self.cpu.a = result;
        self.set_n_flag();
        if self.cpu.a == 0{
            self.set_z_flag();
        }
        else {
            self.unset_z_flag();
        }

        if (old_value & 0x0F) < (val & 0x0F) + (self.c_flag_bool() as u8) {
            self.set_h_flag();
        }
        else {
            self.unset_h_flag();
        }

        if overflow {
            self.set_c_flag();
        }
        else {
            self.unset_c_flag();
        }
    }

    fn sbc_a_n(&mut self) {
        let old_value = self.cpu.a;
        let val = self.fetch_byte();
        let (result1, overflow1) = self.cpu.a.overflowing_sub(val);
        let (result, overflow2) = result1.overflowing_sub(self.c_flag_bool() as u8);
        let overflow = overflow1 || overflow2;
        self.cpu.a = result;
        self.set_n_flag();
        if self.cpu.a == 0{
            self.set_z_flag();
        }
        else {
            self.unset_z_flag();
        }

        if (old_value & 0x0F) < (val & 0x0F) + (self.c_flag_bool() as u8) {
            self.set_h_flag();
        }
        else {
            self.unset_h_flag();
        }

        if overflow {
            self.set_c_flag();
        }
        else {
            self.unset_c_flag();
        }

    }

    //xors

    fn xor_r(&mut self, reg: char){
        let curr_reg = self.get_reg(reg);
        let val = *curr_reg;
         
        self.cpu.a = self.cpu.a ^ val;
        if self.cpu.a == 0 {
            self.set_z_flag();
        }
        else {
            self.unset_z_flag();
        }
        self.unset_h_flag();
        self.unset_c_flag();
        self.unset_n_flag();
    }

    fn xor_hl(&mut self){
        let byte = (self.cpu.h as u16) << 8 | self.cpu.l as u16;
        let val = self.memory[byte as usize];
        self.cpu.a = self.cpu.a ^ val;
        if self.cpu.a == 0 {
            self.set_z_flag();
        }
        else {
            self.unset_z_flag();
        }
        self.unset_h_flag();
        self.unset_c_flag();
        self.unset_n_flag();
    }

    fn xor_n(&mut self){
        let val = self.fetch_byte();
        self.cpu.a = self.cpu.a ^ val;
        if self.cpu.a == 0 {
            self.set_z_flag();
        }
        else {
            self.unset_z_flag();
        }
        self.unset_h_flag();
        self.unset_c_flag();
        self.unset_n_flag();
    }

    fn or_r(&mut self, reg: char){
        let curr_reg = self.get_reg(reg);
        let val = *curr_reg;
         
        self.cpu.a = self.cpu.a | val;
        if self.cpu.a == 0 {
            self.set_z_flag();
        }
        else {
            self.unset_z_flag();
        }
        self.unset_h_flag();
        self.unset_c_flag();
        self.unset_n_flag();
    }

    fn or_hl(&mut self){
        let byte = (self.cpu.h as u16) << 8 | self.cpu.l as u16;
        let val = self.memory[byte as usize];
        self.cpu.a = self.cpu.a | val;
        if self.cpu.a == 0 {
            self.set_z_flag();
        }
        else {
            self.unset_z_flag();
        }
        self.unset_h_flag();
        self.unset_c_flag();
        self.unset_n_flag();
    }

    fn or_n(&mut self){
        let val = self.fetch_byte();
        self.cpu.a = self.cpu.a | val;
        if self.cpu.a == 0 {
            self.set_z_flag();
        }
        else {
            self.unset_z_flag();
        }
        self.unset_h_flag();
        self.unset_c_flag();
        self.unset_n_flag();
    }

    // compare

    fn cp_n(&mut self){
        let old_value = self.cpu.a;
        let cmp_val = self.fetch_byte();
        let (result, overflow) = self.cpu.a.overflowing_sub(cmp_val);   
        self.set_flags(if result == 0 {'y'} else {'n'}, if (old_value & 0x0F) < (cmp_val & 0x0f) {'y'} else {'n'}, if (old_value & 0x0F) < (cmp_val & 0x0F) {'y'} else {'n'}, if overflow {'y'} else {'n'})
    }

    fn cp_r_n(&mut self, reg: char) {
        let old_value = self.cpu.a;
        let curr_reg = self.get_reg(reg);
        let curr = *curr_reg;
         
        let (result, overflow) = self.cpu.a.overflowing_sub(curr);
        if result == 0 {
            self.set_z_flag();
        }
        else {
            self.unset_z_flag();
        }
        self.set_n_flag();
        if (old_value & 0x0F) < (curr & 0x0F) {
            self.set_h_flag();
        }
        else {
            self.unset_h_flag();
        }
        if overflow {
            self.set_c_flag();
        }
        else {
            self.unset_c_flag();
        }
    }
    
    fn cp_r_hl(&mut self) {
        let old_value = self.cpu.a;
        let total_byte = (self.cpu.h as u16) << 8 | (self.cpu.l as u16);
        let val = self.memory[total_byte as usize];
        let (result, overflow) = self.cpu.a.overflowing_sub(val);
        if result == 0 {
            self.set_z_flag();
        }
        else {
            self.unset_z_flag();
        }
        self.set_n_flag();
        if (old_value & 0x0F) < (val & 0x0F) {
            self.set_h_flag();
        }
        else {
            self.unset_h_flag();
        }
        if overflow {
            self.set_c_flag();
        }
        else {
            self.unset_c_flag();
        }
    }

    
}


impl Cpu {
    pub fn new() -> Self {
        Cpu {
            a: 0, b: 0, c: 0, d: 0, e: 0, h: 0, l: 0, 
            sp: 0, pc: 0, f: 0, halted: false, ime: false,
        }
    }
}