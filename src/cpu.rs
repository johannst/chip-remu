use super::decoder;
use super::gpu;
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
    // stack is only used to push/pop PC on call/ret
    SP: Vec<u16>,
    prev_PC: u16,

    ram: memory::Memory,
    gpu: gpu::Gpu,
}

impl Cpu {
    pub fn new(ram: memory::Memory, gpu: gpu::Gpu) -> Cpu {
        Cpu {
            V: [0; 16],
            I: 0x0000,
            DT: 0x00,
            ST: 0x00,
            PC: PROGRAM_START,
            SP: Vec::with_capacity(16),
            prev_PC: 0,
            ram: ram,
            gpu: gpu,
        }
    }

    pub fn get_fb(&self) -> &[bool] {
        self.gpu.as_ref()
    }

    pub fn load_rom(&mut self, data: &[u8]) {
        self.ram.load(PROGRAM_START, &data);
    }

    pub fn execute(&mut self) {
        use instruction::Instruction::*;

        let instr_raw =
            u16::from_be_bytes([self.ram.read_byte(self.PC), self.ram.read_byte(self.PC + 1)]);
        let instr = match decoder::decode(instr_raw) {
            Some(instr) => instr,
            None => panic!("UNKNOWN INSTRUCTION"),
        };

        if self.PC == self.prev_PC {
            return;
        }
        self.prev_PC = self.PC;
        self.PC += 2;

        match instr {
            // ---- Flow Control ---- //
            Return => {
                if let Some(ret) = self.SP.pop() {
                    self.PC = ret;
                } else {
                    panic!("BUG: cpu execute RET instruction while stack is empty!");
                }
            }
            Jump(addr) => {
                self.PC = addr;
            }
            Call(addr) => {
                self.SP.push(self.PC);
                self.PC = addr;
            }
            SkipEqVxByte(v, byte) => {
                if self.V[v] == byte {
                    self.PC += 2;
                }
            }
            SkipEqVxVy(vx, vy) => {
                if self.V[vx] == self.V[vy] {
                    self.PC += 2;
                }
            }
            SkipNeqVxByte(v, byte) => {
                if self.V[v] != byte {
                    self.PC += 2;
                }
            }

            // ---- Load/Store ---- //
            LoadVxByte(v, byte) => {
                self.V[v] = byte;
            }
            LoadVxVy(vx, vy) => {
                self.V[vx] = self.V[vy];
            }
            LoadIAddr(addr) => {
                self.I = addr;
            }
            StoreRegsVx(v) => {
                for vi in 0..v + 1 {
                    self.ram.write_byte(self.I + vi as u16, self.V[vi]);
                }
            }
            LoadRegsVx(v) => {
                for vi in 0..v + 1 {
                    self.V[vi] = self.ram.read_byte(self.I + vi as u16);
                }
            }

            // ---- Arithmetic ---- //
            AddVxByte(v, byte) => {
                // this instruction does not set VF as carry
                self.V[v] = self.V[v].wrapping_add(byte);
            }
            AddVxVy(vx, vy) => {
                self.V[15] = (self.V[vy] > 255 - self.V[vx]) as u8; // VF as carry
                self.V[vx] = self.V[vx].wrapping_add(self.V[vy]);
            }
            AddIVx(v) => {
                self.I += self.V[v] as u16;
            }

            // ---- Bit Operations ---- ///
            AndVxVy(vx, vy) => {
                self.V[vx] &= self.V[vy];
            }
            ShlVxby1(v) => {
                // VF = Vx[7]
                self.V[15] = (self.V[v] & 0x80) as u8;
                self.V[v] <<= 1;
            }
            ShrVxby1(v) => {
                // VF = Vx[0]
                self.V[15] = (self.V[v] & 0x01) as u8;
                self.V[v] >>= 1;
            }

            // ---- Rand ----//
            RandVxAndByte(v, byte) => {
                self.V[v] = rand::random::<u8>() & byte;
            }

            // ---- Display ---- //
            DisplaySpriteVxVyNibble(vx, vy, nbytes) => {
                let lines = nbytes as usize;
                // TODO: get rid of copy (slice into memory)
                let mut sprite = [0u8; 16];
                for line in 0..lines {
                    sprite[line] = self.ram.read_byte(self.I + line as u16);
                }
                self.V[15] = (self.gpu.write_sprite(
                    self.V[vx] as usize,
                    self.V[vy] as usize,
                    &sprite[0..lines],
                ) == gpu::Collision::Collision) as u8;
            }

            _ => {
                println!("unimplemented instruction {:#x} {:?}", instr_raw, instr);
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
        //println!("SP: {:02x}", self.SP);
        println!("-------------------");
    }
}
