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


/*
unwrap vs ? operator, ? = The Error Propagation Operator/try operator

Feature          .unwrap()                              The ? Operator
-------------    -----------------------------------    ----------------------------------------
On Success       Returns the inner value.               Returns the inner value.
On Failure       Crashes (panics) the entire program.   Returns the error to the calling function.
Requirement      Can be used anywhere.                  The function must return a Result or Option.

When you use .unwrap(), you are telling the compiler: "I am 100% sure this will succeed. If it doesn't, just crash."
The ? operator is used for error propagation. Instead of crashing, it passes the error up the chain so the caller can decide how to handle it.

expect(msg): 
This is similar to .unwrap(), but it allows you to provide a custom error message.
Crashes the program immediately, No error propagation, Works in any function (no Result return needed)

Use ? — in production code, libraries, anywhere errors should be handled gracefully.
Use expect() — in tests, prototypes, main() setup, or when a failure truly is a bug (i.e., it should never happen).
*/