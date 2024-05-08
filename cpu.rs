use crate::gpu::Gpu as CpuGpu;
use crate::memory::Memory;
use crate::gpu::main;

mod gpu;

pub struct Cpu {}

impl Cpu {
    pub fn new() -> Self {
        Cpu {}
    }

    pub fn execute_instruction(&mut self, memory: &mut Memory, gpu: &mut CpuGpu) {
        let instruction_address = self.fetch_instruction_pointer();
        let opcode = memory.read_byte(instruction_address) as u32;
        self.decode_and_execute(opcode, memory, gpu);
    }

    fn fetch_instruction_pointer(&self) -> usize {
        // Implemente a lógica real para buscar o endereço da próxima instrução
        0 // Neste exemplo, retorna sempre zero
    }

    fn decode_and_execute(&mut self, opcode: u32, memory: &mut Memory, gpu: &mut CpuGpu) {
        match opcode {
            0x01 => self.execute_add(memory),
            0x02 => self.execute_test_command(gpu),
            _ => {
                println!("Instrução não implementada: 0x{:X}", opcode);
            }
        }
    }

    fn execute_add(&mut self, memory: &mut Memory) {
        // Lógica para a instrução Add
        // Implemente sua lógica de adição aqui
        println!("Executando instrução Add");
    }

    fn execute_test_command(&self, gpu: &mut CpuGpu) {
        println!("Enviando comando de teste para a GPU...");
        gpu.send_test_command(); // Assumindo que `send_test_command` está definido na struct `Gpu`
    }
}

#[derive(Default, Clone)] // Implementação do trait Default e Clone
pub struct Registers {
    r: [u32; 16],
    pc: u32,
}

#[derive(Clone)] // Implementação do trait Clone
pub enum Instruction {
    Add,
    TestCommand,
    Unknown(u32),
}

pub struct CpuState {
    pub registers: Registers,
    pipeline: Pipeline,
    memory: Memory,
    pub gpu: Gpu,
}

impl CpuState {
    pub fn new(memory_data: Vec<u32>, gpu: Gpu) -> Self {
        CpuState {
            registers: Registers::default(),
            pipeline: Pipeline::new(),
            memory: Memory::new(memory_data),
            gpu,
        }
    }

    pub fn execute_cycle(&mut self) {
        self.pipeline.advance();
        let pc = self.registers.read_pc() as usize;
        self.pipeline.fetch_instruction(&self.memory, pc);
        self.pipeline.decode_instruction();
        self.execute_instruction();
    }

    fn execute_instruction(&mut self) {
        if let Some(instruction) = self.pipeline.get_current_instruction() {
            match instruction {
                Instruction::Add => {
                    // Lógica para a instrução Add
                }
                Instruction::TestCommand => {
                    println!("Enviando comando de teste para a GPU...");
                    // Chamada do método corrigida para enviar um comando de teste para a GPU
                    self.gpu.send_test_command();
                }
                Instruction::Unknown(_) => {
                    unreachable!("Instrução desconhecida encontrada!");
                }
            }
        } else {
            println!("Nenhuma instrução para executar no momento.");
        }
    }
}

struct Memory {
    data: Vec<u32>,
}

impl Memory {
    fn new(memory_data: Vec<u32>) -> Self {
        Memory { data: memory_data }
    }

    fn read_word(&self, address: usize) -> u32 {
        self.data[address]
    }
}

struct Pipeline {
    fetch: Option<Instruction>,
    decode: Option<Instruction>,
    execute: Option<Instruction>,
    writeback: Option<Instruction>,
}

impl Pipeline {
    fn new() -> Self {
        Pipeline {
            fetch: None,
            decode: None,
            execute: None,
            writeback: None,
        }
    }

    fn advance(&mut self) {
        self.writeback = self.execute.take();
        self.execute = self.decode.take();
        self.decode = self.fetch.take();
    }

    fn get_current_instruction(&self) -> Option<Instruction> {
        self.execute.clone()
    }

    fn fetch_instruction(&mut self, memory: &Memory, address: usize) {
        let opcode = memory.read_word(address);
        let instruction = Instruction::from(opcode);
        self.update(instruction);
    }

    fn decode_instruction(&mut self) {
        // Implemente a lógica de decodificação das instruções aqui
    }

    fn update(&mut self, instruction: Instruction) {
        self.writeback = self.execute.take();
        self.execute = self.decode.take();
        self.decode = self.fetch.take();
        self.fetch = Some(instruction);
    }
}

impl From<u32> for Instruction {
    fn from(opcode: u32) -> Self {
        match opcode {
            // Defina instruções suportadas
            _ => Instruction::Add, // Retorna Add para instruções não suportadas
        }
    }
}
