// file reading
#![allow(dead_code, unused_variables)] // to silence unused code warnings

use std::fs;
use std::fs::File;
use std::io::Read;

use std::error::Error;

static READ_FILE_PATH: &str = "src/bin/rust_notes.txt";

fn main(){

    println!("File input example.");

    // match file_input2() {
    //     Ok(()) => println!("File operation completed successfully"),
    //     Err(e) => eprintln!("Error: {}", e),
    // }

    match file_input4() {
        Ok(st) => println!("File operation completed successfully."),
        Err(e) => eprintln!("Error: {}", e),
    }

    //file_input2().expect("Failed to read file");
    //file_input4().unwrap();

    // file_input1();
}


fn file_input1(){ // ok
   // Read and print the entire file content
    let content = fs::read_to_string(READ_FILE_PATH)
        .expect("Failed to read rust_notes.txt - make sure the file exists in the project root");
    println!("\n--- File Content ---");
    println!("{}", content);
    println!("--------------------"); 
}

fn file_input2() -> Result<(), Box<dyn Error>>{ // ok
    // Read file with ? for automatic error propagation
    let content = fs::read_to_string(READ_FILE_PATH)?;
    // The ? operator is the idiomatic way to propagate errors. 
    // It works inside any function whose return type is compatible with the 
    // error being returned (typically Result<T, E> or Option<T>).
    println!("\n File Content:");
    println!("{}", content);
    println!("--------------------");
    Ok(())   // Must return Ok at the end
    // No manual match or expect needed for propagation, Errors bubble up cleanly to main.
}

fn file_input3(){ // ok
    // with File::open()
    let mut file = File::open(READ_FILE_PATH)
        .expect("Failed to open rust_notes.txt");
    
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Failed to read the file");
    
    println!("{}", content);
}

fn file_input4() -> Result<String, Box<dyn Error>>{ // ok
    // same as file_input3, with return
    let mut f = File::open(READ_FILE_PATH)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    println!("File content: {}", s);
    Ok(s) // // Must return Ok at the end
}

fn file_input5(){ // ok
    // Basic handling with match
    let f = File::open(READ_FILE_PATH);

    let mut file = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Failed to read the file");
    println!("File content: {}", content);
}