use super::decoder;
use super::gpu;
use super::memory;

const PROGRAM_START: u16 = 0x200;

#[derive(PartialEq)]
enum PCOp {
    Inc,
    Stay,
    SkipNext,
    JumpAddr(u16),
}

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

    pub fn timer_tick(&mut self) {
        if self.DT > 0 {
            self.DT -= 1;
        }
        if self.ST > 0 {
            self.ST -= 1;
        }
    }

    pub fn get_next_n_instr(&self, n: usize) -> std::vec::Vec<u16> {
        let mut instrs = Vec::with_capacity(n * std::mem::size_of::<u16>());
        let mut offset = 0;
        for _ in 0..n {
            instrs.push(u16::from_be_bytes([
                self.ram.read_byte(self.PC + offset),
                self.ram.read_byte(self.PC + 1 + offset),
            ]));
            offset += 2;
        }
        instrs
    }

    pub fn execute(&mut self, keys: Vec<u8>) {
        use decoder::Instruction::*;

        let instr_raw =
            u16::from_be_bytes([self.ram.read_byte(self.PC), self.ram.read_byte(self.PC + 1)]);
        let instr = match decoder::decode(instr_raw) {
            Some(instr) => instr,
            None => panic!("UNKNOWN INSTRUCTION"),
        };

        let mut pc_op = PCOp::Inc;
        match instr {
            // ---- Flow Control ---- //
            Return => {
                if let Some(ret) = self.SP.pop() {
                    pc_op = PCOp::JumpAddr(ret);
                } else {
                    panic!("BUG: cpu execute RET instruction while stack is empty!");
                }
            }
            Jump(addr) => {
                pc_op = PCOp::JumpAddr(addr);
            }
            JumpV0Addr(addr) => {
                pc_op = PCOp::JumpAddr(self.V[0] as u16 + addr);
            }
            Call(addr) => {
                self.SP.push(self.PC + 2); // push addr of next instr
                pc_op = PCOp::JumpAddr(addr);
            }
            SkipEqVxByte(v, byte) => {
                if self.V[v] == byte {
                    pc_op = PCOp::SkipNext;
                }
            }
            SkipNeqVxByte(v, byte) => {
                if self.V[v] != byte {
                    pc_op = PCOp::SkipNext;
                }
            }
            SkipEqVxVy(vx, vy) => {
                if self.V[vx] == self.V[vy] {
                    pc_op = PCOp::SkipNext;
                }
            }
            SkipNeqVxVy(vx, vy) => {
                if self.V[vx] != self.V[vy] {
                    pc_op = PCOp::SkipNext;
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
            LoadBVx(v) => {
                let v = self.V[v];
                self.ram.write_byte(self.I, v / 100);
                self.ram.write_byte(self.I + 1, (v / 10) % 10);
                self.ram.write_byte(self.I + 2, v % 10);
            }
            LoadSpriteAddrVx(v) => {
                self.I = self.V[v] as u16 * 5;
            }

            // ---- Timer ---- //
            LoadDTVx(v) => {
                self.DT = self.V[v];
            }
            LoadVxDT(v) => {
                self.V[v] = self.DT;
            }
            LoadSTVx(v) => {
                self.ST = self.V[v];
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
            SubVxVy(vx, vy) => {
                //If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx.
                self.V[15] = (self.V[vx] > self.V[vy]) as u8; // VF as carry
                self.V[vx] = self.V[vx].wrapping_sub(self.V[vy]);
            }
            SubnVxVy(vx, vy) => {
                // If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.
                self.V[15] = (self.V[vy] > self.V[vx]) as u8; // VF as carry
                self.V[vx] = self.V[vy].wrapping_sub(self.V[vx]);
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
            XorVxVy(vx, vy) => {
                self.V[vx] ^= self.V[vy];
            }
            OrVxVy(vx, vy) => {
                self.V[vx] |= self.V[vy];
            }

            // ---- Rand ----//
            RandVxAndByte(v, byte) => {
                self.V[v] = rand::random::<u8>() & byte;
            }

            // ---- Display ---- //
            ClearDisplay => {
                self.gpu.clear();
            }
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

            // ---- Key Input ---- //
            SkipKeyPressedVx(v) => {
                if keys.contains(&self.V[v]) == true {
                    pc_op = PCOp::SkipNext;
                }
            }
            SkipKeyNotPressedVx(v) => {
                if keys.contains(&self.V[v]) == false {
                    pc_op = PCOp::SkipNext;
                }
            }
            LoadVxKey(v) => {
                if keys.is_empty() {
                    pc_op = PCOp::Stay;
                } else {
                    self.V[v] = keys[0];
                }
            }
        }

        if self.PC == self.prev_PC {
            panic!("BUG: cpu stuck at {:04x}", self.PC);
        }
        if pc_op != PCOp::Stay {
            self.prev_PC = self.PC;
        }

        match pc_op {
            PCOp::Inc => {
                self.PC += 2;
            }
            PCOp::SkipNext => {
                self.PC += 4;
            }
            PCOp::Stay => {}
            PCOp::JumpAddr(a) => {
                self.PC = a;
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
        for (i, val) in self.SP.iter().rev().enumerate() {
            if i == 0 {
                println!("SP: {:04x}", val);
            } else {
                println!("    {:04x}", val);
            }
        }
        println!("-------------------");
    }

    pub fn dump_to_vec_str(&self) -> std::vec::Vec<String> {
        let mut state = Vec::new();
        state.push(format!("---- CPU STATE ----"));
        for i in 0..4 {
            let i = 4 * i;
            state.push(format!(
                "V{:X}: {:02X}   V{:X}: {:02X}  V{:X}: {:02X}  V{:X}: {:02X}",
                i,
                self.V[i],
                i + 1,
                self.V[i + 1],
                i + 2,
                self.V[i + 2],
                i + 3,
                self.V[i + 3]
            ));
        }
        state.push(format!("DT: {:02X}   ST: {:02X}", self.DT, self.ST));
        state.push(format!("I : {:04X}", self.I));
        state.push(format!("PC: {:04X}", self.PC));
        for (i, val) in self.SP.iter().rev().enumerate() {
            if i == 0 {
                state.push(format!("SP: {:04X}", val));
            } else {
                state.push(format!("    {:04X}", val));
            }
        }
        state
    }
}
