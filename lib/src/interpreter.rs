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
    /// Stack vector. Is initialized with 30 000 memory cells
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
            stack: Vec::with_capacity(30_000),
            forward_jumps: HashMap::new(),
            backward_jumps: HashMap::new(),
        }
    }

    /// Execute some brainfuck code from a byte iterator
    pub fn execute<I: IntoIterator<Item = u8>>(&mut self, bytes: I) {
        // Tokenize the program
        let program = tokenize_all(bytes);

        println!(
            "This program contains {} brainfuck instructions",
            program.len()
        );

        // Do a single forward pass over the whole code in order to match all loop brackets in the hash maps
        let mut bracket_indices: Vec<usize> = Vec::new(); // Store the encountered forward brackets on a stack

        for (index, instr) in program.into_iter().enumerate() {
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

        // TODO: Now execute the program
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
        self.stack.clear();
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
