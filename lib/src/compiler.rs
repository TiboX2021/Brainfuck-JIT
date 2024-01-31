//! JIT compiler implementation

use crate::instructions::Instruction;
use memmap::MmapMut;

pub struct Compiler {
    /// Buffer for the generated machine code
    machine_code: Vec<u8>,

    /// Memory buffer of 30_000 bytes
    memory: Vec<u8>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            machine_code: Vec::new(),
            memory: vec![0; 30_000],
        }
    }

    /// Compile the brainfuck source code into some machine code.
    /// The machine code will be stored inside this struct for later execution.
    pub fn compile(&mut self, source: &[Instruction]) {
        let memory_adress = self.memory.as_ptr();

        // Add the mov r13, memory_address instruction into the machine code
        self.machine_code.extend_from_slice(&[0x49, 0xbd]);
        self.machine_code
            .extend_from_slice(&(memory_adress as u64).to_le_bytes());

        // TODO: compile the whole thing

        // Last: append the RET instruction
        self.machine_code.push(0xc3);
    }

    /// Execute the compiled machine code
    pub fn execute(&self) {
        assert!(!self.machine_code.is_empty(), "No machine code to execute");

        // Create an anonymous memory map the size of our machine code
        let mut memory_map = MmapMut::map_anon(self.machine_code.len()).unwrap();
        memory_map.clone_from_slice(&self.machine_code); // Clone the machine code into it

        // Make the memory map executable
        let memory_map = memory_map.make_exec().unwrap();
        // Get a pointer to the machine code
        let func_ptr = memory_map.as_ptr();
        unsafe {
            let main: fn() = std::mem::transmute(func_ptr);
            main();
        }
    }

    /// Clear the compiler from its previous run
    pub fn clear(&mut self) {
        self.machine_code.clear();
        self.memory = vec![0; 30_000];
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Compiler::new()
    }
}
