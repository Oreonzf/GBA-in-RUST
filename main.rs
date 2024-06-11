mod memory;
mod gpu;
mod cpu;
mod menu;
mod font;
use gpu::Gpu;

fn main() {
    let mut gpu = Gpu::new(800, 600);
    gpu.run_menu();
}