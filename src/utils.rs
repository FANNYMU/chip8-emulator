use std::{fs::File, io::Read, path::Path};

pub fn load_rom_file(path: &str) -> Vec<u8> {
    if !Path::new(path).exists() {
        panic!("ROM file not found: {}", path);
    }
    
    let mut f = File::open(path)
        .unwrap_or_else(|e| panic!("Failed to open ROM {}: {}", path, e));
    
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)
        .unwrap_or_else(|e| panic!("Failed to read ROM {}: {}", path, e));
    
    println!("ROM loaded: {} bytes", buffer.len());
    buffer
}
