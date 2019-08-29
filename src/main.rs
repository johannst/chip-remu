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

fn remap_keys(keys: Vec<Key>) -> Vec<u8> {
    keys.iter()
        .filter_map(|key| match key {
            Key::X => Some(0x0),
            Key::Key1 => Some(0x1),
            Key::Key2 => Some(0x2),
            Key::Key3 => Some(0x3),
            Key::Q => Some(0x4),
            Key::W => Some(0x5),
            Key::E => Some(0x6),
            Key::A => Some(0x7),
            Key::S => Some(0x8),
            Key::D => Some(0x9),
            Key::Z => Some(0xA),
            Key::C => Some(0xB),
            Key::Key4 => Some(0xC),
            Key::R => Some(0xD),
            Key::F => Some(0xE),
            Key::V => Some(0xF),
            _ => None,
        })
        .collect::<Vec<u8>>()
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
    //let rom_data = load_rom("./roms/demos/Maze_David_Winter_199x.ch8");
    //let rom_data = load_rom("./roms/demos/Particle_Demo_zeroZshadow_2008.ch8");
    //let rom_data = load_rom("./roms/demos/Trip8_Demo_2008_Revival_Studios.ch8");
    //let rom_data = load_rom("./roms/demos/Zero_Demo_zeroZshadow_2007.ch8");
    let rom_data = load_rom("./roms/games/Space_Invaders_David_Winter.ch8");

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
            f500hz_ref = now;
            cpu.execute(remap_keys(window.get_keys().unwrap_or_default()));
        }

        if (now - f60hz_ref) > Duration::from_millis(16) {
            f60hz_ref = now;
            cpu.timer_tick();
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
