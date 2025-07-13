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

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let rom = load_rom_file("roms/Space_Invaders.ch8");
    let mut chip8 = Chip8::new();
    chip8.load_rom(&rom);

    let mut buffer: Vec<u32> = vec![0; display_width * display_height];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        chip8.set_keys(&window);
        chip8.emulate_cycle();

        if chip8.draw_flag {
            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    let pixel = if chip8.gfx[y * WIDTH + x] != 0 { 0xFFFFFF } else { 0x000000 };

                    for sy in 0..SCALE {
                        for sx in 0..SCALE {
                            let scaled_x = x * SCALE + sx;
                            let scaled_y = y * SCALE + sy;
                            let index = scaled_y * display_width + scaled_x;

                            if index < buffer.len() {
                                buffer[index] = pixel;
                            }
                        }
                    }
                }
            }
            chip8.draw_flag = false;
        }

        window.update_with_buffer(&buffer, display_width, display_height).unwrap();
    }
}