#[derive(PartialEq, Debug)]
pub enum Instruction {
    ClearDisplay,
    Return,
    Jump(u16),
    JumpV0Addr(u16),
    Call(u16),
    SkipEqVxByte(usize, u8),
    SkipEqVxVy(usize, usize),
    SkipKeyPressedVx(usize),
    SkipKeyNotPressedVx(usize),
    SkipNeqVxByte(usize, u8),
    SkipNeqVxVy(usize, usize),
    LoadRegsVx(usize),
    StoreRegsVx(usize),
    LoadBVx(usize),
    LoadDTVx(usize),
    LoadIAddr(u16),
    LoadSTVx(usize),
    LoadSpriteAddrVx(usize),
    LoadVxByte(usize, u8),
    LoadVxDT(usize),
    LoadVxKey(usize),
    LoadVxVy(usize, usize),
    AddIVx(usize),
    AddVxByte(usize, u8),
    AddVxVy(usize, usize),
    SubVxVy(usize, usize),
    SubnVxVy(usize, usize),
    XorVxVy(usize, usize),
    AndVxVy(usize, usize),
    OrVxVy(usize, usize),
    ShlVxby1(usize),
    ShrVxby1(usize),
    RandVxAndByte(usize, u8),
    DisplaySpriteVxVyNibble(usize, usize, u8),
}

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

pub fn decode(instr: u16) -> Option<Instruction> {
    match CHIP8_INSTRUCTIONS
        .iter()
        .find(|i| (instr & i.mask) == i.opcode)
    {
        Some(InstructionCode { opcode, mask: _ }) => {
            // field access helper
            let vx = ((instr & 0x0f00) >> 8) as usize;
            let vy = ((instr & 0x00f0) >> 4) as usize;
            let nnn = instr & 0x0fff;
            let nn = (instr & 0x00ff) as u8;
            let n = (instr & 0x000f) as u8;

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
                0xf055 => StoreRegsVx(vx),
                0xf065 => LoadRegsVx(vx),
                _ => unreachable!(),
            };
            Some(instr)
        }
        None => {
            eprintln!("Failed to decode, unknown instruction (0x{:04x})", instr);
            None
        }
    }
}

#[cfg(test)]
mod unittest {
    use super::*;

    #[test]
    fn test_no_arg_instr() {
        assert_eq!(Some(Instruction::ClearDisplay), decode(0x00e0));
        assert_eq!(Some(Instruction::Return), decode(0x00ee));
    }

    #[test]
    fn test_addr_instr() {
        assert_eq!(Some(Instruction::Jump(0x123)), decode(0x1123));
        assert_eq!(Some(Instruction::Call(0xabc)), decode(0x2abc));
        assert_eq!(Some(Instruction::LoadIAddr(0x777)), decode(0xa777));
        assert_eq!(Some(Instruction::JumpV0Addr(0x987)), decode(0xb987));
    }

    #[test]
    fn test_reg_byte_instr() {
        assert_eq!(Some(Instruction::SkipEqVxByte(1, 0xff)), decode(0x31ff));
        assert_eq!(Some(Instruction::SkipNeqVxByte(2, 0xee)), decode(0x42ee));
        assert_eq!(Some(Instruction::LoadVxByte(4, 0xcc)), decode(0x64cc));
        assert_eq!(Some(Instruction::AddVxByte(5, 0xbb)), decode(0x75bb));
        assert_eq!(Some(Instruction::RandVxAndByte(6, 0xaa)), decode(0xc6aa));
    }

    #[test]
    fn test_reg_reg_instr() {
        assert_eq!(Some(Instruction::SkipEqVxVy(1, 2)), decode(0x5120));
        assert_eq!(Some(Instruction::LoadVxVy(1, 2)), decode(0x8120));
        assert_eq!(Some(Instruction::OrVxVy(3, 4)), decode(0x8341));
        assert_eq!(Some(Instruction::AndVxVy(3, 4)), decode(0x8342));
        assert_eq!(Some(Instruction::XorVxVy(5, 6)), decode(0x8563));
        assert_eq!(Some(Instruction::AddVxVy(5, 6)), decode(0x8564));
        assert_eq!(Some(Instruction::SubVxVy(7, 8)), decode(0x8785));
        assert_eq!(Some(Instruction::SubnVxVy(0xa, 0xa)), decode(0x8aa7));
        assert_eq!(Some(Instruction::SkipNeqVxVy(0xf, 0xf)), decode(0x9ff0));
    }

    #[test]
    fn test_reg_reg_nibble_instr() {
        assert_eq!(
            Some(Instruction::DisplaySpriteVxVyNibble(0xa, 0xb, 0xc)),
            decode(0xdabc)
        );
    }

    #[test]
    fn test_reg_instr() {
        assert_eq!(Some(Instruction::ShrVxby1(0)), decode(0x8006));
        assert_eq!(Some(Instruction::ShlVxby1(1)), decode(0x810e));
        assert_eq!(Some(Instruction::SkipKeyPressedVx(2)), decode(0xe29e));
        assert_eq!(Some(Instruction::SkipKeyNotPressedVx(3)), decode(0xe3a1));
        assert_eq!(Some(Instruction::LoadVxDT(4)), decode(0xf407));
        assert_eq!(Some(Instruction::LoadVxKey(5)), decode(0xf50a));
        assert_eq!(Some(Instruction::LoadDTVx(6)), decode(0xf615));
        assert_eq!(Some(Instruction::LoadSTVx(7)), decode(0xf718));
        assert_eq!(Some(Instruction::AddIVx(8)), decode(0xf81e));
        assert_eq!(Some(Instruction::LoadSpriteAddrVx(9)), decode(0xf929));
        assert_eq!(Some(Instruction::LoadBVx(0xa)), decode(0xfa33));
        assert_eq!(Some(Instruction::StoreRegsVx(0xc)), decode(0xfc55));
        assert_eq!(Some(Instruction::LoadRegsVx(0xb)), decode(0xfb65));
    }

    #[test]
    fn test_unknown_nistr() {
        assert_eq!(None, decode(0xf00d));
    }
}
