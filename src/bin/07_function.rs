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

    // Closures (Anonymous Functions)
    let add = |x: i32, y: i32| x + y;
    println!("{}", add(2, 3));

    // Multi-line closure
    let complex = |a, b| {
        let sum = a + b;
        let product = a * b;
        (sum, product)
    };
    
    let (s, p) = complex(3, 4);
    println!("Sum: {}, Product: {}", s, p);
    
    // Generic function
    let numbers = vec![34, 50, 12, 87, 99];
    println!("Largest number: {}", largest(&numbers));
    
    let chars = vec![ 'm', 'y', 'b', 'o', 't'];
    println!("Largest char: {}", largest(&chars));
    
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

// Generic function, works with multiple types
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// rustc "07_function.rs" --crate-name run_program && .\run_program


// powered by step 3.5, kimi K2.5



/*
// Error Handling with Result
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    match divide(10.0, 2.0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    // Using unwrap_or
    let result = divide(10.0, 0.0).unwrap_or(f64::NAN);
    println!("Result with fallback: {}", result);
}
*/