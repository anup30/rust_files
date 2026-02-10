// input with helper function
use std::io::{self, Write}; // Import Write to use flush()

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    // Ensure the prompt displays before waiting for input
    io::stdout().flush().expect("Could not flush stdout");

    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");

    buffer.trim().to_string() // return without semicolon
}

fn main() {
    let name = get_input("Enter your name: ");

    let age_str = get_input("Enter your age: ");
    let age: i32 = age_str.parse().expect("Please type a number!");

    println!("Hello, {}! Next year you will be {}.", name, age + 1);
}

// rustc "03.1 user input.rs" --crate-name run_program && .\run_program
