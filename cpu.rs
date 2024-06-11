use crate::gpu::Gpu as GpuModule;
use crate::memory::Memory;

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
    pub gpu: &'a mut GpuModule,
    pub pipeline: Pipeline<u32>,
}

impl<'a> Cpu<'a> {
    pub fn new(gpu: &'a mut GpuModule) -> Self {
        Cpu {
            gpu,
            pipeline: Pipeline::new(),
        }
    }

    pub fn execute_cycle(&mut self, memory: &mut Memory) {
        let instruction_address = self.gpu.get_instruction_address();
        let opcode = memory.read_word(instruction_address);
        let instruction = Instruction::from(opcode);

        self.execute_instruction(instruction, memory);

        self.gpu.increment_pc();
    }

    pub fn execute_instruction(&mut self, instruction: Instruction, memory: &mut Memory) {
        match instruction {
            Instruction::Add => {
                println!("Executing Add instruction");
            }
            Instruction::Sub => {
                println!("Executing Sub instruction");
            }
            Instruction::Mul => {
                println!("Executing Mul instruction");
            }
            Instruction::TestCommand => {
                self.execute_test_command();
            }
            Instruction::OpenWindow => {
                self.gpu.open_window();
            }
            Instruction::Unknown(op) => {
                println!("Unknown instruction: 0x{:X}", op);
            }
        }

        self.pipeline.fetch_instruction(memory, self.gpu.get_instruction_address());
        self.pipeline.decode_instruction();
        self.pipeline.update(memory.read_word(self.gpu.get_instruction_address()));
    }

    pub fn execute_test_command(&mut self) {
        println!("Sending test command to the GPU...");
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
        self.update(opcode);
    }

    pub fn decode_instruction(&mut self) {
        // Implement instruction decoding logic here
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
