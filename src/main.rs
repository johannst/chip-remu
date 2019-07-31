use std::env;
use std::fs::File;
use std::io::Read;

mod decoder;

fn main() {
    let rom = env::args().nth(1).expect("No ROM file passed as argument!");
    println!("[+] using ROM file: {}", rom);
    let mut rom = File::open(&rom).expect("Failed to open ROM file!");

    let mut rom_data = Vec::new();
    let rom_len = rom
        .read_to_end(&mut rom_data)
        .expect("Failed to read ROM file!");
    println!("[+] read ROM {}bytes", rom_len);

    for word in rom_data.chunks(2) {
        if word.len() == 2 {
            let instr: u16 = u16::from_be_bytes([word[0], word[1]]);

            decoder::decode(instr);
        } else {
            println!("[+] ingoring malformed instruction {:x?}", word);
        }
    }
}
