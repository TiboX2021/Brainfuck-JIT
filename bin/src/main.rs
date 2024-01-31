use clap::Parser;
use lib::interpreter::Interpreter;
use std::{path::Path, time::Instant};

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
    let filepath = Path::new(&args.source);

    // Execute the code in interpreter mode
    if args.interpret {
        let mut interp = Interpreter::new();
        interp.execute_file(filepath)?;
    } else {
        todo!("Implement brainfuck JIT");
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
