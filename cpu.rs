use crate::gpu::Gpu;
use crate::memory::Memory;

// Definition of the Instruction type (assuming it will be used in the module)
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
    pub pipeline: Pipeline<u32>,
}

impl<'a> Cpu<'a> {
    pub fn new(gpu: &'a mut Gpu) -> Self {
        Cpu {
            gpu,
            pipeline: Pipeline::new(),
        }
    }

    pub fn execute_cycle(&mut self, memory: &mut Memory) {
        // Fetch instruction and execute it
        let instruction_address = self.gpu.get_instruction_address(); // Assuming this method exists
        let opcode = memory.read_word(instruction_address);
        let instruction = Instruction::from(opcode);

        self.execute_instruction(instruction, memory, instruction_address, opcode);

        // Update PC (Program Counter)
        self.gpu.increment_pc(); // Assuming this method exists
    }

    fn execute_instruction(
        &mut self,
        instruction: Instruction,
        memory: &mut Memory,
        instruction_address: usize,
        opcode: u32,
    ) {
        match instruction {
            Instruction::Add => {
                // Logic for Add instruction
                println!("Executing Add instruction");
            }
            Instruction::Sub => {
                // Logic for Sub instruction
                println!("Executing Sub instruction");
            }
            Instruction::Mul => {
                // Logic for Mul instruction
                println!("Executing Mul instruction");
            }
            Instruction::TestCommand => {
                // Execute test command for the GPU
                self.execute_test_command();
            }
            Instruction::OpenWindow => {
                // Open a window using the GPU
                self.gpu.open_window(); // Assuming this method exists
            }
            Instruction::Unknown(op) => {
                println!("Unknown instruction: 0x{:X}", op);
            }
        }

        // Fetch, decode, and update pipeline
        self.pipeline.fetch_instruction(memory, instruction_address);
        self.pipeline.decode_instruction();
        self.pipeline.update(opcode);
    }

    pub fn execute_test_command(&mut self) {
        println!("Sending test command to the GPU...");
        // Call the `send_test_command` method of the GPU
        self.gpu.send_test_command(); // Assuming this method exists
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
        self.update(opcode); // Update `fetch` with the `opcode` (u32)
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
