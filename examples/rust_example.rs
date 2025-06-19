use sentencex::segment;

fn main() {
    let language_code = "en";
    let text = "Hello world. This is a test.";
    let sentences = segment(language_code, text);

    println!("Segmented sentences: {:?}", sentences);
}
