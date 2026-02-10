// started learning rust feb4, 2026
use std::io;

fn main() {
    println!("hello world! i am learning rust");
	print!("print without newline\n");
	
	println!("Press Enter to exit...");
    let _ = io::stdin().read_line(&mut String::new());
}

/*
cmd:
rustc --version
cargo --version
---
Main Rust binaries in C:\Users\Anup\.cargo\bin
compile and run: rustc file_name.rs && file_name
*/