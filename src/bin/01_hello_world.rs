// started learning rust feb 4, 2026
use std::io;

fn main() {
    println!("hello world! i am learning rust");
    print!("print without newline\n");

    println!("Press Enter to exit...");
    let _ = io::stdin().read_line(&mut String::new()); // dont close program immediately
}

/*
cmd:
rustc --version
cargo --version
Main Rust binaries in C:\Users\Anup\.cargo\bin
---
cargo init : creates Cargo.toml file
cargo run --bin 01_hello world (run the specific file, if kept in src/bin/)
---
or, run by single click code runner icon // in settings.json, in "code-runner.executorMap" : "rust": "cd $dir && rustc \"$fileName\" --crate-name run_program && .\\run_program.exe",
or, run button on main function by rust-analyzer extension
or, compile and run in pwsh:
  rustc "01_hello world.rs" --crate-name run_program && .\run_program
  (rustc file_name.rs && file_name)
*/

