use std::env;

fn main() {
    let rom_file_path = env::args().nth(1).expect("No ROM file passed as argument!");
    println!("ROM file path {}", rom_file_path);
}
