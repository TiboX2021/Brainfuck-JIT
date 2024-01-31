//! Simple lexer utils that help convert a byte stream into a brainfuck Instruction stream.

use crate::instructions::Instruction;

/// Convert an iterator over bytes into an iterator over brainfuck Instructions
pub fn tokenize<I: IntoIterator<Item = u8>>(bytes: I) -> impl Iterator<Item = Instruction> {
    bytes
        .into_iter()
        .filter_map(|c| match Instruction::try_from(c as char) {
            Ok(i) => Some(i),
            _ => None,
        })
}

/// Convert an iterator over bytes into a collected Vec<Instruction>
pub fn tokenize_all<I: IntoIterator<Item = u8>>(bytes: I) -> Vec<Instruction> {
    tokenize(bytes).collect()
}
