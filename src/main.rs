use std::io;

fn main() {
    println!("Please input your words");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    println!("Word count of input: {}", word_count(input));
}

pub fn word_count(words: String) -> u128 {
    let mut count: u128 = 0;
    words
        .split(|c: char| !c.is_ascii_alphanumeric() && c != '\'')
        .filter(|word| !word.trim().is_empty())
        .for_each(|_word| {
            count += 1;
        });
    return count;
}
