use minifb::{Key, Window, WindowOptions};

pub struct Gpu {
    screen_width: u32,
    screen_height: u32,
    frame_buffer: Vec<u32>,
    image_x: usize,
    image_y: usize,
}

impl Gpu {
    pub fn new(screen_width: u32, screen_height: u32) -> Self {
        let frame_buffer_size = (screen_width * screen_height) as usize;
        let frame_buffer = vec![0; frame_buffer_size];

        Gpu {
            screen_width,
            screen_height,
            frame_buffer,
            image_x: 300,   // Posição inicial da imagem (x)
            image_y: 200,   // Posição inicial da imagem (y)
        }
    }

    pub fn open_window(&self) -> Window {
        Window::new(
            "Pixelated Heart",
            self.screen_width as usize,
            self.screen_height as usize,
            WindowOptions::default(),
        )
        .expect("Error creating window")
    }

    pub fn render_frame(&self) {
        println!("Rendering a frame on the screen...");
    }

    pub fn clear_screen(&mut self) {
        self.frame_buffer.iter_mut().for_each(|pixel| *pixel = 0);
    }

    pub fn draw_rectangle(&mut self, x: usize, y: usize, width: usize, height: usize, color: u32) {
        // Calculate the coordinates of the rectangle within the screen boundaries
        let start_x = x.min(self.screen_width as usize);
        let start_y = y.min(self.screen_height as usize);
        let end_x = (x + width).min(self.screen_width as usize);
        let end_y = (y + height).min(self.screen_height as usize);

        // Draw the rectangle in the valid area of the screen
        for py in start_y..end_y {
            for px in start_x..end_x {
                let index = py * (self.screen_width as usize) + px;
                self.frame_buffer[index] = color;
            }
        }
    }

    pub fn update_window(&self, window: &mut Window) {
        if let Some(frame_buffer) = self.get_frame_buffer() {
            window
                .update_with_buffer(frame_buffer, self.screen_width as usize, self.screen_height as usize)
                .expect("Error updating window");
        }
    }

    fn get_frame_buffer(&self) -> Option<&[u32]> {
        Some(&self.frame_buffer)
    }

    pub fn move_image(&mut self, key: Key) {
        match key {
            Key::W => self.image_y = self.image_y.saturating_sub(5), // Move up
            Key::A => self.image_x = self.image_x.saturating_sub(5), // Move left
            Key::S => self.image_y = self.image_y.saturating_add(5), // Move down
            Key::D => self.image_x = self.image_x.saturating_add(5), // Move right
            _ => {} // Ignore other keys
        }
    }

    pub fn send_test_command(&mut self) {
        println!("Sending test command to the GPU...");
        // Implement the necessary logic for the test command here
    }

    pub fn get_image_x(&self) -> usize {
        self.image_x
    }

    pub fn get_image_y(&self) -> usize {
        self.image_y
    }

    // Method to get the instruction address from GPU
    pub fn get_instruction_address(&self) -> usize {
        // Replace this with the actual logic to get the instruction address from the GPU
        // For demonstration purposes, returning a dummy value
        0
    }

    // Method to increment the program counter in GPU
    pub fn increment_pc(&mut self) {
        // Replace this with the actual logic to increment the program counter in the GPU
        // For demonstration purposes, no-op
    }
}
