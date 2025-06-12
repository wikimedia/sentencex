use clap::Parser;
use sentencex::segment;

/// CLI for Sentence Segmentation
#[derive(Parser, Debug)]
#[command(name = "sentencex")]
#[command(about = "A CLI tool for sentence segmentation", long_about = None)]
#[clap(version)]
struct Cli {
    /// The text to segment
    #[arg(short, long)]
    text: String,

    /// The language of the text
    #[arg(short, long, default_value = "en")]
    language: String,
}

fn main() {
    let cli = Cli::parse();

    let sentences = segment(&cli.language, &cli.text);

    for (i, sentence) in sentences.iter().enumerate() {
        println!("{}. {}", i + 1, sentence);
    }
}
