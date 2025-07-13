use minifb::{Key, Window, WindowOptions};

pub const WIDTH: usize = 64;  
pub const HEIGHT: usize = 32; 
pub const SCALE: usize = 10; 


mod chip8;
mod utils;

use chip8::Chip8;
use utils::load_rom_file;

fn main() {
    let display_width = WIDTH * SCALE;
    let display_height = HEIGHT * SCALE;

    let mut window = Window::new(
        "Chip-8 Emulator",
        display_width,
        display_height,
        WindowOptions::default()
    ).unwrap_or_else(|e| panic!("Window error: {}", e));

    window.limit_update_rate(Some(std::time::Duration::from_millis(16))); // ~60 FPS

    let rom = load_rom_file("roms/Space_Invaders.ch8");
    let mut chip8 = Chip8::new();
    chip8.load_rom(&rom);

    let mut buffer: Vec<u32> = vec![0; display_width * display_height];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        chip8.set_keys(&window);
        chip8.emulate_cycle();

        if chip8.draw_flag {
            // Pre-calculate colors
            const WHITE: u32 = 0xFFFFFF;
            const BLACK: u32 = 0x000000;
            
            for (i, &pixel) in chip8.gfx.iter().enumerate() {
                let color = if pixel != 0 { WHITE } else { BLACK };
                let chip8_x = i % WIDTH;
                let chip8_y = i / WIDTH;
                
                // Calculate base position in scaled buffer
                let base_x = chip8_x * SCALE;
                let base_y = chip8_y * SCALE;
                let base_index = base_y * display_width + base_x;
                
                // Fill SCALE x SCALE block efficiently
                for dy in 0..SCALE {
                    let row_start = base_index + dy * display_width;
                    let row_end = row_start + SCALE;
                    if row_end <= buffer.len() {
                        buffer[row_start..row_end].fill(color);
                    }
                }
            }
            chip8.draw_flag = false;
        }


        window.update_with_buffer(&buffer, display_width, display_height).unwrap();
    }
}