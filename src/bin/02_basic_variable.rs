// rust variables/data types
// cmd: rustc 02. basic variables.rs

fn main() {
    // Immutable by default
    let x = 5;
    println!("x = {}", x); // 5
                           // x = 6; // not allowed: Cannot assign a new value to an immutable variable more than once
    let x = x * 2; // Variable Shadowing - creating a new variable with the same name
    println!("x = {}", x); // 10

    // Mutable variable (use `mut` keyword)
    let mut y = 10;
    println!("y = {}", y);
    y = 20;
    println!("y = {}", y);

    // Type annotation
    let z: i32 = 30;
    println!("z = {}", z);

    let _unused_ok = 10; // no warning with _ unused
    let _der = 1.5;
    const PI: f64 = std::f64::consts::PI; // const
    println!("PI = {:.3}", PI); // print upto 3 decimal places

    //const DISCOUNT: f64 = 0.5;  // float

    //let x: i32 = "hello"; // rust-analyzer error ?

    // Declare multiple variables
    let (name, age) = ("Alice", 30);
    println!("{} is {} years old", name, age);

    // Type annotations with tuples
    let (x, y): (i32, f64) = (5, 3.14);
    println!("x = {}, y = {}", x, y);

    let (name, age): (&str, i32) = ("Alice", 30);
    let message: String = format!("{} is {} years old", name, age);
    println!("message = {}", message);

    // Multiple mutable variables
    let (mut a, mut b) = (1, 2);
    println!("a = {}, b = {}", a, b);
    a = 10;
    b = 20;
    println!("a = {}, b = {}", a, b);

    //let int_default = 42; // i32 by default
    //let arai = 2.5; // f64 by default
    //let float64 = std::f64::consts::E;

    let s1 = String::from("hello mom");
    let _s2 = s1.clone();
    println!("string = {}", s1);
    let mut s3: &str = "this is a test";
    println!("st2 = {}", s3);
    s3 = "changed";
    println!("s3 = {}", s3);

    // random number
    use fastrand::Rng;
    // in cargo.toml: fastrand = "2.3.0" 
    // The fastrand crate is often used in games, simulations, or any application that needs fast random numbers but doesn't require cryptographic security.
    // std::random is coming soon, or use rand crate https://crates.io/ (example in 07f_tokio_reqwest_rand_result.rs)
    let mut rng = Rng::new();
    let random1 = rng.u32(0..=2); // 0 to 2
    let random2 = fastrand::u32(0..10);  // 0 to 9
    let mut rng2 = Rng::new();
    let mut numbers = vec![1, 2, 3, 4, 5];
    rng2.shuffle(&mut numbers);
    println!("random nums = {}, {}", random1, random2);
    println!("shuffled numbers: {:?}", numbers);
}

// rustc "src\bin\02_basic_variable.rs" --crate-name run_program && .\run_program

/*
Key Differences: &str vs. &'static str vs. String

Type                Lifetime           Ownership Location
------------------- ------------------ --------- --------------------------
String              Until dropped      Owned     Heap
&str                Temporary ('a)     Borrowed  Anywhere (Heap/Stack)
&'static str        Program duration   Borrowed  Binary (Data Segment)
*/


/*
// raw string literal:
fn main() {
// Regular string with escapes
let s1 = "Hello\nWorld";

// Raw string - backslashes are treated literally
let s2 = r"Hello\nWorld";  // Contains actual characters: H e l l o \ n W o r l d

// Raw string with quotes inside
let s3 = r#"He said "Hello""#;  // No need to escape quotes
    println!("{}",s1);
    println!("{}",s2);
    println!("{}",s3);
}

output:
Hello
World
Hello\nWorld
He said "Hello"
*/

/*
&str:
  is a string slice in Rust
  Immutable reference to string data
  Borrowed, not owned - doesn't own the string data
  More efficient than String when you just need to read string data
*/