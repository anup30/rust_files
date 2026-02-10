// user input output
use std::io; // library to interact with input/output.

fn main() {
    let mut buffer = String::new();
    println!("Enter your name:");
    io::stdin()
        .read_line(&mut buffer) // Pass a mutable reference to the variable
        .expect("Failed to read line"); // Handle potential errors
    let name = buffer.trim().to_string(); // new string without newline
    buffer.clear();
    println!("Hello, {}!", name);
    println!("Enter your age:");
    std::io::stdin().read_line(&mut buffer).unwrap(); // unwrap() is similar to expect()
    let age: i32 = buffer.trim().parse().expect("Please type a whole number!");
    println!("next year you will be {} years old", age + 1);
}

// rustc "03. user input.rs" --crate-name run_program && .\run_program
