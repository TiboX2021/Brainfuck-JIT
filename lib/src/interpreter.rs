//! Interpreter to run Brainfuck code.

use std::{
    collections::HashMap,
    io::{self},
    path::Path,
};

use crate::{instructions::Instruction, lexer::tokenize_all};

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

    /// Execute some brainfuck code from a byte iterator
    pub fn execute<I: IntoIterator<Item = u8>>(&mut self, bytes: I) {
        // Tokenize the program
        let program = tokenize_all(bytes);

        // Do a single forward pass over the whole code in order to match all loop brackets in the hash maps
        let mut bracket_indices: Vec<usize> = Vec::new(); // Store the encountered forward brackets on a stack

        for (index, instr) in program.iter().enumerate() {
            match instr {
                Instruction::JumpForward => bracket_indices.push(index),
                Instruction::JumpBackwards => {
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
        while instruction_pointer < program.len() {
            match program[instruction_pointer] {
                Instruction::MoveRight => self.stack_pointer += 1,
                Instruction::MoveLeft => self.stack_pointer -= 1,
                Instruction::Increment => {
                    self.stack[self.stack_pointer] = self.stack[self.stack_pointer].wrapping_add(1);
                }
                Instruction::Decrement => {
                    self.stack[self.stack_pointer] = self.stack[self.stack_pointer].wrapping_sub(1)
                }

                Instruction::Output => print!("{}", self.stack[self.stack_pointer] as char),
                Instruction::JumpForward => {
                    if self.stack[self.stack_pointer] == 0 {
                        instruction_pointer = self.forward_jumps[&instruction_pointer];
                    }
                }
                Instruction::JumpBackwards => {
                    if self.stack[self.stack_pointer] != 0 {
                        instruction_pointer = self.backward_jumps[&instruction_pointer];
                    }
                }
            }

            // Go to the next instruction
            instruction_pointer += 1;
        }
    }

    /// Execute brainfuck code from String slices
    pub fn execute_str(&mut self, s: &str) {
        self.execute(s.bytes())
    }

    /// Execute brainfuck code from a file
    /// In order to do this, we load and unwrap the whole file into a String.
    /// This allow handling File io errors before starting the execution.
    /// We assume that brainfuck programs are not large enough to mandate streaming support for files.
    pub fn execute_file(&mut self, path: &Path) -> io::Result<()> {
        let bytes = std::fs::read(path)?;
        self.execute(bytes);
        Ok(())
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
