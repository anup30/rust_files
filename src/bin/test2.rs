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


// rustc "src/bin/test2.rs" --crate-name run_program && .\run_program 
// cargo run --bin test2  // run from project root  // can run from project root or src/bin/