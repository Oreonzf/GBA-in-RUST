use crate::gpu::Gpu;
use minifb::{Key, Window, WindowOptions};

pub fn create_menu_window(title: &str, width: usize, height: usize) -> Window {
    let mut options = WindowOptions::default();
    options.resize = false; // Disable resizing
    options.scale = minifb::Scale::X1; // Disable scaling

    Window::new(
        title,
        width,
        height,
        options,
    ).expect("Error creating menu window")
}

pub fn show_menu(gpu: &mut Gpu) {
    let mut window = create_menu_window("Gameboy Advanced Emulator Menu", 800, 600);

    let menu_items = vec!["Start Game", "Load Game", "Settings", "Exit"];
    let mut selected_item = 0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_down(Key::Up) {
            if selected_item > 0 {
                selected_item -= 1;
            }
        }
        if window.is_key_down(Key::Down) {
            if selected_item < menu_items.len() - 1 {
                selected_item += 1;
            }
        }
        if window.is_key_down(Key::Enter) {
            match selected_item {
                0 => println!("Start Game selected"),
                1 => println!("Load Game selected"),
                2 => println!("Settings selected"),
                3 => break,
                _ => (),
            }
        }

        // Render the menu
        println!("Rendering menu. Selected item: {}", selected_item);
        for (index, item) in menu_items.iter().enumerate() {
            println!("Menu item {}: {}", index, item);
        }
        
        gpu.clear_screen(0x000000); // Black background
        gpu.draw_menu(&menu_items, selected_item);

        if let Err(e) = window.update_with_buffer(&gpu.frame_buffer, gpu.screen_width as usize, gpu.screen_height as usize) {
            println!("Error updating window: {}", e);
            break;
        }
    }
}
