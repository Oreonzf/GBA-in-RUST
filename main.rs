mod cpu;
mod font;
mod gpu;
mod memory;
mod menu;

use gpu::Gpu;
use memory::Memory;
use cpu::Cpu;

fn main() {
    let mut gpu = Gpu::new(800, 600);
    menu::show_menu(&mut gpu);

    let rom_path = "path/to/your/rom/file.rom"; // Set the correct path
    let mut memory = Memory::new(vec![], 8192, 8192);
    memory.load_rom(rom_path);

    let mut cpu = Cpu::new(&mut gpu);
    cpu.start(&mut memory);
}
