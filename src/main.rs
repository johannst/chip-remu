extern crate minifb;
use minifb::{Key, Window, WindowOptions};

use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::time::{Duration, Instant};

mod cpu;
mod decoder;
mod gpu;
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
    //let rom_data = load_rom("./roms/demos/Maze_David_Winter_199x.ch8");
    let rom_data = load_rom("./roms/demos/Particle_Demo_zeroZshadow_2008.ch8");

    let mut cpu = cpu::Cpu::new(memory::Memory::new(), gpu::Gpu::new());
    cpu.load_rom(&rom_data);

    let mut window = Window::new(
        "CHIP-8 - ESC to exit",
        gpu::WIDTH,
        gpu::HEIGHT,
        WindowOptions {
            borderless: false,
            title: true,
            resize: false,
            scale: minifb::Scale::X8,
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.set_position(64, 64);

    let mut f500hz_ref = Instant::now();
    let mut f60hz_ref = Instant::now();
    let mut f30hz_ref = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let now = Instant::now();

        if (now - f500hz_ref) > Duration::from_millis(2) {
            cpu.execute();
            f500hz_ref = now;
        }

        if (now - f60hz_ref) > Duration::from_millis(16) {
            // cpu timer tick
            f60hz_ref = now;
        }

        if (now - f30hz_ref) > Duration::from_millis(32) {
            f30hz_ref = now;
            // expensive copy, could be cleaned up
            let fb: Vec<u32> = cpu
                .get_fb()
                .iter()
                .map(|&pixel| 0x00ffffff * pixel as u32)
                .collect();
            window.update_with_buffer(&fb).unwrap();
        }
    }
}
