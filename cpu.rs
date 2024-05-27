use crate::gpu::Gpu;
use crate::memory::Memory;
use crate::scheduler::{Scheduler, ProcessControlBlock};

// Definição do tipo Instruction (assumindo que será usado no módulo)
#[derive(Debug, Clone)]
pub enum Instruction {
    Add,
    Sub,
    Mul,
    TestCommand,
    OpenWindow,
    Unknown(u32),
}

pub struct Cpu<'a> {
    pub gpu: &'a mut Gpu,
    pub scheduler: Scheduler,
    pub pipeline: Pipeline<u32>,
}

impl<'a> Cpu<'a> {
    pub fn new(gpu: &'a mut Gpu) -> Self {
        Cpu {
            gpu,
            scheduler: Scheduler::new(),
            pipeline: Pipeline::new(),
        }
    }

    pub fn execute_cycle(&mut self, memory: &mut Memory) {
        // Primeiramente, obtenha o `current_process` como mutável
        let mut current_process_option = self.scheduler.current_process.take();

        // Execute a instrução se há um processo atual
        if let Some(ref mut current_process) = current_process_option {
            self.execute_instruction(current_process, memory);
        }

        // Devolva o `current_process` ao `scheduler`
        self.scheduler.current_process = current_process_option;

        // Troca para o próximo processo
        self.scheduler.schedule();
    }

    fn execute_instruction(&mut self, process: &mut ProcessControlBlock, memory: &mut Memory) {
        let instruction_address = process.cpu_state.pc;
        let opcode = memory.read_word(instruction_address);
        let instruction = Instruction::from(opcode);

        match instruction {
            Instruction::Add => {
                // Lógica para a instrução Add
                println!("Executando instrução Add");
            }
            Instruction::Sub => {
                // Lógica para a instrução Sub
                println!("Executando instrução Sub");
            }
            Instruction::Mul => {
                // Lógica para a instrução Mul
                println!("Executando instrução Mul");
            }
            Instruction::TestCommand => {
                // Executar comando de teste para a GPU
                self.execute_test_command();
            }
            Instruction::OpenWindow => {
                // Abrir uma janela utilizando a GPU
                self.gpu.open_window();
            }
            Instruction::Unknown(op) => {
                println!("Instrução desconhecida: 0x{:X}", op);
            }
        }

        // Atualiza o program counter do processo atual
        process.cpu_state.pc += 1; // Supondo que cada instrução ocupa 1 palavra de memória

        // Atualiza o pipeline
        self.pipeline.fetch_instruction(memory, instruction_address);
        self.pipeline.decode_instruction();
        self.pipeline.update(opcode);
    }

    pub fn execute_test_command(&mut self) {
        println!("Enviando comando de teste para a GPU...");
        // Chame o método `send_test_command` da GPU
        self.gpu.send_test_command();
    }
}

pub struct Pipeline<T> {
    fetch: Option<T>,
    decode: Option<T>,
    execute: Option<T>,
    writeback: Option<T>,
}

impl Pipeline<u32> {
    pub fn new() -> Self {
        Pipeline {
            fetch: None,
            decode: None,
            execute: None,
            writeback: None,
        }
    }

    pub fn fetch_instruction(&mut self, memory: &Memory, address: usize) {
        let opcode = memory.read_word(address);
        self.update(opcode); // Atualizamos o `fetch` com o `opcode` (u32)
    }

    pub fn decode_instruction(&mut self) {
        // Implemente a lógica de decodificação das instruções aqui
    }

    pub fn update(&mut self, instruction: u32) {
        self.writeback = self.execute.take();
        self.execute = self.decode.take();
        self.decode = self.fetch.take();
        self.fetch = Some(instruction);
    }
}

impl Instruction {
    pub fn from(opcode: u32) -> Self {
        match opcode {
            0x01 => Instruction::Add,
            0x02 => Instruction::Sub,
            0x03 => Instruction::Mul,
            0x04 => Instruction::TestCommand,
            0x05 => Instruction::OpenWindow,
            _ => Instruction::Unknown(opcode),
        }
    }
}
