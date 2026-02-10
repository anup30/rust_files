// rust follows snake_case for function and variable names
// fn keyword for function
// https://doc.rust-lang.org/rust-by-example/fn.html

fn greet(name: &str) {
    // Rust requires explicit type annotations for all parameters
    // functions can be defined anywhere in the scope, function overloading is not allowed in rust
    println!("Hello, {}!", name);
}

fn main() {
    // the main function is the entry point of every Rust program
    greet("Rustacean"); // Call the function
                        // return values
    let res: i32 = add(2, 3); // sending arguments
    println!("{}", res);
    let (sum, diff) = add_sub(10, 4); // multiple return
    println!("Sum: {}, Diff: {}", sum, diff);

    let mut word = String::from("hello");
    word += " world";
    let len = calculate_length(&word); // pass by reference, avoids ownership transfer, for non-Copy types (like String or Vec), pass by reference
    println!("Length of '{}' is {}", word, len); // word still valid
    let len = calculate_length_and_modify(&mut word); // pass by mutable reference
    println!("Length of '{}' is {}", word, len);
    let _len = calculate_length_full_ownership(word);
    // ^ pass by value, word is moved into the function, in main it no longer owns any valid data // different case for ints (as stays on stack)
    // println!("Length of '{}' is {}", word, len); // error: value borrowed here after move
    // or, let _len = calculate_length_full_ownership(word.clone()); // send a copy
    /*
      in function parameters:
      &String   → read-only view (very common, very safe)
      &mut String → read + write permission
      String     → full ownership (can mutate or destroy)
    */
}

fn add(a: i32, b: i32) -> i32 {
    // receives parameters
    return a + b; // explicit return, semicolon allowed here
}

fn add_sub(a: i32, b: i32) -> (i32, i32) {
    (a + b, a - b) // no semicolon, Implicit return of tuple
}

fn calculate_length(s: &String) -> usize {
    // read only
    s.len()
}

fn calculate_length_and_modify(s: &mut String) -> usize {
    // can modify
    s.push_str(" (modified)");
    s.len()
}

fn calculate_length_full_ownership(s: String) -> usize {
    // with taking ownership
    s.len()
}

// rustc "07. function.rs" --crate-name run_program && .\run_program
