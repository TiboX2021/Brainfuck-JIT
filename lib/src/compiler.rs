//! JIT compiler implementation

use std::mem;

use crate::instructions::Instruction;

pub struct Compiler {
    /// Buffer for the generated machine code
    machine_code: Vec<u8>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            machine_code: Vec::new(),
        }
    }

    /// Compile the brainfuck source code into some machine code.
    /// The machine code will be stored inside this struct for later execution.
    pub fn compile(&mut self, source: &[Instruction]) {
        self.machine_code.clear();

        // TODO : compile
    }

    /// Execute the compiled machine code
    pub fn execute(&self) {
        assert!(!self.machine_code.is_empty(), "No machine code to execute");

        // Execute the machine code as a main function
        let main: fn() -> i32 = unsafe { mem::transmute(self.machine_code.as_ptr()) };
        main();
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Compiler::new()
    }
}
