//! Code optimization during compilation

use crate::instructions::{ExtendedInstruction, Instruction};

/// Convert regular brainfuck instructions to extended instructions for further processing
pub fn instructions_to_extended(instructions: &[Instruction]) -> Vec<ExtendedInstruction> {
    instructions
        .iter()
        .map(|i| ExtendedInstruction::Regular(*i))
        .collect()
}

/// Optimize instruction repetitions by aggregating them using extended instructions
pub fn optimize_instruction_repetitions(
    instructions: &[ExtendedInstruction],
) -> Vec<ExtendedInstruction> {
    let mut output = Vec::new();

    // Flags and counters to identify repeated instructions
    let mut current_instruction: Option<ExtendedInstruction> = None;
    let mut instruction_count: i32 = 0; // Count consecutive instructions to be grouped (arithmetic !)

    for instruction in instructions {
        // Check if we changed instructions
        match (instruction, current_instruction) {
            // Update the instruction count for Increment / Decrement instructions
            (
                ExtendedInstruction::Regular(Instruction::Increment),
                Some(ExtendedInstruction::Regular(Instruction::Increment))
                | Some(ExtendedInstruction::Regular(Instruction::Decrement)),
            ) => instruction_count += 1, // Increment = +1

            (
                ExtendedInstruction::Regular(Instruction::Decrement),
                Some(ExtendedInstruction::Regular(Instruction::Increment))
                | Some(ExtendedInstruction::Regular(Instruction::Decrement)),
            ) => instruction_count -= 1, // Decrement = -1

            // Update the instruction count for MoveRight / MoveLeft instructions
            (
                ExtendedInstruction::Regular(Instruction::MoveRight),
                Some(ExtendedInstruction::Regular(Instruction::MoveRight))
                | Some(ExtendedInstruction::Regular(Instruction::MoveLeft)),
            ) => instruction_count += 1, // Move Right = +1

            (
                ExtendedInstruction::Regular(Instruction::MoveLeft),
                Some(ExtendedInstruction::Regular(Instruction::MoveRight))
                | Some(ExtendedInstruction::Regular(Instruction::MoveLeft)),
            ) => instruction_count -= 1, // Move Left = -1

            _ => {
                // If we changed instructions, we need to process the previous instruction
                push_optimized_repeat_instruction(
                    &mut output,
                    &current_instruction,
                    instruction_count,
                );

                // Reset instruction count with the correct count depending on the conventions
                match instruction {
                    ExtendedInstruction::Regular(Instruction::Decrement)
                    | ExtendedInstruction::Regular(Instruction::MoveLeft) => instruction_count = -1,
                    _ => instruction_count = 1,
                }
            }
        }
        // Update current instruction
        current_instruction = Some(*instruction);
    }

    // Flush the buffer for the last instruction
    push_optimized_repeat_instruction(&mut output, &current_instruction, instruction_count);

    output
}

// Optimization patterns, listed in DECREASING PRIORITY ORDER
static PATTERNS: &[(&[ExtendedInstruction], ExtendedInstruction)] = &[
    // Set zero pattern: [-]
    (
        &[
            ExtendedInstruction::Regular(Instruction::JumpForward),
            ExtendedInstruction::Regular(Instruction::Decrement),
            ExtendedInstruction::Regular(Instruction::JumpBackwards),
        ],
        ExtendedInstruction::SetZero,
    ),
    // TODO : put other patterns here
];

/// Optimize the given instructions by recognizing patterns and replacing them with more efficient instructions
/// Example: `[-]` will be replaced by SetZero
pub fn optimize_pattern_based(instructions: &[ExtendedInstruction]) -> Vec<ExtendedInstruction> {
    let mut output = instructions.to_vec();
    let mut optimized_output = Vec::new();

    // For each optimization pattern, do a single pass through the instructions and replace them
    for (pattern, replacement) in PATTERNS {
        let mut matching_size = 0;

        // While there is still some output to process
        for (index, instruction) in output.iter().enumerate() {
            // MATCHING DETECTION
            // Check if the current instruction matches the pattern
            if *instruction == pattern[matching_size] {
                matching_size += 1;
            } else {
                // If the pattern doesn't match, we reset the matching size and move to the next instruction
                optimized_output.extend_from_slice(&output[index - matching_size..index + 1]);
                matching_size = 0;
            }

            // MATCHING PROCESSING
            // If we matched the whole pattern, we replace it with the optimized instruction
            if matching_size == pattern.len() {
                optimized_output.push(*replacement);

                // Reset the counters
                matching_size = 0;
            }
        }

        // If the matching size is not zero, we need to flush the buffer
        optimized_output.extend_from_slice(&output[output.len() - matching_size..]);

        // Swap the buffers
        output = optimized_output;
        optimized_output = Vec::new();
    }

    output
}

// ********************************************************************************************* //
//                                           HELPER FUNCTIONS                                    //
// ********************************************************************************************* //

/// Helper: pushes the optimized repeated instruction corresponding to the input and count inside the given buffer
fn push_optimized_repeat_instruction(
    buffer: &mut Vec<ExtendedInstruction>,
    instruction: &Option<ExtendedInstruction>,
    instruction_count: i32,
) {
    match (instruction, instruction_count) {
        (
            Some(ExtendedInstruction::Regular(Instruction::Increment))
            | Some(ExtendedInstruction::Regular(Instruction::Decrement)),
            instruction_count,
        ) if instruction_count > 1 => {
            buffer.push(ExtendedInstruction::Add(instruction_count as u8));
        }
        (
            Some(ExtendedInstruction::Regular(Instruction::Increment))
            | Some(ExtendedInstruction::Regular(Instruction::Decrement)),
            instruction_count,
        ) if instruction_count < -1 => {
            buffer.push(ExtendedInstruction::Sub(-instruction_count as u8));
        }
        (
            Some(ExtendedInstruction::Regular(Instruction::MoveRight))
            | Some(ExtendedInstruction::Regular(Instruction::MoveLeft)),
            instruction_count,
        ) if instruction_count > 1 => {
            buffer.push(ExtendedInstruction::JumpRight(instruction_count as u32));
        }
        (
            Some(ExtendedInstruction::Regular(Instruction::MoveRight))
            | Some(ExtendedInstruction::Regular(Instruction::MoveLeft)),
            instruction_count,
        ) if instruction_count < -1 => {
            buffer.push(ExtendedInstruction::JumpLeft(-instruction_count as u32));
        }
        _ => {
            // By default : we just add the current instruction to the output "as is"
            if let Some(instruction) = instruction {
                buffer.push(*instruction);
            }
        }
    }
}
