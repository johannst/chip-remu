use super::decoder;
use super::instruction;
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
    prev_PC: u16,

    ram: memory::Memory,
}

impl Cpu {
    pub fn new(ram: memory::Memory) -> Cpu {
        Cpu {
            V: [0; 16],
            I: 0x0000,
            DT: 0x00,
            ST: 0x00,
            PC: PROGRAM_START,
            SP: 0x00,
            prev_PC: 0,
            ram: ram,
        }
    }

    pub fn load_rom(&mut self, data: &[u8]) {
        self.ram.load(PROGRAM_START, &data);
    }

    pub fn get_pc(&self) -> u16 {
        self.PC
    }

    pub fn execute(&mut self) {
        use instruction::Instruction::*;

        let instr = match decoder::decode(u16::from_be_bytes([
            self.ram.read_byte(self.PC),
            self.ram.read_byte(self.PC + 1),
        ])) {
            Some(instr) => instr,
            None => panic!("UNKNOWN INSTRUCTION"),
        };

        if self.PC == self.prev_PC {
            panic!("stuck!");
        }
        self.prev_PC = self.PC;
        self.PC += 2;

        match instr {
            LoadIAddr(addr) => {
                self.I = addr;
            }
            RandVxAndByte(v, byte) => {
                self.V[v] = rand::random::<u8>() & byte;
            }
            SkipEqVxByte(v, byte) => {
                if self.V[v] == byte {
                    self.PC += 2;
                }
            }
            _ => {
                unimplemented!();
            }
        }
    }

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
