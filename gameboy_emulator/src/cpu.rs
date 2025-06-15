const ZERO_FLAG: u8 = 0b10000000;
const CARRY_FLAG: u8 = 0b00010000;


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
       0xFA => self.ld_a_nn(),
       
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
       
       //adds
       0x87 => self.add_a_a(),
       0x80 => self.add_a_b(),
       
       //subs
       0x9F => self.sub_a_a(),
       0x98 => self.sub_a_b(),
       
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
       
       _ => unimplemented!("nothing here yet"),
   }
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

    // lds

    //ld x, n, immediate
    // add opcodes
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
    //add opcodes
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
    fn add_a_a(&mut self) {
        let (result, overflow) = self.cpu.a.overflowing_add(self.cpu.a);
        self.cpu.a = result;
        if self.cpu.a == 0 {
            self.cpu.f |= 0b10000000;
        }
        else{
            self.cpu.f &= !0b10000000;
        }

        if overflow == true {
            self.cpu.f |= 0b00010000;
        }
        else {
            self.cpu.f &= !0b00010000;
        }
        self.cpu.f &= !0b01000000;

    }

    fn add_a_b(&mut self) {
        let (result, overflow) = self.cpu.a.overflowing_add(self.cpu.b);
        self.cpu.a = result;
        if self.cpu.a == 0 {
            self.cpu.f |= 0b10000000;
        }
        else{
            self.cpu.f &= !0b10000000;
        }

        if overflow == true {
            self.cpu.f |= 0b00010000;
        }
        else {
            self.cpu.f &= !0b00010000;
        }
        self.cpu.f &= !0b01000000;

    }

    //sub functions

    fn sub_a_a(&mut self) {
        let (result, overflow) = self.cpu.a.overflowing_sub(self.cpu.a);
        self.cpu.f |= 0b01000000;
        self.cpu.a = result;
        if (self.cpu.a == 0)
        {
            self.cpu.f |= 0b10000000;
        }
        else {
            self.cpu.f &= !0b10000000
        }
        if (overflow) {
            self.cpu.f |= 0b00010000;
        }
        else {
            self.cpu.f &= !0b00010000;
        }

    }

    fn sub_a_b(&mut self) {
        let (result, overflow) = self.cpu.a.overflowing_sub(self.cpu.b);
        self.cpu.f |= 0b01000000;
        self.cpu.a = result;
        if (self.cpu.a == 0)
        {
            self.cpu.f |= 0b10000000;
        }
        else {
            self.cpu.f &= !0b10000000
        }
        if (overflow) {
            self.cpu.f |= 0b00010000;
        }
        else {
            self.cpu.f &= !0b00010000;
        }

    }
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            a: 0, b: 0, c: 0, d: 0, e: 0, h: 0, l: 0, 
            sp: 0, pc: 0, f: 0, halted: true, ime: false,
        }
    }
}