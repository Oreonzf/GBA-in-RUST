use minifb::{Key, Window, WindowOptions};

pub struct Gpu {
    pub screen_width: usize,
    pub screen_height: usize,
    pub frame_buffer: Vec<u32>,
    pub pc: usize,
}

impl Gpu {
    pub fn new(screen_width: usize, screen_height: usize) -> Self {
        Gpu {
            screen_width,
            screen_height,
            frame_buffer: vec![0; screen_width * screen_height],
            pc: 0,
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

    pub fn run_menu(&mut self) {
        let mut window = Window::new(
            "Gameboy Advanced Emulator Menu",
            self.screen_width,
            self.screen_height,
            WindowOptions::default(),
        ).expect("Error creating window");

        let menu_items = vec!["TEST1", "TEST2", "TEST3"];
        let mut selected_item = 0;

        while window.is_open() && !window.is_key_down(Key::Escape) {
            self.clear_screen(0x000000); // Clear screen
            self.draw_menu(&menu_items, selected_item); // Draw menu

            for t in window.get_keys() {
                match t {
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
                    Key::Enter => {
                        match selected_item {
                            0 => println!("Starting TEST1..."),
                            1 => println!("Starting TEST2..."),
                            2 => {
                                println!("Exiting...");
                                return; // Exit the function
                            }
                            _ => (),
                        }
                    }
                    _ => (),
                }
            }

            window.update_with_buffer(&self.frame_buffer, self.screen_width, self.screen_height)
                .expect("Failed to update window");
        }
    }

    fn draw_text(&mut self, text: &str, start_x: u32, start_y: u32, color: u32) {
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
