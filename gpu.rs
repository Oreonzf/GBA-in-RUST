use minifb::{Key, Window, WindowOptions};

pub struct Gpu {
    pub screen: Screen,
}

pub struct Screen {
    width: usize,
    height: usize,
    pixels: Vec<u32>,
}

impl Screen {
    pub fn new(width: usize, height: usize) -> Self {
        let pixels = vec![0; width * height];
        Screen { width, height, pixels }
    }

    pub fn draw_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            self.pixels[index] = color;
        }
    }

    pub fn clear(&mut self) {
        self.pixels.iter_mut().for_each(|pixel| *pixel = 0);
    }

    pub fn get_pixels(&self) -> &[u32] {
        &self.pixels
    }
}

impl Gpu {
    pub fn new(screen_width: usize, screen_height: usize) -> Self {
        let screen = Screen::new(screen_width, screen_height);
        Gpu { screen }
    }

    pub fn draw_pixel(&mut self, x: u32, y: u32, color: u32) {
        self.screen.draw_pixel(x as usize, y as usize, color);
    }

    pub fn clear_screen(&mut self) {
        self.screen.clear();
    }

    pub fn update_window(&mut self, window: &mut Window) {
        let pixels = self.screen.get_pixels();
        let width = self.screen.width;
        let height = self.screen.height;
        window.update_with_buffer(pixels, width, height).unwrap();
    }

    pub fn draw_heart(&mut self, x: u32, y: u32, size: u32, color: u32) {
        let half_size = size >> 1;
    
        // Desenhar a parte superior do coração (dois triângulos)
        for dy in 0..half_size {
            let dx = (half_size.saturating_sub(dy)) << 1;  // Use saturating_sub para evitar estouro
    
            // Desenhar os pixels da parte esquerda do coração
            for i in 0..=dx {
                if let Some(px) = (x as i32 + half_size as i32 - dx as i32 + i as i32).checked_sub(1) {
                    let px = px as usize;
                    if px < self.screen.width {
                        self.draw_pixel(px as u32, y + dy, color);
                    }
                }
            }
    
            // Desenhar os pixels da parte direita do coração
            for i in 0..=dx {
                if let Some(px) = (x as i32 + half_size as i32 + i as i32).checked_sub(1) {
                    let px = px as usize;
                    if px < self.screen.width {
                        self.draw_pixel(px as u32, y + dy, color);
                    }
                }
            }
        }
    
        // Desenhar a parte inferior do coração (triângulo invertido)
        for dy in 0..half_size {
            let dx = dy << 1;  // Calcular dx para o triângulo invertido
    
            // Desenhar os pixels da parte inferior do coração
            for i in 0..(size.saturating_sub(dx * 2)) {
                if let Some(px) = (x as i32 + dx as i32 + i as i32).checked_sub(1) {
                    let px = px as usize;
                    if px < self.screen.width {
                        self.draw_pixel(px as u32, y + half_size + dy, color);
                    }
                }
            }
        }
    }
    
}

fn main() {
    let width = 800;
    let height = 600;
    let mut window = Window::new("Pixelated Heart", width, height, WindowOptions::default())
        .expect("Unable to create window");

    let mut gpu = Gpu::new(width, height);

    // Loop principal
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Limpa a janela
        gpu.clear_screen();

        // Desenha um coração vermelho na tela
        gpu.draw_heart(300, 200, 100, 0xFF0000);

        // Atualiza a janela
        gpu.update_window(&mut window);
    }
}
