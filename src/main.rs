use std::io::stdin;

fn main() {
    println!("Welcome to the first word finder.");
    println!("This program finds the first word of your input string.");
    println!("Please enter your string:");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line.");
    let input = input.trim();
    let f_word = first_word(input);
    println!("First word is: {f_word}");
}

fn first_word(input: &str) -> &str {
    let input_as_bytes = input.as_bytes();
    let input_enum = input_as_bytes.iter().enumerate();
    for (index, &byte_char) in input_enum {
        if byte_char == b' ' {
            return &input[0..index];
        }
    }
    &input[..]
}
