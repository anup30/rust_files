// cp input example
// problem: A. Sieve of Erato67henes: https://codeforces.com/contest/2195/problem/A

use std::io::{self, Read, BufWriter, Write};

fn main() {
    //println!("testing cp buf:");
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    
    let mut tokens = buf.split_whitespace().map(|x| x.parse::<i32>().unwrap());
    
    let tc = tokens.next().unwrap() as usize;
    let mut out = BufWriter::new(io::stdout());
    for _ in 0..tc {
        let n = tokens.next().unwrap() as usize;
        let mut found = false;
        for _ in 0..n {
            if tokens.next().unwrap() == 67 {
                found = true;
            }
        }
        //println!("{}", if found { "YES" } else { "NO" }); // ok
        writeln!(out, "{}", if found { "YES" } else { "NO" }).unwrap(); // faster
    }
    // let _ = io::stdin().read_line(&mut String::new()); // dont close program immediately
}

// buf:
/*
2
5
1 7 6 7 67
5
1 3 5 7 8
*/
// press ctrl+z then enter, in terminal to signal eof


// rustc "03c_cp_input.rs" --crate-name run_program && .\run_program  // run from src/bin/
// cargo run --bin 03c_cp_input  // run from project root  // can run from project root or src/bin/

// cmd: cargo run --bin 03c_cp_input < buf.txt
// cmd: rustc "03c_cp_input.rs" --crate-name run_program && .\run_program < buf.txt

// pwsh: cat src/bin buf.txt | cargo run --bin 03c_cp_input

// build with Kimi K2.5, Grok 4.2 // fast for large buf outputs