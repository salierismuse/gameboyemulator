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
}

pub struct Gameboy {
    pub cpu: Cpu,
    pub memory: [u8; 65536],
}

impl Gameboy {
    pub fn step(&mut self) {
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
            //loads
            0x78 => self.ld_a_b(),
            0x3E => self.ld_a_n(),
            0xFA => self.ld_a_nn(),
            //jumps
            0xC3 => self.jp_nn(),
            0xC2 => self.jp_nz_nn(),
            0xCA => self.jp_z_nn(),
            0xD2 => self.jp_nc_nn(),
            0xDA => self.jp_c_nn(),
            //adds
            0x87 => self.add_a_a(),
            0x80 => self.add_a_b(),
            //subs
            0x9F => self.sub_a_a(),
            0x98 => self.sub_a_b(),
            //AND
            0xA7 => self.and_a_gen('a'),
            0xA0 =>self.and_a_gen('b'),
            0xA1 => self.and_a_gen('c'),
            0xA2 =>self.and_a_gen('d'),
            0xA3 => self.and_a_gen('e'),
            0xA4 =>self.and_a_gen('h'),
            0xA5 =>self.and_a_gen('l'),
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

    // lds
    fn ld_a_b(&mut self) {
        self.cpu.a = self.cpu.b;
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


    // fn jp_nz_nn(&mut self) {


    // }

}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            a: 0, b: 0, c: 0, d: 0, e: 0, h: 0, l: 0, 
            sp: 0, pc: 0, f: 0,
        }
    }
}
