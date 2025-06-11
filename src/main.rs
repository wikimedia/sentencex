use clap::Parser;
use sentencex::{LanguageOption, SentenceSegmenter};

/// CLI for Sentence Segmentation
#[derive(Parser)]
#[command(name = "sentencex")]
#[command(about = "A CLI tool for sentence segmentation", long_about = None)]
struct Cli {
    /// The text to segment
    #[arg(short, long)]
    text: String,

    /// The language of the text
    #[arg(short, long, default_value = "English")]
    language: String,
}

fn main() {
    let cli = Cli::parse();

    let language = match cli.language.to_lowercase().as_str() {
        "english" => LanguageOption::English,
        "spanish" => LanguageOption::Spanish,
        "malayalam" => LanguageOption::Malayalam,
        "portuguese" => LanguageOption::Portuguese,
        "italian" => LanguageOption::Italian,
        "amharic" => LanguageOption::Amharic,
        _ => {
            eprintln!("Unsupported language: {}", cli.language);
            std::process::exit(1);
        }
    };

    let segmenter = SentenceSegmenter::new(language);
    let sentences = segmenter.segment(&cli.text);

    for (i, sentence) in sentences.iter().enumerate() {
        println!("{}. {}", i + 1, sentence);
    }
}
