use minifb::{Key, Window};

use crate::{HEIGHT, WIDTH};

pub struct Chip8 {
    memory: [u8; 4096],      
    v: [u8; 16],       
    i: u16,                    
    pc: u16,                   
    pub gfx: [u8; WIDTH * HEIGHT], 
    delay_timer: u8,
    sound_timer: u8,
    stack: [u16; 16],
    sp: u8,
    keypad: [bool; 16],
    pub draw_flag: bool,       
}

impl Chip8 {
    pub fn new() -> Self {
        let mut chip8 = Self {
            memory: [0; 4096],
            v: [0; 16],
            i: 0,
            pc: 0x200,
            gfx: [0; WIDTH * HEIGHT],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            sp: 0,
            keypad: [false; 16],
            draw_flag: false,
        };
        
        chip8.load_fontset();
        chip8
    }

    pub fn load_fontset(&mut self) {
        let fontset: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80  // F
        ];

        for (i, &byte) in fontset.iter().enumerate() {
            self.memory[0x50 + i] = byte;
        }
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        for (i, byte) in rom.iter().enumerate() {
            if 0x200 + i < 4096 {
                self.memory[0x200 + i] = *byte;
            }
        }
    }

    pub fn emulate_cycle(&mut self) {
        // Fetch opcode
        let opcode = (self.memory[self.pc as usize] as u16) << 8 | self.memory[(self.pc + 1) as usize] as u16;
        
        // Decode and execute
        self.execute_opcode(opcode);
        
        // Update timers
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        
        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                println!("BEEP!");
            }
            self.sound_timer -= 1;
        }
    }

    fn execute_opcode(&mut self, opcode: u16) {
        match opcode & 0xF000 {
            0x0000 => {
                match opcode & 0x00FF {
                    0x00E0 => {
                        // Clear screen
                        self.gfx = [0; WIDTH * HEIGHT];
                        self.draw_flag = true;
                        self.pc += 2;
                    }
                    0x00EE => {
                        // Return from subroutine
                        self.sp -= 1;
                        self.pc = self.stack[self.sp as usize];
                        self.pc += 2;
                    }
                    _ => {
                        println!("Unknown opcode: 0x{:X}", opcode);
                        self.pc += 2;
                    }
                }
            }
            0x1000 => {
                // Jump to address
                self.pc = opcode & 0x0FFF;
            }
            0x2000 => {
                // Call subroutine
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = opcode & 0x0FFF;
            }
            0x3000 => {
                // Skip next instruction if VX == NN
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let nn = (opcode & 0x00FF) as u8;
                if self.v[x] == nn {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            0x4000 => {
                // Skip next instruction if VX != NN
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let nn = (opcode & 0x00FF) as u8;
                if self.v[x] != nn {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            0x5000 => {
                // Skip next instruction if VX == VY
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let y = ((opcode & 0x00F0) >> 4) as usize;
                if self.v[x] == self.v[y] {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            0x6000 => {
                // Set VX = NN
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let nn = (opcode & 0x00FF) as u8;
                self.v[x] = nn;
                self.pc += 2;
            }
            0x7000 => {
                // Add NN to VX
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let nn = (opcode & 0x00FF) as u8;
                self.v[x] = self.v[x].wrapping_add(nn);
                self.pc += 2;
            }
            0x8000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let y = ((opcode & 0x00F0) >> 4) as usize;
                
                match opcode & 0x000F {
                    0x0000 => self.v[x] = self.v[y],
                    0x0001 => self.v[x] |= self.v[y],
                    0x0002 => self.v[x] &= self.v[y],
                    0x0003 => self.v[x] ^= self.v[y],
                    0x0004 => {
                        let sum = self.v[x] as u16 + self.v[y] as u16;
                        self.v[0xF] = if sum > 255 { 1 } else { 0 };
                        self.v[x] = sum as u8;
                    }
                    0x0005 => {
                        self.v[0xF] = if self.v[x] > self.v[y] { 1 } else { 0 };
                        self.v[x] = self.v[x].wrapping_sub(self.v[y]);
                    }
                    0x0006 => {
                        self.v[0xF] = self.v[x] & 0x1;
                        self.v[x] >>= 1;
                    }
                    0x0007 => {
                        self.v[0xF] = if self.v[y] > self.v[x] { 1 } else { 0 };
                        self.v[x] = self.v[y].wrapping_sub(self.v[x]);
                    }
                    0x000E => {
                        self.v[0xF] = (self.v[x] & 0x80) >> 7;
                        self.v[x] <<= 1;
                    }
                    _ => {
                        println!("Unknown opcode: 0x{:X}", opcode);
                    }
                }
                self.pc += 2;
            }
            0x9000 => {
                // Skip next instruction if VX != VY
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let y = ((opcode & 0x00F0) >> 4) as usize;
                if self.v[x] != self.v[y] {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            0xA000 => {
                // Set I = NNN
                self.i = opcode & 0x0FFF;
                self.pc += 2;
            }
            0xB000 => {
                // Jump to NNN + V0
                self.pc = (opcode & 0x0FFF) + self.v[0] as u16;
            }
            0xC000 => {
                // Set VX = random byte AND NN
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let nn = (opcode & 0x00FF) as u8;
                self.v[x] = rand::random::<u8>() & nn;
                self.pc += 2;
            }
            0xD000 => {
                // Draw sprite at VX, VY with height N
                let x = self.v[((opcode & 0x0F00) >> 8) as usize] as usize;
                let y = self.v[((opcode & 0x00F0) >> 4) as usize] as usize;
                let height = (opcode & 0x000F) as usize;
                
                self.v[0xF] = 0;
                for yline in 0..height {
                    let pixel = self.memory[(self.i + yline as u16) as usize];
                    for xline in 0..8 {
                        if (pixel & (0x80 >> xline)) != 0 {
                            let px = (x + xline) % WIDTH;
                            let py = (y + yline) % HEIGHT;
                            let index = py * WIDTH + px;
                            
                            if self.gfx[index] == 1 {
                                self.v[0xF] = 1;
                            }
                            self.gfx[index] ^= 1;
                        }
                    }
                }
                self.draw_flag = true;
                self.pc += 2;
            }
            0xE000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                match opcode & 0x00FF {
                    0x009E => {
                        // Skip next instruction if key VX is pressed
                        if self.keypad[self.v[x] as usize] {
                            self.pc += 4;
                        } else {
                            self.pc += 2;
                        }
                    }
                    0x00A1 => {
                        // Skip next instruction if key VX is not pressed
                        if !self.keypad[self.v[x] as usize] {
                            self.pc += 4;
                        } else {
                            self.pc += 2;
                        }
                    }
                    _ => {
                        println!("Unknown opcode: 0x{:X}", opcode);
                        self.pc += 2;
                    }
                }
            }
            0xF000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                match opcode & 0x00FF {
                    0x0007 => {
                        // Set VX = delay timer
                        self.v[x] = self.delay_timer;
                        self.pc += 2;
                    }
                    0x000A => {
                        // Wait for key press, store in VX
                        let mut key_pressed = false;
                        for (i, &pressed) in self.keypad.iter().enumerate() {
                            if pressed {
                                self.v[x] = i as u8;
                                key_pressed = true;
                                break;
                            }
                        }
                        if key_pressed {
                            self.pc += 2;
                        }
                    }
                    0x0015 => {
                        // Set delay timer = VX
                        self.delay_timer = self.v[x];
                        self.pc += 2;
                    }
                    0x0018 => {
                        // Set sound timer = VX
                        self.sound_timer = self.v[x];
                        self.pc += 2;
                    }
                    0x001E => {
                        // Add VX to I
                        self.i += self.v[x] as u16;
                        self.pc += 2;
                    }
                    0x0029 => {
                        // Set I = location of sprite for digit VX
                        self.i = 0x50 + (self.v[x] as u16) * 5;
                        self.pc += 2;
                    }
                    0x0033 => {
                        // Store BCD representation of VX
                        self.memory[self.i as usize] = self.v[x] / 100;
                        self.memory[(self.i + 1) as usize] = (self.v[x] / 10) % 10;
                        self.memory[(self.i + 2) as usize] = self.v[x] % 10;
                        self.pc += 2;
                    }
                    0x0055 => {
                        // Store registers V0 through VX in memory starting at I
                        for i in 0..=x {
                            self.memory[(self.i + i as u16) as usize] = self.v[i];
                        }
                        self.pc += 2;
                    }
                    0x0065 => {
                        // Read registers V0 through VX from memory starting at I
                        for i in 0..=x {
                            self.v[i] = self.memory[(self.i + i as u16) as usize];
                        }
                        self.pc += 2;
                    }
                    _ => {
                        println!("Unknown opcode: 0x{:X}", opcode);
                        self.pc += 2;
                    }
                }
            }
            _ => {
                println!("Unknown opcode: 0x{:X}", opcode);
                self.pc += 2;
            }
        }
    }

    pub fn set_keys(&mut self, window: &Window) {
        // CHIP-8 keypad mapping to keyboard
        self.keypad[0x1] = window.is_key_down(Key::Key1);
        self.keypad[0x2] = window.is_key_down(Key::Key2);
        self.keypad[0x3] = window.is_key_down(Key::Key3);
        self.keypad[0xC] = window.is_key_down(Key::Key4);
        
        self.keypad[0x4] = window.is_key_down(Key::Q);
        self.keypad[0x5] = window.is_key_down(Key::W);
        self.keypad[0x6] = window.is_key_down(Key::E);
        self.keypad[0xD] = window.is_key_down(Key::R);
        
        self.keypad[0x7] = window.is_key_down(Key::A);
        self.keypad[0x8] = window.is_key_down(Key::S);
        self.keypad[0x9] = window.is_key_down(Key::D);
        self.keypad[0xE] = window.is_key_down(Key::F);
        
        self.keypad[0xA] = window.is_key_down(Key::Z);
        self.keypad[0x0] = window.is_key_down(Key::X);
        self.keypad[0xB] = window.is_key_down(Key::C);
        self.keypad[0xF] = window.is_key_down(Key::V);
    }
}
