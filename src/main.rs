mod display_file_content;
mod read_file;
use std::io::{stdout, Write};

use clap::Parser;
use read_file::read_file;

#[derive(Parser)]
struct Cli {
    /// file
    file: String,
    #[arg(long, short)]
    /// number all output lines
    number: bool,
    //#[arg(long, short)]
}

fn main() {
    let args = Cli::parse();

    let path = args.file.clone();

    if args.number {
        display_file_content::lines(path.clone()).unwrap_or_else(|e| {
            eprintln!("Failed to display file: {e}");
        });
    } else {
        let content = read_file(&path).unwrap_or_else(|e| {
            eprintln!("Failed to read file: {e}");
            std::process::exit(1);
        });
        stdout().lock().write(&content).unwrap_or_else(|e| {
            eprintln!("Failed: {e}");
            std::process::exit(1);
        });
    }
}
