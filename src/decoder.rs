struct InstrHelper {
    opcode: u16,
    mask: u16,
}

const INSTRUCTIONS: [InstrHelper; 34] = [
    // Cls
    InstrHelper {
        opcode: 0x00e0,
        mask: 0xffff,
    },
    // Ret
    InstrHelper {
        opcode: 0x00ee,
        mask: 0xffff,
    },
    // Jp_Addr
    InstrHelper {
        opcode: 0x1000,
        mask: 0xf000,
    },
    // Call_Addr
    InstrHelper {
        opcode: 0x2000,
        mask: 0xf000,
    },
    // SE_Vx_Byte
    InstrHelper {
        opcode: 0x3000,
        mask: 0xf000,
    },
    // SNE_Vx_Byte
    InstrHelper {
        opcode: 0x4000,
        mask: 0xf000,
    },
    // SE_Vx_Vy
    InstrHelper {
        opcode: 0x5000,
        mask: 0xf00f,
    },
    // LD_Vx_Byte
    InstrHelper {
        opcode: 0x6000,
        mask: 0xf000,
    },
    // ADD_Vx_Byte
    InstrHelper {
        opcode: 0x7000,
        mask: 0xf000,
    },
    // LD_Vx_Vy
    InstrHelper {
        opcode: 0x8000,
        mask: 0xf00f,
    },
    // OR_Vx_Vy
    InstrHelper {
        opcode: 0x8001,
        mask: 0xf00f,
    },
    // AND_Vx_Vy
    InstrHelper {
        opcode: 0x8002,
        mask: 0xf00f,
    },
    // XOR_Vx_Vy
    InstrHelper {
        opcode: 0x8003,
        mask: 0xf00f,
    },
    // ADD_Vx_Vy
    InstrHelper {
        opcode: 0x8004,
        mask: 0xf00f,
    },
    // SUB_Vx_Vy
    InstrHelper {
        opcode: 0x8005,
        mask: 0xf00f,
    },
    // SHR_Vx_Vy
    InstrHelper {
        opcode: 0x8006,
        mask: 0xf00f,
    },
    // SUBN_Vx_Vy
    InstrHelper {
        opcode: 0x8007,
        mask: 0xf00f,
    },
    // SHL_Vx_Vy
    InstrHelper {
        opcode: 0x800e,
        mask: 0xf00f,
    },
    // SNE_Vx_Vy
    InstrHelper {
        opcode: 0x9000,
        mask: 0xf00f,
    },
    // LD_I_Addr
    InstrHelper {
        opcode: 0xa000,
        mask: 0xf000,
    },
    // JP_V0_Addr
    InstrHelper {
        opcode: 0xb000,
        mask: 0xf000,
    },
    // RND_Vx_Byte
    InstrHelper {
        opcode: 0xc000,
        mask: 0xf000,
    },
    // DRW_Vx_Vy_nibble
    InstrHelper {
        opcode: 0xd000,
        mask: 0xf000,
    },
    // SKP_Vx
    InstrHelper {
        opcode: 0xe09e,
        mask: 0xf0ff,
    },
    // SKNP_Vx
    InstrHelper {
        opcode: 0xe0a1,
        mask: 0xf0ff,
    },
    // LD_Vx_DT
    InstrHelper {
        opcode: 0xf007,
        mask: 0xf0ff,
    },
    // LD_Vx_K
    InstrHelper {
        opcode: 0xf00a,
        mask: 0xf0ff,
    },
    // LD_DT_Vx
    InstrHelper {
        opcode: 0xf015,
        mask: 0xf0ff,
    },
    // LD_ST_Vx
    InstrHelper {
        opcode: 0xf018,
        mask: 0xf0ff,
    },
    // ADD_I_Vx
    InstrHelper {
        opcode: 0xf01e,
        mask: 0xf0ff,
    },
    // LD_F_Vx
    InstrHelper {
        opcode: 0xf029,
        mask: 0xf0ff,
    },
    // LD_B_Vx
    InstrHelper {
        opcode: 0xf033,
        mask: 0xf0ff,
    },
    // LD_I_Vx
    InstrHelper {
        opcode: 0xf055,
        mask: 0xf0ff,
    },
    // LD_Vx_I
    InstrHelper {
        opcode: 0xf065,
        mask: 0xf0ff,
    },
];

pub fn decode(bytes: u16) {
    match INSTRUCTIONS.iter().find(|i| (bytes & i.mask) == i.opcode) {
        Some(InstrHelper { opcode, mask: _ }) => {
            match opcode {
                // Cls - clear display
                0x00e0 => {
                    unimplemented!();
                }
                // Ret - return
                0x00ee => {
                    unimplemented!();
                }
                // Jp_Addr - jump to addr
                0x1000 => {
                    unimplemented!();
                }
                // Call_Addr - call subroutine
                0x2000 => {
                    unimplemented!();
                }
                // SE_Vx_Byte - skip Vx eq byte
                0x3000 => {
                    unimplemented!();
                }
                // SNE_Vx_Byte - skip Vx neq byte
                0x4000 => {
                    unimplemented!();
                }
                // SE_Vx_Vy - skip Vx eq Vy
                0x5000 => {
                    unimplemented!();
                }
                // LD_Vx_Byte - load byte into Vx
                0x6000 => {
                    unimplemented!();
                }
                // ADD_Vx_Byte - add byte to Vx
                0x7000 => {
                    unimplemented!();
                }
                // LD_Vx_Vy - load value of Vy into Vx
                0x8000 => {
                    unimplemented!();
                }
                // OR_Vx_Vy - Vx |= Vy
                0x8001 => {
                    unimplemented!();
                }
                // AND_Vx_Vy - Vx &= Vy
                0x8002 => {
                    unimplemented!();
                }
                // XOR_Vx_Vy - Vx ^= Vy
                0x8003 => {
                    unimplemented!();
                }
                // ADD_Vx_Vy - Vx += Vy; set VF if Vx > 255
                0x8004 => {
                    unimplemented!();
                }
                // SUB_Vx_Vy - Vx -= Vy; set VF if Vx > Vy
                0x8005 => {
                    unimplemented!();
                }
                // SHR_Vx_Vy - Vx >>= 1; set VF if Vx[0] == 1 (before shift)
                0x8006 => {
                    unimplemented!();
                }
                // SUBN_Vx_Vy
                0x8007 => {
                    unimplemented!();
                }
                // SHL_Vx_Vy
                0x800e => {
                    unimplemented!();
                }
                // SNE_Vx_Vy
                0x9000 => {
                    unimplemented!();
                }
                // LD_I_Addr
                0xa000 => {
                    unimplemented!();
                }
                // JP_V0_Addr
                0xb000 => {
                    unimplemented!();
                }
                // RND_Vx_Byte
                0xc000 => {
                    unimplemented!();
                }
                // DRW_Vx_Vy_nibble
                0xd000 => {
                    unimplemented!();
                }
                // SKP_Vx
                0xe09e => {
                    unimplemented!();
                }
                // SKNP_Vx
                0xe0a1 => {
                    unimplemented!();
                }
                // LD_Vx_DT
                0xf007 => {
                    unimplemented!();
                }
                // LD_Vx_K
                0xf00a => {
                    unimplemented!();
                }
                // LD_DT_Vx
                0xf015 => {
                    unimplemented!();
                }
                // LD_ST_Vx
                0xf018 => {
                    unimplemented!();
                }
                // ADD_I_Vx
                0xf01e => {
                    unimplemented!();
                }
                // LD_F_Vx
                0xf029 => {
                    unimplemented!();
                }
                // LD_B_Vx
                0xf033 => {
                    unimplemented!();
                }
                // LD_I_Vx
                0xf055 => {
                    unimplemented!();
                }
                // LD_Vx_I
                0xf065 => {
                    unimplemented!();
                }
                _ => unreachable!(),
            }
        }
        None => eprintln!("Failed to decode, unknown instruction ({:x})", bytes),
    }
}
