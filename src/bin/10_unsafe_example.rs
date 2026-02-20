// using rust unsafe block

/*
rust unsafe doesn't mean the code is dangerous, 
it means the compiler can't guarantee memory safety - the programmer must ensure it.
The compiler still enforces some checks in unsafe blocks (type system, ownership), 
but memory safety and thread safety become your responsibility.
*/


fn raw_pointer_dereferencing() {
    let mut num = 42;
    
    // Create raw pointers from references
    let r1 = &num as *const i32; // immutable raw pointer
    let r2 = &mut num as *mut i32; // mutable raw pointer
    let ptr: *const i32 = &num;
    if ptr.is_null() { // Check if pointer is null, dont need unsafe for .is_null()
        println!("The pointer is null!");
    } else {
        println!("The pointer is not null.");
    }
    let ptr2: *const i32 = std::ptr::null();
    
    unsafe {
        // Raw pointers allow aliasing (multiple pointers to same data) and can be null—Rust can't verify safety, so you must use unsafe.
        println!("Raw pointer value: {}", *r1);  // Dereferencing raw pointers
        *r2 = 100;
        println!("After modification: {}", *r1);
        println!("ptr address: {:p}", ptr); // 0x7b8c4ffbe4
        println!("ptr2 address: {:p}", ptr2); // 0x0
        println!("ptr2 is null: {}", ptr2.is_null()); // true
        // println!("ptr2 value: {}", *ptr2); // crashed: thread 'main' (10840) panicked at src\bin\test.rs:26:9: null pointer dereference occurred ...
    }
    unsafe {
        match ptr2.as_ref() { // unsafe required
            Some(value) => println!("ptr2 value is: {}", value),
            None => println!("ptr2 pointer is null"),
        }
        match r2.as_mut(){ // unsafe required
            Some(val) => println!("r2 value: {}", val),
            None => println!("r2 is null"),
        }
    }
}

unsafe extern "C" {
    // Binding to C standard library function
    fn abs(input: i32) -> i32;
    fn strlen(s: *const i8) -> usize;
    fn sqrt(x: f64) -> f64; 
}

static mut COUNTER: u32 = 0;

fn main() {
    // 1. Raw pointer dereferencing example
    raw_pointer_dereferencing();
    // 2. FFI (Foreign Function Interface) with external C code
    unsafe {
        // Calling foreign functions is unsafe because Rust can't check C code
        println!("Absolute value of -3 according to C: {}", abs(-3));
        let s = b"Rust\0".as_ptr() as *const i8;
        println!("Length: {}", strlen(s));
        println!("square root: {}", sqrt(6.25));
    }
    // 3. Mutable static variables
    unsafe {
        COUNTER += 1;
        let count = COUNTER;  // copy the value out first // because &COUNTER is forbidden in println! in rust 2024
        println!("Counter: {}", count);
    }
}
