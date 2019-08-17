use std::fs::File;
use std::io::Read;
use std::path::Path;

mod cpu;
mod decoder;
mod instruction;
use cpu::Cpu;
mod memory;
use memory::Memory;

fn execute_cpu_instruction(mem: &mut Memory, cpu: &mut Cpu) {
    let pc = cpu.get_pc();
    let instr_raw: u16 = u16::from_be_bytes([mem.read_byte(pc), mem.read_byte(pc + 1)]);

    if let Some(instr) = decoder::decode(instr_raw) {
        println!("@0x{:04x} {:x}:{:?}", pc, instr_raw, instr);
        cpu.execute(instr);
        cpu.dump();
    }
}

fn load_rom<P: AsRef<Path>>(path: P) -> Vec<u8> {
    println!("[+] using ROM file: {}", path.as_ref().to_str().unwrap());
    let mut rom = File::open(&path).expect("Failed to open ROM file!");

    let mut rom_data = Vec::new();
    let rom_len = rom
        .read_to_end(&mut rom_data)
        .expect("Failed to read ROM file!");
    println!("[+] read ROM {}bytes", rom_len);
    rom_data
}

fn main() {
    let rom_data = load_rom("./roms/demos/Maze_David_Winter_199x.ch8");

    let mut mem = Memory::new();

    let mut cpu = cpu::Cpu::new();
    cpu.load_rom(&mut mem, &rom_data);

    //mem.dump_range(0x100, 0x200);

    for _ in 1..10 {
        execute_cpu_instruction(&mut mem, &mut cpu);
    }
}
