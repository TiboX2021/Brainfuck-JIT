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

use crate::instructions::Instruction;

/// Implement conversion from basic instructions to machine code
/// NOTE: producing slices is more efficient than producing vectors
impl From<&Instruction> for &[u8] {
    fn from(instruction: &Instruction) -> Self {
        match instruction {
            Instruction::MoveRight => &[0x49, 0xff, 0xc5], // inc r13
            Instruction::MoveLeft => &[0x49, 0xff, 0xcd],  // dec r13
            Instruction::Increment => &[0x41, 0xfe, 0x45, 0x00], // inc byte ptr [r13] (size u8)
            Instruction::Decrement => &[0x41, 0xfe, 0x4d, 0x00], // dec byte ptr [r13] (size u8)
            Instruction::JumpBackwards => &[
                0x49, 0x8b, 0x45, 0x00, 0x84, 0xc0, 0x0f, 0x85, 0xff, 0xff, 0xff, 0xff,
            ],
            Instruction::JumpForward => &[
                0x49, 0x8b, 0x45, 0x00, 0x84, 0xc0, 0x0f, 0x84, 0xff, 0xff, 0xff, 0xff,
            ],
            Instruction::Output => &[
                0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, 0x48, 0xc7, 0xc7, 0x01, 0x00, 0x00, 0x00,
                0x4c, 0x89, 0xee, 0x48, 0xc7, 0xc2, 0x01, 0x00, 0x00, 0x00, 0x0f, 0x05,
            ], // mov rax, 1 | mov rdi, 1 | mov rsi, r13 | mov rdx, 1 | syscall (ie: print [r13])
        }
    }
}
