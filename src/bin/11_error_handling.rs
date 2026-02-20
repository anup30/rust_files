// error handling

/*
rust does not have exceptions, try catch block.
Rust employs an explicit, type-safe approach that avoids exceptions entirely. 
Errors fall into two categories: unrecoverable (handled via the panic! macro) 
and recoverable (handled via the Result<T, E> type). 
The ? operator enables concise error propagation.
*/


// Example of a defensive panic in a constructor
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess { value }
    }
}

fn main() {
    println!("rust error handling example");
    // panic!("crash and burn");  // Explicit panic
    let g1 = Guess::new(50);
    println!("Guess value: {}", g1.value);
    // see file_io.rs for file input/output examples
    

}

/*
When to use panic! 
Use panic! when an assumption or contract is violated and recovery is impossible or would leave the program in an invalid state.
Prefer returning Result for expected, recoverable failures (e.g., invalid user input, file-not-found).
In library code, document panic conditions clearly in API documentation.
Acceptable in prototypes, tests, or when you have proven (and documented) that a failure cannot occur.
*/