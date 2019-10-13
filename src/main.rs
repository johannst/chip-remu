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

fn load_rom_file<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, String> {
    println!("[+] using ROM file: {}", path.as_ref().to_str().unwrap());
    match File::open(&path) {
        Ok(mut file) => {
            let mut rom_data = Vec::new();
            match file.read_to_end(&mut rom_data) {
                Ok(rom_len) => {
                    println!("[+] read ROM {}bytes", rom_len);
                    Ok(rom_data)
                }
                Err(_) => Err("Failed to read tom file!".to_string()),
            }
        }
        Err(_) => Err("Failed to open rom file!".to_string()),
    }
}

fn load_rom_from_disk() -> Result<Vec<u8>, String> {
    match std::env::args().nth(1) {
        Some(p) => load_rom_file(p),
        None => Err(format!("Use as {} <rom>", std::env::args().nth(0).unwrap())),
    }
}

fn main() {
    let rom_data = match load_rom_from_disk() {
        Ok(rom) => rom,
        Err(e) => {
            eprintln!("FAILED: {}", e);
            std::process::exit(1);
        }
    };

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

    let mut f500hz_ref = Instant::now();
    let mut f60hz_ref = Instant::now();
    let mut f30hz_ref = Instant::now();

    #[derive(Debug)]
    enum RunMode {
        FreeRunning,
        Stepping,
    }
    let mut run_mode = RunMode::Stepping;

    println!("[+] RunMode: {:?}", run_mode);
    println!("[+] Change RunMode with 'G' | 'B'");
    println!("    'G': FreeRunning");
    println!("    'B': Stepping");
    println!("[+] In Stepping mode use 'SPACE' to step one instruction");

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update();
        window
            .get_keys_pressed(minifb::KeyRepeat::No)
            .unwrap()
            .iter()
            .for_each(|&k| match k {
                Key::B => {
                    run_mode = RunMode::Stepping;
                    println!("switching RunMode: {:?}", run_mode);
                }
                Key::G => {
                    run_mode = RunMode::FreeRunning;
                    println!("switching RunMode: {:?}", run_mode);
                }
                _ => {}
            });

        match run_mode {
            RunMode::FreeRunning => {
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
            RunMode::Stepping => {
                if (window.is_key_pressed(Key::Space, minifb::KeyRepeat::No)) {
                    cpu.execute(remap_keys(window.get_keys().unwrap_or_default()));
                    cpu.timer_tick();

                    let fb: Vec<u32> = cpu
                        .get_fb()
                        .iter()
                        .map(|&pixel| 0x00ffffff * pixel as u32)
                        .collect();
                    window.update_with_buffer(&fb).unwrap();
                }
            }
        }
    }
}
