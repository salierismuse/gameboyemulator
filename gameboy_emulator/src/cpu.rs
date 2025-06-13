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
            0x78 => self.cpu.ld_a_b(),
            _ => unimplemented!("nothing here yet"),
        }
    }


}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            a: 0, b: 0, c: 0, d: 0, e: 0, h: 0, l: 0, 
            sp: 0, pc: 0, f: 0,
        }
    }

    fn ld_a_b(&mut self) {
        self.a = self.b;
    }
    
}
