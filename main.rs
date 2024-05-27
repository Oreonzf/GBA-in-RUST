// main.rs

mod cpu;
mod gpu;
mod memory;
mod scheduler;

use cpu::Cpu;
use gpu::Gpu;
use memory::Memory;
use minifb::Key;
use scheduler::{ProcessControlBlock, CpuState};

fn main() {
    let screen_width = 800;
    let screen_height = 600;

    // Inicializa a GPU e a janela
    let mut gpu = Gpu::new(screen_width, screen_height);
    let mut window = gpu.open_window();

    // Dados de memória
    let memory_data = vec![0; 1024];
    let ram_size = 1024;  // Exemplo: 1 KB de RAM
    let vram_size = 1024; // Exemplo: 1 KB de VRAM

    let mut memory = Memory::new(memory_data, ram_size, vram_size);

    // Inicializa a CPU com a instância da GPU
    let mut cpu = Cpu::new(&mut gpu);

    // Inicializa alguns processos (apenas para exemplo)
    cpu.scheduler.add_process(ProcessControlBlock {
        id: 1,
        cpu_state: CpuState { pc: 0, registers: [0; 16] },
    });
    cpu.scheduler.add_process(ProcessControlBlock {
        id: 2,
        cpu_state: CpuState { pc: 0, registers: [0; 16] },
    });

    // Loop principal
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Atualiza a posição da imagem com base nas teclas pressionadas
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

        // Executa um ciclo da CPU
        cpu.execute_cycle(&mut memory);

        // Limpa o buffer da GPU
        cpu.gpu.clear_screen();

        // Desenha um retângulo rosa na tela na posição da imagem
        let image_x = cpu.gpu.get_image_x();
        let image_y = cpu.gpu.get_image_y();
        cpu.gpu.draw_rectangle(image_x, image_y, 20, 20, 0xFFCCFF);

        // Renderiza o quadro
        cpu.gpu.render_frame();

        // Atualiza a janela com o conteúdo da GPU
        cpu.gpu.update_window(&mut window);
    }
}
