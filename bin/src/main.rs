use clap::Parser;
use lib::{compiler::Compiler, interpreter::Interpreter, lexer::tokenize_all};
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(author="Thibaut de Saivre", version, about="JIT for brainfuck", long_about = None)]
struct Args {
    /// Source file
    #[arg()]
    source: String,

    /// Measure execution time
    #[arg(short, long)]
    time: bool,

    /// Execute the program in interpreter mode, rather than JIT
    #[arg(short, long)]
    interpret: bool,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let start_time = Instant::now();

    // Read the entire source code into a byte array
    let bytes = std::fs::read(&args.source)?;

    // Tokenize the source code and remove invalid instructions
    let source_code = tokenize_all(bytes);

    if args.interpret {
        // Execute the code in interpreter mode
        let mut interpreter = Interpreter::new();
        interpreter.execute(&source_code);
    } else {
        // Execute the code in JIT mode
        let mut compiler = Compiler::new();
        compiler.compile(&source_code);
        compiler.execute();
    }

    // Measure the elapsed time
    let elapsed_time = start_time.elapsed();

    // Print elapsed time if the flag is set
    if args.time {
        // Print the elapsed time in seconds and milliseconds
        println!(
            "Elapsed time: {}.{:09} seconds",
            elapsed_time.as_secs(),
            elapsed_time.subsec_nanos()
        );
    }

    Ok(())
}
