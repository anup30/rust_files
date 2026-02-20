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

    Ok(())
}