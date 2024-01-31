use clap::Parser;
use lib::is_brainfuck_code;
use std::{
    fs::File,
    io::{BufReader, Read},
};

#[derive(Parser, Debug)]
#[command(author="Thibaut de Saivre", version, about="JIT for brainfuck", long_about = None)]
struct Args {
    /// Source file
    #[arg()]
    source: String,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let file = File::open(args.source)?;
    let reader = BufReader::new(file);

    // Note that brainfuck chars fit into ASCII, thus reading the file as bytes is enough
    // Iterate through each character
    for byte_result in reader.bytes() {
        // Unwrap the byte result
        let byte = byte_result?;

        // Convert byte to char
        let character = byte as char;

        if is_brainfuck_code(character) {
            // DEBUG: print the character. We would typically process it there
            println!("{}", character);
        }
    }

    Ok(())
}
