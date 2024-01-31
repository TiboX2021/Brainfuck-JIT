use clap::Parser;

#[derive(Parser, Debug)]
#[command(author="Thibaut de Saivre", version, about="JIT for brainfuck", long_about = None)]
struct Args {
    /// Source file
    #[arg()]
    source: String,
}

fn main() {
    let args = Args::parse();

    println!("Brainfuck JIT called for source {}", args.source);
}
