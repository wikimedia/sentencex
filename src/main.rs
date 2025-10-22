use clap::Parser;
use sentencex::segment;
use std::fs;
use std::io::{self, Read};

/// CLI for Sentence Segmentation
#[derive(Parser, Debug)]
#[command(name = "sentencex")]
#[command(about = "A tool for sentence segmentation", long_about = None)]
#[clap(version)]
struct Cli {
    /// Path to input file
    #[arg(short, long)]
    file: Option<String>,

    /// The language of the text
    #[arg(short, long, default_value = "en")]
    language: String,
}

fn main() {
    let cli = Cli::parse();

    let text = match cli.file {
        Some(file_path) => fs::read_to_string(file_path).expect("Failed to read file"),
        None => {
            // Read from stdin when no file is provided
            let mut buffer = String::new();
            io::stdin()
                .read_to_string(&mut buffer)
                .expect("Failed to read from stdin");
            buffer
        }
    };

    let sentences = segment(&cli.language, &text);

    for (i, sentence) in sentences.iter().enumerate() {
        println!("{}", sentence);
    }
}
