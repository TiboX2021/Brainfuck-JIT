/// Brainfuck instruction symbols
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    MoveRight,
    MoveLeft,
    Increment,
    Decrement,
    Output,
    // Input,
    JumpForward,
    JumpBackwards,
}

impl TryFrom<char> for Instruction {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '>' => Ok(Instruction::MoveRight),
            '<' => Ok(Instruction::MoveLeft),
            '+' => Ok(Instruction::Increment),
            '-' => Ok(Instruction::Decrement),
            '.' => Ok(Instruction::Output),
            // ',' => Ok(Instruction::Input),
            '[' => Ok(Instruction::JumpForward),
            ']' => Ok(Instruction::JumpBackwards),
            _ => Err("Unknown instruction character"),
        }
    }
}

impl From<Instruction> for char {
    fn from(val: Instruction) -> Self {
        match val {
            Instruction::MoveRight => '>',
            Instruction::MoveLeft => '<',
            Instruction::Increment => '+',
            Instruction::Decrement => '-',
            Instruction::Output => '.',
            Instruction::JumpForward => '[',
            Instruction::JumpBackwards => ']',
        }
    }
}

/// Extended instructions that are not part of the original brainfuck language,
/// but that will be generated via code optimization.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExtendedInstruction {
    Regular(Instruction),

    // Factorized instructions
    Add(u8),
    Sub(u8),
    JumpRight(u32),
    JumpLeft(u32),
    SetZero,
}
