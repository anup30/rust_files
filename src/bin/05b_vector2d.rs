// 2d vector

fn main() {
    let row_count = 3;
    let mut grid: Vec<Vec<char>> = (0..row_count).map(|_| Vec::new()).collect();
    // [[], [], []]
    grid[0].push('H');
    grid[0].push('i');
    
    grid[1].push('R');
    grid[1].push('u');
    grid[1].push('s');
    grid[1].push('t');
    
    grid[2].push('!');
    println!("{:?}", grid); // [['H', 'i'], ['R', 'u', 's', 't'], ['!']]
}

// rustc "src/bin/05b_vector2d.rs" --crate-name run_program && .\run_program 
// cargo run --bin 05b_vector2d

/*
// alternatively by vector of strings -----
fn main() {
    let row_count = 3;

    // Initialize 3 empty Strings
    let mut rows: Vec<String> = (0..row_count)
        .map(|_| String::new())
        .collect();

    // Push characters into specific strings
    rows[0].push('H');
    rows[0].push('i');
    
    rows[1].push_str("Rust"); // You can push entire strings too!
    
    rows[2].push('!');

    // Print the results
    for (i, row) in rows.iter().enumerate() {
        println!("Row {}: {}", i, row);
    }
}
*/