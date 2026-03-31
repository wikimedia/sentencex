use sentencex::{languages::English, segment};
fn main() {
    let language = English{};
    let text = "Hello world. This is a test.";
    let sentences = segment(&language, text);
    println!("Segmented sentences: {:?}", sentences);
}
