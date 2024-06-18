use crate::memory::Memory;
use minifb::{Key, Window, WindowOptions};
use std::time::{Duration, Instant};

pub struct Gpu {
    pub screen_width: usize,
    pub screen_height: usize,
    pub frame_buffer: Vec<u32>,
    pub pc: usize,
    pub memory: Memory, // Adicionar referência à memória
}

impl Gpu {
    pub fn new(screen_width: usize, screen_height: usize) -> Self {
        Gpu {
            screen_width,
            screen_height,
            frame_buffer: vec![0; screen_width * screen_height],
            pc: 0,
            memory: Memory::new(vec![], 8192, 8192), // Inicializar memória
        }
    }

    pub fn get_instruction_address(&self) -> usize {
        self.pc
    }

    pub fn increment_pc(&mut self) {
        self.pc += 1;
    }

    pub fn open_window(&self) {
        println!("Window opened");
    }

    pub fn send_test_command(&self) {
        println!("Test command sent");
    }

    pub fn load_rom(&mut self, path: &str) {
        self.memory.load_rom(path);
    }

    pub fn run_menu(&mut self) {
        let mut window = Window::new(
            "Gameboy Advanced Emulator Menu",
            self.screen_width,
            self.screen_height,
            WindowOptions::default(),
        ).expect("Error creating window");

        let menu_items = vec!["Start Game", "Load Game", "Settings", "Exit"];
        let mut selected_item = 0;

        let mut last_update = Instant::now();

        while window.is_open() && !window.is_key_down(Key::Escape) {
            self.clear_screen(0x000000); // Clear screen
            self.draw_header("Main Menu", 50, 20, 0x00FF00);
            self.draw_menu(&menu_items, selected_item);
            self.draw_instructions(50, self.screen_height as u32 - 50, 0xFFFFFF);

            window.update_with_buffer(&self.frame_buffer, self.screen_width, self.screen_height)
                .expect("Failed to update window");

            if last_update.elapsed() >= Duration::from_millis(100) {
                let keys = window.get_keys();
                for key in keys {
                    match key {
                        Key::Up => {
                            if selected_item > 0 {
                                selected_item -= 1;
                            }
                        }
                        Key::Down => {
                            if selected_item < menu_items.len() - 1 {
                                selected_item += 1;
                            }
                        }
                        Key::Enter => match selected_item {
                            0 => self.start_game(),
                            1 => self.load_game(),
                            2 => self.settings(&mut window),
                            3 => {
                                println!("Exiting...");
                                return;
                            }
                            _ => (),
                        },
                        _ => (),
                    }
                }
                last_update = Instant::now();
            }
        }
    }

    pub fn start_game(&self) {
        println!("Start Game selected");
        // Implement start game logic
    }

    pub fn load_game(&self) {
        println!("Load Game selected");
        // Implement load game logic
    }

    pub fn settings(&mut self, window: &mut Window) {
        let submenu_items = vec!["Option 1", "Option 2", "Back"];
        let mut selected_item = 0;

        let mut last_update = Instant::now();

        while window.is_open() && !window.is_key_down(Key::Escape) {
            self.clear_screen(0x000000); // Clear screen
            self.draw_header("Settings", 50, 20, 0x00FF00);
            self.draw_menu(&submenu_items, selected_item);
            self.draw_instructions(50, self.screen_height as u32 - 50, 0xFFFFFF);

            window.update_with_buffer(&self.frame_buffer, self.screen_width, self.screen_height)
                .expect("Failed to update window");

            if last_update.elapsed() >= Duration::from_millis(100) {
                let keys = window.get_keys();
                for key in keys {
                    match key {
                        Key::Up => {
                            if selected_item > 0 {
                                selected_item -= 1;
                            }
                        }
                        Key::Down => {
                            if selected_item < submenu_items.len() - 1 {
                                selected_item += 1;
                            }
                        }
                        Key::Enter => match selected_item {
                            0 => println!("Option 1 selected"),
                            1 => println!("Option 2 selected"),
                            2 => return,
                            _ => (),
                        },
                        _ => (),
                    }
                }
                last_update = Instant::now();
            }
        }
    }

    pub fn draw_header(&mut self, text: &str, x: u32, y: u32, color: u32) {
        self.draw_text(text, x, y, color);
    }

    pub fn draw_instructions(&mut self, x: u32, y: u32, color: u32) {
        self.draw_text("Use Up/Down to navigate, Enter to select", x, y, color);
    }

    pub fn draw_text(&mut self, text: &str, start_x: u32, start_y: u32, color: u32) {
        for (i, c) in text.chars().enumerate() {
            if let Some(bitmap) = crate::font::get_char_bitmap(c) {
                self.draw_bitmap(bitmap, start_x + (i * 8) as u32, start_y, color);
            }
        }
    }

    fn draw_bitmap(&mut self, bitmap: [u8; 8], x: u32, y: u32, color: u32) {
        for (row, &byte) in bitmap.iter().enumerate() {
            for col in 0..8 {
                if (byte >> col) & 1 == 1 {
                    let pixel_x = x + row as u32;
                    let pixel_y = y + col as u32;
                    let offset = pixel_y as usize * self.screen_width + pixel_x as usize;
                    if offset < self.frame_buffer.len() {
                        self.frame_buffer[offset] = color;
                    }
                }
            }
        }
    }

    pub fn clear_screen(&mut self, color: u32) {
        for pixel in self.frame_buffer.iter_mut() {
            *pixel = color;
        }
    }

    pub fn draw_menu(&mut self, menu_items: &[&str], selected_item: usize) {
        let white = 0xFFFFFF;
        let yellow = 0xFFFF00;

        for (index, &item) in menu_items.iter().enumerate() {
            let color = if index == selected_item { yellow } else { white };
            self.draw_text(item, 100, 100 + (index * 20) as u32, color);
        }
    }
}
