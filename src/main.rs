use std::env;
use std::fs::File;
use std::io::Read;

mod cpu;
mod decoder;
mod instruction;
use cpu::Cpu;
mod memory;
use memory::Memory;

fn execute_cpu_instruction(mem: &mut Memory, cpu: &mut Cpu) {
    let pc = cpu.get_pc();
    let instr: u16 = u16::from_be_bytes([mem.read_byte(pc), mem.read_byte(pc + 1)]);

    if let Some(i) = decoder::decode(instr) {
        println!("{:?}", i);
        cpu.execute(instr);
    }
}

fn main() {
    let rom = env::args().nth(1).expect("No ROM file passed as argument!");
    println!("[+] using ROM file: {}", rom);
    let mut rom = File::open(&rom).expect("Failed to open ROM file!");

    let mut rom_data = Vec::new();
    let rom_len = rom
        .read_to_end(&mut rom_data)
        .expect("Failed to read ROM file!");
    let rom_data = rom_data;
    println!("[+] read ROM {}bytes", rom_len);

    let mut mem = Memory::new();

    let mut cpu = cpu::Cpu::new();
    cpu.dump();
    cpu.load_rom(&mut mem, &rom_data);

    mem.dump_range(0x100, 0x200);

    execute_cpu_instruction(&mut mem, &mut cpu);
}
