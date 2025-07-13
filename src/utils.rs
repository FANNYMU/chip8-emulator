use std::{fs::File, io::Read};

pub fn load_rom_file(path: &str) -> Vec<u8> {
    let mut f = File::open(path).expect("ROM not found");
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).expect("Error reading ROM");
    buffer
}