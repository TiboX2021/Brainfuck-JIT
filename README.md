# Brainfuck JIT in Rust

Author: Thibaut de Saivre

A simple brainfuck interpreter and JIT implemented as an exercize for course INF559 at Ecole Polytechnique.
The few implemented optimizations were taken from https://bfc.wilfred.me.uk/.

Run with

```bash
cargo run -- -h
cargo run -- examples/mandelbrot.bf -t
```

## Projet structure

This project uses `cargo workspaces`.

```bash
├── bin           # CLI executable
│   └── main
├── examples      # Example brainfuck programs
└── lib           # Implementation logic
    ├── compiler      # JIT compiler implementation
    ├── instructions  # Instructions Enum definitions
    ├── interpreter   # Interpreter implementation
    ├── lexer         # Simple tokenization function
    ├── lib           # Root lib module
    ├── optimizer     # JIT optimization functions
    └── x86_64        # Conversion from tokens to machine code
```

## Optimizations performed

### Interpreter optimizations

- Identify jump target indexes in the source code using a linear scan before execution.

### JIT compiler optimizations

- Regroup `+` and `-` instructions
- Regroup `>` and `<` instructions
- Replace `[-]` loops with a "set to 0" instruction
