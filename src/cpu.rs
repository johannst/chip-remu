use super::memory;

const PROGRAM_START: u16 = 0x200;

#[allow(non_snake_case)]
pub struct Cpu {
    V: [u8; 16],
    I: u16,
    DT: u8,
    ST: u8,
    PC: u16,
    SP: u8,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            V: [0; 16],
            I: 0x0000,
            DT: 0x00,
            ST: 0x00,
            PC: PROGRAM_START,
            SP: 0x00,
        }
    }

    pub fn load_rom(&self, mem: &mut memory::Memory, data : &[u8]) {
        mem.load(PROGRAM_START, &data);
    }

    pub fn get_pc(&self) -> u16 {
        self.PC
    }

    pub fn execute(&mut self, instruction: u16) {}

    pub fn dump(&self) {
        println!("---- CPU state ----");
        for l in 0..4 {
            for (i, v) in self.V[l..l + 4].iter().enumerate() {
                print!("V{:X}: {:02x}    ", i + 4 * l, v);
            }
            println!();
        }
        println!("DT: {:02x}    ST: {:02x}", self.DT, self.ST);
        println!("I : {:04x}", self.I);
        println!("PC: {:04x}", self.PC);
        println!("SP: {:02x}", self.SP);
        println!("-------------------");
    }
}
