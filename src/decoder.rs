use super::instruction::Instruction;

struct InstructionCode {
    opcode: u16,
    mask: u16,
}

const CHIP8_INSTRUCTIONS: [InstructionCode; 34] = [
    // Cls
    InstructionCode {
        opcode: 0x00e0,
        mask: 0xffff,
    },
    // Ret
    InstructionCode {
        opcode: 0x00ee,
        mask: 0xffff,
    },
    // Jp_Addr
    InstructionCode {
        opcode: 0x1000,
        mask: 0xf000,
    },
    // Call_Addr
    InstructionCode {
        opcode: 0x2000,
        mask: 0xf000,
    },
    // SE_Vx_Byte
    InstructionCode {
        opcode: 0x3000,
        mask: 0xf000,
    },
    // SNE_Vx_Byte
    InstructionCode {
        opcode: 0x4000,
        mask: 0xf000,
    },
    // SE_Vx_Vy
    InstructionCode {
        opcode: 0x5000,
        mask: 0xf00f,
    },
    // LD_Vx_Byte
    InstructionCode {
        opcode: 0x6000,
        mask: 0xf000,
    },
    // ADD_Vx_Byte
    InstructionCode {
        opcode: 0x7000,
        mask: 0xf000,
    },
    // LD_Vx_Vy
    InstructionCode {
        opcode: 0x8000,
        mask: 0xf00f,
    },
    // OR_Vx_Vy
    InstructionCode {
        opcode: 0x8001,
        mask: 0xf00f,
    },
    // AND_Vx_Vy
    InstructionCode {
        opcode: 0x8002,
        mask: 0xf00f,
    },
    // XOR_Vx_Vy
    InstructionCode {
        opcode: 0x8003,
        mask: 0xf00f,
    },
    // ADD_Vx_Vy
    InstructionCode {
        opcode: 0x8004,
        mask: 0xf00f,
    },
    // SUB_Vx_Vy
    InstructionCode {
        opcode: 0x8005,
        mask: 0xf00f,
    },
    // SHR_Vx_Vy
    InstructionCode {
        opcode: 0x8006,
        mask: 0xf00f,
    },
    // SUBN_Vx_Vy
    InstructionCode {
        opcode: 0x8007,
        mask: 0xf00f,
    },
    // SHL_Vx_Vy
    InstructionCode {
        opcode: 0x800e,
        mask: 0xf00f,
    },
    // SNE_Vx_Vy
    InstructionCode {
        opcode: 0x9000,
        mask: 0xf00f,
    },
    // LD_I_Addr
    InstructionCode {
        opcode: 0xa000,
        mask: 0xf000,
    },
    // JP_V0_Addr
    InstructionCode {
        opcode: 0xb000,
        mask: 0xf000,
    },
    // RND_Vx_Byte
    InstructionCode {
        opcode: 0xc000,
        mask: 0xf000,
    },
    // DRW_Vx_Vy_nibble
    InstructionCode {
        opcode: 0xd000,
        mask: 0xf000,
    },
    // SKP_Vx
    InstructionCode {
        opcode: 0xe09e,
        mask: 0xf0ff,
    },
    // SKNP_Vx
    InstructionCode {
        opcode: 0xe0a1,
        mask: 0xf0ff,
    },
    // LD_Vx_DT
    InstructionCode {
        opcode: 0xf007,
        mask: 0xf0ff,
    },
    // LD_Vx_K
    InstructionCode {
        opcode: 0xf00a,
        mask: 0xf0ff,
    },
    // LD_DT_Vx
    InstructionCode {
        opcode: 0xf015,
        mask: 0xf0ff,
    },
    // LD_ST_Vx
    InstructionCode {
        opcode: 0xf018,
        mask: 0xf0ff,
    },
    // ADD_I_Vx
    InstructionCode {
        opcode: 0xf01e,
        mask: 0xf0ff,
    },
    // LD_F_Vx
    InstructionCode {
        opcode: 0xf029,
        mask: 0xf0ff,
    },
    // LD_B_Vx
    InstructionCode {
        opcode: 0xf033,
        mask: 0xf0ff,
    },
    // LD_I_Vx
    InstructionCode {
        opcode: 0xf055,
        mask: 0xf0ff,
    },
    // LD_Vx_I
    InstructionCode {
        opcode: 0xf065,
        mask: 0xf0ff,
    },
];

pub fn decode(bytes: u16) -> Option<Instruction> {
    match CHIP8_INSTRUCTIONS
        .iter()
        .find(|i| (bytes & i.mask) == i.opcode)
    {
        Some(InstructionCode { opcode, mask: _ }) => {
            // field access helper
            let vx = (bytes & 0x0f00 >> 8) as u8;
            let vy = (bytes & 0x00f0 >> 4) as u8;
            let nnn = bytes & 0x0fff;
            let nn = (bytes & 0x00ff) as u8;
            let n = (bytes & 0x000f) as u8;

            use Instruction::*;
            let instr = match opcode {
                0x00e0 => ClearDisplay,
                0x00ee => Return,
                0x1000 => Jump(nnn),
                0x2000 => Call(nnn),
                0x3000 => SkipEqVxByte(vx, nn),
                0x4000 => SkipNeqVxByte(vx, nn),
                0x5000 => SkipEqVxVy(vx, vy),
                0x6000 => LoadVxByte(vx, nn),
                0x7000 => AddVxByte(vx, nn),
                0x8000 => LoadVxVy(vx, vy),
                0x8001 => OrVxVy(vx, vy),
                0x8002 => AndVxVy(vx, vy),
                0x8003 => XorVxVy(vx, vy),
                0x8004 => AddVxVy(vx, vy),
                0x8005 => SubVxVy(vx, vy),
                0x8006 => ShrVxby1(vx),
                0x8007 => SubnVxVy(vx, vy),
                0x800e => ShlVxby1(vx),
                0x9000 => SkipNeqVxVy(vx, vy),
                0xa000 => LoadIAddr(nnn),
                0xb000 => JumpV0Addr(nnn),
                0xc000 => RandVxAndByte(vx, nn),
                0xd000 => DisplaySpriteVxVyNibble(vx, vy, n),
                0xe09e => SkipKeyPressedVx(vx),
                0xe0a1 => SkipKeyNotPressedVx(vx),
                0xf007 => LoadVxDT(vx),
                0xf00a => LoadVxKey(vx),
                0xf015 => LoadDTVx(vx),
                0xf018 => LoadSTVx(vx),
                0xf01e => AddIVx(vx),
                0xf029 => LoadSpriteAddrVx(vx),
                0xf033 => LoadBVx(vx),
                0xf055 => LoadRegsVx(vx),
                0xf065 => StoreRegsVx(vx),
                _ => unreachable!(),
            };
            Some(instr)
        }
        None => {
            eprintln!("Failed to decode, unknown instruction (0x{:04x})", bytes);
            None
        }
    }
}
