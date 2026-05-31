// HashMap<K, V>  frequency counter example

use std::collections::HashMap; // must explicitly import

fn main() {
    let text = "hello world rust programming";

    // Create a HashMap that maps char -> count (usize)
    let mut char_count: HashMap<char, usize> = HashMap::new();

    // Count frequency of each character
    for c in text.chars() {
        if c.is_alphanumeric() || c.is_whitespace() {  // optional: filter characters
            *char_count.entry(c).or_insert(0) += 1; // entry api, increase value by 1 if key exists, otherwise insert 0 and then increase by 1 -----
        }
    }
    // char_count.insert('b', 20);
    
    // Print all character frequencies
    println!("All character frequencies:");
    for (ch, count) in &char_count {
        println!("'{}': {}", ch, count);
    }

    // === Check if 'r' exists and save count as i32 ===
    let r_count: i32;

    if let Some(&count) = char_count.get(&'r') {
        r_count = count as i32;        // Convert usize to i32
        println!("\n'r' exists! Count saved as i32: {}", r_count);
    } else {
        r_count = 0;
        println!("\n'r' does not exist. Count set to 0.");
    }

    // use r_count as i32 variable anywhere below
    println!("Final r_count (i32): {}", r_count);
    // one line versions
    let r1: &usize = char_count.get(&'r').unwrap_or(&0); // &usize, map keeps ownership
    let z1: usize = *char_count.get(&'z').unwrap_or(&0); // usize, dereferenced value
    let o1: usize = char_count.get(&'o').copied().unwrap_or(0); // usize, copied value
    println!("r1, z1, o1 : {}, {}, {}", r1, z1, o1);    
    
}


// rustc "src\bin\06b_HashMap.rs" --crate-name run_program && .\run_program

