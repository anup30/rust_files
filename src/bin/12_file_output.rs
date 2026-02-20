// rust file output

use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("rust file output example");

    // Read input file (from previous example)
    let content = fs::read_to_string("src/bin/rust_notes.txt")?;

    // === WRITE TO A NEW FILE ===
    let output_path = "src/bin/output.txt";
    fs::write(output_path, content.clone())?;   // writes the entire content

    println!("\nSuccessfully wrote {} bytes to '{}'", content.len(), output_path);

    // Optional: also write a simple string
    fs::write("src/bin/hello.txt", "Hello from Rust!\nThis is a new file.\n")?;

    use_write_macro()?;
    append_file()?;
    Ok(())
}

use std::fs::File;
use std::io::Write;   // required for write! macro
use chrono;

fn use_write_macro() -> Result<(), Box<dyn std::error::Error>>{
    // useful when you want to write line-by-line or in a loop.
    let mut file = File::create("src/bin/output2.txt")   // creates or overwrites
        .expect("Failed to create output.txt");
    // Write line by line
    writeln!(file, "Rust writeln! Demo")?;
    writeln!(file, "=========================")?;
    writeln!(file, "Timestamp: {}", chrono::Utc::now())?;  // requires chrono crate, chrono = "0.4" in Cargo.toml
    // eg. Timestamp: 2026-02-20 15:00:59.846352900 UTC

    println!("Content written to output.txt");
    Ok(())
}

use std::fs::OpenOptions;

fn append_file()-> Result<(), Box<dyn std::error::Error>>{
    let mut file = OpenOptions::new()
    .write(true)
    .append(true)      // ← key difference
    .create(true)      // create if file doesn't exist
    .open("src/bin/output2.txt")?;

    writeln!(file, "New append at: {}", chrono::Utc::now())?;
    Ok(())
}
