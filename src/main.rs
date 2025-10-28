use clap::Parser;
use sentencex::{get_sentence_boundaries, segment};
use std::fs;
use std::io::{self, Read};
use std::time::Instant;

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

    /// Print debug information including boundary details
    #[arg(short, long)]
    debug: bool,
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

    if cli.debug {
        let start_time = Instant::now();
        let boundaries = get_sentence_boundaries(&cli.language, &text);
        let elapsed = start_time.elapsed();

        eprintln!("Time taken for get_sentence_boundaries(): {:?}", elapsed);

        for (i, boundary) in boundaries.iter().enumerate() {
            println!("Boundary {}: ", i + 1);
            println!("  Start Index: {}", boundary.start_index);
            println!("  End Index: {}", boundary.end_index);
            println!("  Text: {:?}", boundary.text);
            println!("  Boundary Symbol: {:?}", boundary.boundary_symbol);
            println!("  Is Paragraph Break: {}", boundary.is_paragraph_break);
            println!();
        }
    } else {
        let start_time = Instant::now();
        let sentences = segment(&cli.language, &text);
        let elapsed = start_time.elapsed();

        eprintln!("Time taken for segment(): {:?}", elapsed);

        for sentence in sentences.iter() {
            println!("{}", sentence);
        }
    }
}
