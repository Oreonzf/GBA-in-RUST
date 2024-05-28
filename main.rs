mod cpu;
mod gpu;
mod memory;

use cpu::Cpu;
use gpu::Gpu;
use memory::Memory;
use minifb::Key;

fn main() {
    let screen_width = 800;
    let screen_height = 600;

    // Initialize GPU and window
    let mut gpu = Gpu::new(screen_width, screen_height);
    let mut window = gpu.open_window();

    // Memory data
    let memory_data = vec![0; 1024];
    let ram_size = 1024;  // Example: 1 KB RAM
    let vram_size = 1024; // Example: 1 KB VRAM

    let mut memory = Memory::new(memory_data, ram_size, vram_size);

    // Initialize CPU with GPU instance
    let mut cpu = Cpu::new(&mut gpu);

    // Main loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Update image position based on pressed keys
        if window.is_key_down(Key::W) {
            cpu.gpu.move_image(Key::W);
        }
        if window.is_key_down(Key::A) {
            cpu.gpu.move_image(Key::A);
        }
        if window.is_key_down(Key::S) {
            cpu.gpu.move_image(Key::S);
        }
        if window.is_key_down(Key::D) {
            cpu.gpu.move_image(Key::D);
        }

        // Execute a CPU cycle
        cpu.execute_cycle(&mut memory);

        // Clear GPU buffer
        cpu.gpu.clear_screen();

        // Draw a pink rectangle on the screen at the image position
        let image_x = cpu.gpu.get_image_x();
        let image_y = cpu.gpu.get_image_y();
        cpu.gpu.draw_rectangle(image_x, image_y, 20, 20, 0xFFCCFF);

        // Render frame
        cpu.gpu.render_frame();

        // Update window with GPU content
        cpu.gpu.update_window(&mut window);
    }
}
