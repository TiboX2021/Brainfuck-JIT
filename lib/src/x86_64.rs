//! Machine code helper
//!
//! This module implements From and Into (implicit) for all instructions in order to produce machine code.
//!
//! Some details about the more complex calls:
//!
//! Output - print the current cell - syscall to `print`
//! ```
//! mov rax, 1      ; 0x48 0xc7 0xc0 0x01 0x00 0x00 0x00
//! mov rdi, 1      ; 0x48 0xc7 0xc7 0x01 0x00 0x00 0x00
//! mov rsi, r13    ; 0x4c 0x89 0xee
//! mov rdx, 1      ; 0x48 0xc7 0xc2 0x01 0x00 0x00 0x00
//! syscall         ; 0x0f 0x05
//! ```
//!
//! Jump Forward - jump to the matching `]` if the current cell is 0
//!
//! We put the value of the current cell `[r13]` into the `rax` register so that we can access its lower byte using `al`.
//! We can then set the jump flags using `test`, and add the jump instruction.
//! We use `0xff` as placeholder addresses, that will be resolved during the second pass.
//!
//! Note that the `jz` instruction accepts a signed 64-bit offset (8 bytes) as an argument.
//! ```
//! mov rax, [r13]  ; 0x49 0x8B 0x45 0x00
//! test al, al     ; 0x84 0xc0
//! jz xxx          ; 0x0f 0x84 0xff 0xff 0xff 0xff
//! ```
//! The last `0xff` instructions will need to be replaced with the actual offset.
//!
//! Jump Backwards - jump to the matching `[` if the current cell is not 0
//!
//! We put the value of the current cell `[r13]` into the `rax` register so that we can access its lower byte using `al`.
//! We can then set the jump flags using `test`, and add the jump instruction.
//! We use `0xff` as placeholder addresses, that will be resolved during the second pass.
//!
//! Note that the `jnz` instruction accepts a signed 64-bit offset (8 bytes) as an argument.
//! ```
//! mov rax, [r13]  ; 0x49 0x8B 0x45 0x00
//! test al, al     ; 0x84 0xc0
//! jnz xxx          ; 0x0f 0x85 0xff 0xff 0xff 0xff
//! ```
//! The last `0xff` instructions will need to be replaced with the actual offset.

use crate::instructions::{ExtendedInstruction, Instruction};

/// Implement conversion from basic instructions to machine code
impl From<&Instruction> for Vec<u8> {
    fn from(instruction: &Instruction) -> Self {
        match instruction {
            Instruction::MoveRight => vec![0x49, 0xff, 0xc5], // inc r13
            Instruction::MoveLeft => vec![0x49, 0xff, 0xcd],  // dec r13
            Instruction::Increment => vec![0x41, 0xfe, 0x45, 0x00], // inc byte ptr [r13] (size u8)
            Instruction::Decrement => vec![0x41, 0xfe, 0x4d, 0x00], // dec byte ptr [r13] (size u8)
            Instruction::JumpBackwards => vec![
                0x49, 0x8b, 0x45, 0x00, 0x84, 0xc0, 0x0f, 0x85, 0xff, 0xff, 0xff, 0xff,
            ],
            Instruction::JumpForward => vec![
                0x49, 0x8b, 0x45, 0x00, 0x84, 0xc0, 0x0f, 0x84, 0xff, 0xff, 0xff, 0xff,
            ],
            Instruction::Output => vec![
                0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, 0x48, 0xc7, 0xc7, 0x01, 0x00, 0x00, 0x00,
                0x4c, 0x89, 0xee, 0x48, 0xc7, 0xc2, 0x01, 0x00, 0x00, 0x00, 0x0f, 0x05,
            ], // mov rax, 1 | mov rdi, 1 | mov rsi, r13 | mov rdx, 1 | syscall (ie: print [r13])
        }
    }
}

/// Implement conversion from extended instructions to machine code
impl From<&ExtendedInstruction> for Vec<u8> {
    fn from(instruction: &ExtendedInstruction) -> Self {
        match instruction {
            ExtendedInstruction::Regular(instruction) => instruction.into(),
            ExtendedInstruction::Add(count) => vec![
                0x41, 0x83, 0x45, 0x00, *count, // add byte ptr [r13], count
            ],
            ExtendedInstruction::Sub(count) => vec![
                0x41, 0x80, 0x6d, 0x00, *count, // sub byte ptr [r13], count
            ],
            ExtendedInstruction::JumpLeft(offset) => {
                let mut bytes = Vec::new();

                if *offset <= 0xff {
                    // sub with 1 byte format
                    bytes.extend_from_slice(&[0x49, 0x83, 0xed]);
                    bytes.push(*offset as u8);
                } else {
                    // sub with 4 bytes format
                    bytes.extend_from_slice(&[0x49, 0x81, 0xed]);
                    bytes.extend(&offset.to_be_bytes());
                }

                bytes
            } // sub r13, offset
            ExtendedInstruction::JumpRight(offset) => {
                let mut bytes = Vec::new();

                if *offset <= 0xff {
                    // add with 1 byte format
                    bytes.extend_from_slice(&[0x49, 0x83, 0xC5]);
                    bytes.push(*offset as u8);
                } else {
                    // add with 4 bytes format
                    bytes.extend_from_slice(&[0x49, 0x81, 0xC5]);
                    bytes.extend(&offset.to_be_bytes());
                }

                bytes
            } // add r13, offset
        }
    }
}
