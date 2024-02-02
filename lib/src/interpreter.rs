//! Interpreter to run Brainfuck code.

use std::collections::HashMap;

use crate::{
    instructions::{ExtendedInstruction, Instruction},
    optimizer::{
        instructions_to_extended, optimize_instruction_repetitions, optimize_pattern_based,
    },
};

/// An implementation of a Brainfuck interpreter
pub struct Interpreter {
    /// Stack pointer internal variable
    stack_pointer: usize,
    /// Stack vector. Is initialized with 30 000 memory cells at 0
    stack: Vec<u8>,

    /// Hashmap that associates each '[' bracket index with its corresponding ']' bracket index
    forward_jumps: HashMap<usize, usize>,
    /// Hashmap that associates each ']' bracket index with its corresponding '[' bracket index
    backward_jumps: HashMap<usize, usize>,
}

#[allow(dead_code)]
impl Interpreter {
    /// Build a new interpreter
    pub fn new() -> Self {
        Self {
            stack_pointer: 0,
            stack: vec![0; 30_000],
            forward_jumps: HashMap::new(),
            backward_jumps: HashMap::new(),
        }
    }

    /// Execute some brainfuck code from a tokenized program
    pub fn execute(&mut self, program: &[Instruction]) {
        let instructions = instructions_to_extended(program);
        let instructions = optimize_instruction_repetitions(&instructions);
        let instructions = optimize_pattern_based(&instructions);

        // Do a single forward pass over the whole code in order to match all loop brackets in the hash maps
        let mut bracket_indices: Vec<usize> = Vec::new(); // Store the encountered forward brackets on a stack

        for (index, instr) in instructions.iter().enumerate() {
            match instr {
                ExtendedInstruction::Regular(Instruction::JumpForward) => {
                    bracket_indices.push(index)
                }
                ExtendedInstruction::Regular(Instruction::JumpBackwards) => {
                    let forward_index = bracket_indices.pop().expect("Unmatched closing bracket");
                    self.forward_jumps.insert(forward_index, index);
                    self.backward_jumps.insert(index, forward_index);
                }
                _ => {}
            }
        }

        assert!(
            bracket_indices.is_empty(),
            "There exists unmatched opening brackets"
        );

        // Now, we can execute the program until the instructions run out
        let mut instruction_pointer: usize = 0;
        while instruction_pointer < instructions.len() {
            match instructions[instruction_pointer] {
                ExtendedInstruction::Regular(Instruction::MoveRight) => self.stack_pointer += 1,
                ExtendedInstruction::Regular(Instruction::MoveLeft) => self.stack_pointer -= 1,
                ExtendedInstruction::Regular(Instruction::Increment) => {
                    self.stack[self.stack_pointer] = self.stack[self.stack_pointer].wrapping_add(1);
                }
                ExtendedInstruction::Regular(Instruction::Decrement) => {
                    self.stack[self.stack_pointer] = self.stack[self.stack_pointer].wrapping_sub(1)
                }

                ExtendedInstruction::Regular(Instruction::Output) => {
                    print!("{}", self.stack[self.stack_pointer] as char)
                }
                ExtendedInstruction::Regular(Instruction::JumpForward) => {
                    if self.stack[self.stack_pointer] == 0 {
                        instruction_pointer = self.forward_jumps[&instruction_pointer];
                    }
                }
                ExtendedInstruction::Regular(Instruction::JumpBackwards) => {
                    if self.stack[self.stack_pointer] != 0 {
                        instruction_pointer = self.backward_jumps[&instruction_pointer];
                    }
                }
                ExtendedInstruction::Add(n) => {
                    self.stack[self.stack_pointer] = self.stack[self.stack_pointer].wrapping_add(n)
                }
                ExtendedInstruction::Sub(n) => {
                    self.stack[self.stack_pointer] = self.stack[self.stack_pointer].wrapping_sub(n)
                }
                ExtendedInstruction::JumpLeft(n) => self.stack_pointer -= n as usize,
                ExtendedInstruction::JumpRight(n) => self.stack_pointer += n as usize,
                ExtendedInstruction::SetZero => self.stack[self.stack_pointer] = 0,
            }

            // Go to the next instruction
            instruction_pointer += 1;
        }
    }

    /// Clear the interpreter state from its previous execution
    pub fn clear(&mut self) {
        self.stack_pointer = 0;
        self.stack = vec![0; 30_000];
        self.backward_jumps.clear();
        self.forward_jumps.clear();
    }
}

// Implement the Default trait
impl Default for Interpreter {
    fn default() -> Self {
        Interpreter::new()
    }
}
