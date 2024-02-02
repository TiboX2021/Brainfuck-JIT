//! JIT compiler implementation

use std::collections::HashMap;

use crate::{
    instructions::{ExtendedInstruction, Instruction},
    optimizer::{
        instructions_to_extended, optimize_instruction_repetitions, optimize_pattern_based,
    },
};
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

        let instructions = instructions_to_extended(source);
        let instructions = optimize_instruction_repetitions(&instructions);
        let instructions = optimize_pattern_based(&instructions);

        // Prepare [ to ] and ] to [ index hashmaps
        let mut forward_jumps: HashMap<usize, usize> = HashMap::new();
        let mut orphan_forwards: Vec<usize> = Vec::new();

        // Compile the actual instructions
        for instruction in instructions.iter() {
            // Record the jump instruction indexes in the machine_code array before insertion
            match instruction {
                ExtendedInstruction::Regular(Instruction::JumpForward) => {
                    orphan_forwards.push(self.machine_code.len())
                } // It is actually the next byte that marks the jump address
                ExtendedInstruction::Regular(Instruction::JumpBackwards) => {
                    let forward_index = orphan_forwards.pop().expect("Unmatched opening bracket");
                    forward_jumps.insert(forward_index, self.machine_code.len());
                }
                _ => {}
            }

            // Add each instruction's corresponding byte slice to the machine code
            let vec: Vec<u8> = instruction.into();
            self.machine_code.extend(vec);
        }

        assert!(
            orphan_forwards.is_empty(),
            "There exists unmatched opening brackets"
        );

        // Last: append the RET instruction
        self.machine_code.push(0xc3);

        // Finally: copy the machine code into the executable memory, and replace the jump instructions
        // Create an anonymous memory map the size of our machine code
        let mut temp_memory = MmapMut::map_anon(self.machine_code.len()).unwrap();
        temp_memory.clone_from_slice(&self.machine_code); // Clone the machine code into it

        // Record the memory address of each bracket jump instruction
        for (start_index, end_index) in forward_jumps.iter() {
            // Compute the memory addresses of the jump instructions
            let start_address = unsafe { temp_memory.as_ptr().add(*start_index) };
            let end_address = unsafe { temp_memory.as_ptr().add(*end_index) };

            // Compute the relative signed offset.
            let forward_offset = end_address as i32 - start_address as i32;
            let backwards_offset = -forward_offset;

            // Replace the jump addresses placeholders
            // The total jump instruction has 11 bytes. We want to replace the last 4: offset of 8
            temp_memory[start_index + 8..start_index + 12]
                .copy_from_slice(forward_offset.to_le_bytes().as_ref());
            temp_memory[end_index + 8..end_index + 12]
                .copy_from_slice(backwards_offset.to_le_bytes().as_ref());
        }

        // Make the memory map executable
        self.executable_memory = temp_memory.make_exec().unwrap();
    }

    /// Execute the compiled machine code
    pub fn execute(&self) {
        assert!(!self.machine_code.is_empty(), "No machine code to execute");

        // Get a pointer to the machine code
        let func_ptr = self.executable_memory.as_ptr();

        unsafe {
            let main: extern "C" fn() = std::mem::transmute(func_ptr);
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
