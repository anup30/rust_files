// Calling Unsafe Functions (FFI Example)

unsafe extern "C" {
    // Binding to C standard library function
    fn abs(input: i32) -> i32;
    fn strlen(s: *const i8) -> usize;
}

fn main() {
    unsafe {
        // Calling foreign functions is unsafe because Rust can't check C code
        println!("Absolute value of -3 according to C: {}", abs(-3));
        let s = b"Rust\0".as_ptr() as *const i8;
        println!("Length: {}", strlen(s));
    }
}