use std::io;

fn main() {
    println!("Please input your words");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    println!("Word count of input: {}", word_count(input));
}

pub fn word_count(words: String) -> usize {
    return words
        .split(|c: char| !c.is_ascii_alphanumeric() && c != '\'')
        .filter(|word| !word.trim().is_empty())
        .count()
}
