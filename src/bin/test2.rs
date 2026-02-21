// 

use std::io::{self, Read};

fn main() {
    println!("testing cp input:");
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut tokens = input.split_whitespace();

    let tc: usize = tokens.next().unwrap().parse().unwrap();

    for _ in 0..tc {
        let n: usize = tokens.next().unwrap().parse().unwrap();

        let mut has_67 = false;
        for _ in 0..n {
            let val: i32 = tokens.next().unwrap().parse().unwrap();
            if val == 67 {
                has_67 = true;
            }
        }

        println!("{}", if has_67 { "YES" } else { "NO" });
    }
    let _ = io::stdin().read_line(&mut String::new()); // dont close program immediately
}

// input:
/*
2
5
1 7 6 7 67
5
1 3 5 7 8
*/
// press ctrl+z then enter, in terminal to signal eof


// rustc "test2.rs" --crate-name run_program && .\run_program  // run from src/bin/
// cargo run --bin test2  // run from project root  // can run from project root or src/bin/