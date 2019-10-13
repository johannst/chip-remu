extern crate minifb;
use minifb::{Key, Window, WindowOptions};

use pixel_engine::PixelBuffer;

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
        640,
        280,
        WindowOptions {
            borderless: false,
            title: true,
            resize: false,
            scale: minifb::Scale::X1,
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

    let mut fb = pixel_engine::PixelVec::new(640, 280);

    while window.is_open() && !window.is_key_down(Key::Escape) {
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

        let mut draw_dbg = false;
        let mut draw_fb = false;

        match run_mode {
            RunMode::FreeRunning => {
                let now = Instant::now();

                if (now - f500hz_ref) > Duration::from_millis(2) {
                    f500hz_ref = now;
                    cpu.execute(remap_keys(window.get_keys().unwrap_or_default()));
                    draw_dbg = true;
                }

                if (now - f60hz_ref) > Duration::from_millis(16) {
                    f60hz_ref = now;
                    cpu.timer_tick();
                }

                if (now - f30hz_ref) > Duration::from_millis(32) {
                    f30hz_ref = now;
                    draw_fb = true;
                }
            }
            RunMode::Stepping => {
                if (window.is_key_pressed(Key::Space, minifb::KeyRepeat::Yes)) {
                    cpu.execute(remap_keys(window.get_keys().unwrap_or_default()));
                    cpu.timer_tick();

                    draw_dbg = true;
                    draw_fb = true;
                } else {
                    window.update();
                }
            }
        }

        if draw_dbg {
            let mut y = 0;
            for (c, &instr) in cpu.get_next_n_instr(15).iter().enumerate() {
                let disasm = decoder::disassemble(instr).to_ascii_uppercase();

                for x in 0..32 {
                    pixel_engine::draw_pixel_with_scale(
                        &mut fb,
                        4 * gpu::WIDTH + 50 + x * 8,
                        y + c * 8,
                        0x0,
                        pixel_engine::PixelScale::X8,
                    );
                }
                pixel_engine::draw_str(&mut fb, 4 * gpu::WIDTH + 50, y + c * 8, disasm.as_str());
            }

            y = 16 * 8;
            for (c, state) in cpu.dump_to_vec_str().iter().enumerate() {
                for x in 0..32 {
                    pixel_engine::draw_pixel_with_scale(
                        &mut fb,
                        4 * gpu::WIDTH + 50 + x * 8,
                        y + c * 8,
                        0x0,
                        pixel_engine::PixelScale::X8,
                    );
                }
                pixel_engine::draw_str(&mut fb, 4 * gpu::WIDTH + 50, y + 8 * c, state.as_str());
            }
        }

        if draw_fb {
            for y in 0..gpu::HEIGHT {
                for x in 0..gpu::WIDTH {
                    let pixel = cpu.get_fb()[y * gpu::WIDTH + x] as u32 * 0x00ffffff;
                    pixel_engine::draw_pixel_with_scale(
                        &mut fb,
                        x * 4,
                        y * 4,
                        pixel,
                        pixel_engine::PixelScale::X4,
                    );
                }
            }
            window.update_with_buffer(fb.buffer()).unwrap();
        }
    }
}
