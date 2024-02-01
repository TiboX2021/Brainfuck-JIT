//! JIT compiler implementation

use crate::instructions::Instruction;
use memmap2::{Mmap, MmapMut};

pub struct Compiler {
    /// Buffer for the generated machine code
    machine_code: Vec<u8>,

    /// Actual executable memory. The jump addresses will have to be resolved here.
    executable_memory: Mmap,

    /// Memory buffer of 30_000 bytes
    memory: Vec<u8>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            machine_code: Vec::new(),
            executable_memory: MmapMut::map_anon(1).unwrap().make_exec().unwrap(),
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

        // Compile the actual instructions
        for instruction in source {
            // Add each instruction's corresponding byte slice to the machine code
            self.machine_code.extend_from_slice(instruction.into());

            // TODO: record the index position of every forward and backward jump. We will use them to swap the jump addresses in the final memory
        }

        // Last: append the RET instruction
        self.machine_code.push(0xc3);

        // Finally: copy the machine code into the executable memory, and replace the jump instructions
        // Create an anonymous memory map the size of our machine code
        let mut temp_memory = MmapMut::map_anon(self.machine_code.len()).unwrap();
        temp_memory.clone_from_slice(&self.machine_code); // Clone the machine code into it

        // TODO: in this section, the memory can still be modified, but it already has is final addresses.
        // We can set and get the final jump addresses here.

        // Make the memory map executable
        self.executable_memory = temp_memory.make_exec().unwrap();
    }

    /// Execute the compiled machine code
    pub fn execute(&self) {
        assert!(!self.machine_code.is_empty(), "No machine code to execute");

        // Get a pointer to the machine code
        let func_ptr = self.executable_memory.as_ptr();

        unsafe {
            let main: fn() = std::mem::transmute(func_ptr);
            main();
        }
    }

    /// Clear the compiler from its previous run (reset the memory in place)
    pub fn clear(&mut self) {
        for i in 0..30_000 {
            self.memory[i] = 0;
        }
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Compiler::new()
    }
}
