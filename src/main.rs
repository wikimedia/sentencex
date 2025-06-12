use clap::Parser;
use sentencex::{LanguageOption, SentenceSegmenter};

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
    #[arg(short, long, default_value_t, value_enum)]
    language: LanguageOption,
}

fn main() {
    let cli = Cli::parse();

    let segmenter = SentenceSegmenter::new(cli.language);
    let sentences = segmenter.segment(&cli.text);

    for (i, sentence) in sentences.iter().enumerate() {
        println!("{}. {}", i + 1, sentence);
    }
}
