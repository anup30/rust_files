// rust variables,
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
}

// rustc "02_basic_variable.rs" --crate-name run_program && .\run_program