// scheduler.rs

pub struct ProcessControlBlock {
    pub id: usize,
    pub cpu_state: CpuState,
    // Outros campos relevantes para o estado do processo
}

pub struct CpuState {
    pub pc: usize,  // Program Counter
    pub registers: [u32; 16],  // Registradores do CPU
    // Outros campos relevantes para o estado do CPU
}

pub struct Scheduler {
    pub ready_queue: Vec<ProcessControlBlock>,
    pub current_process: Option<ProcessControlBlock>,
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            ready_queue: Vec::new(),
            current_process: None,
        }
    }

    pub fn add_process(&mut self, pcb: ProcessControlBlock) {
        self.ready_queue.push(pcb);
    }

    pub fn schedule(&mut self) {
        if let Some(current) = self.current_process.take() {
            self.ready_queue.push(current);
        }
        self.current_process = self.ready_queue.pop();
    }
}
