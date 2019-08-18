use std::fs::File;
use std::io::Read;
use std::path::Path;

mod cpu;
mod decoder;
mod instruction;
mod memory;

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

    let mut cpu = cpu::Cpu::new(memory::Memory::new());
    cpu.load_rom(&rom_data);

    loop {
        cpu.execute();
    }
}
