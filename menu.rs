use crate::gpu::Gpu;
use minifb::{Key, Window, WindowOptions};
use std::time::{Duration, Instant};
use rfd::FileDialog;

pub fn show_menu(gpu: &mut Gpu) {
    let mut window = Window::new(
        "Gameboy Advanced Emulator Menu",
        gpu.screen_width,
        gpu.screen_height,
        WindowOptions::default(),
    ).expect("Error creating window");

    let menu_items = vec!["Start Game", "Load Game", "Settings", "Exit"];
    let mut selected_item = 0;

    let mut last_update = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        gpu.clear_screen(0x000000); // Clear screen
        gpu.draw_header("Main Menu", 50, 20, 0x00FF00);
        gpu.draw_menu(&menu_items, selected_item);
        gpu.draw_instructions(50, gpu.screen_height as u32 - 50, 0xFFFFFF);

        window.update_with_buffer(&gpu.frame_buffer, gpu.screen_width, gpu.screen_height)
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
                        0 => gpu.start_game(),
                        1 => load_game(gpu),
                        2 => settings(gpu, &mut window),
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

fn load_game(gpu: &mut Gpu) {
    if let Some(file) = FileDialog::new().add_filter("GBA ROM", &["gba"]).pick_file() {
        println!("Loading game from {:?}", file);
        gpu.load_rom(file.to_str().unwrap());
    } else {
        println!("No file selected.");
    }
}

fn settings(gpu: &mut Gpu, window: &mut Window) {
    let submenu_items = vec!["Option 1", "Option 2", "Back"];
    let mut selected_item = 0;

    let mut last_update = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        gpu.clear_screen(0x000000); // Clear screen
        gpu.draw_header("Settings", 50, 20, 0x00FF00);
        gpu.draw_menu(&submenu_items, selected_item);
        gpu.draw_instructions(50, gpu.screen_height as u32 - 50, 0xFFFFFF);

        window.update_with_buffer(&gpu.frame_buffer, gpu.screen_width, gpu.screen_height)
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
