//! Machine code helper

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
            Instruction::JumpBackwards => &[],             // TODO
            Instruction::JumpForward => &[],               // TODO
            Instruction::Output => &[],                    // TODO
        }
    }
}
