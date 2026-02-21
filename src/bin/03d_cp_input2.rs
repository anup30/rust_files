// problem: B. Heapify 1: https://codeforces.com/contest/2195/problem/B // Accepted
// my first rust problem solution

use std::io::{self, Read, BufWriter, Write};

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    
    let mut tokens = buf.split_whitespace().map(|x| x.parse::<usize>().unwrap());
    
    let tc = tokens.next().unwrap() as usize;
    let mut out = BufWriter::new(io::stdout());

    const SZ: usize = 200001; // 2e5+1

    let mut pos: Vec<usize> = Vec::with_capacity(SZ);
    let mut num:usize;

    for _ in 0..tc {
        let n = tokens.next().unwrap() as usize;
        pos.resize(SZ, 0);
        let mut is_perm = true;

        for i in 1..=n {
            num = tokens.next().unwrap();
            pos[num as usize] = i;
        }        
        // println!("pos: {:?}", pos);
        for i in 1..=n { // main loop
            let mut p = pos[i];
            if i>p {
                while i>p{p*=2;}
                if i != p {
                    is_perm = false;
                    break; // break1, even though nested ifs, breaks innermost enclosing loop. similar in c/c++/python
                }
            }else if i<p{
                while i<p{p/=2;}
                if i != p {
                    is_perm = false;
                    break; // break2
                }
            }
        }
        writeln!(out, "{}", if is_perm { "YES" } else { "NO" }).unwrap();
        pos.clear();
    }
    
}

// input:
/*
2
5
1 4 3 2 5
5
1 4 2 3 5
*/

/*
2
3
2 1 3
4
2 4 3 1

YES
YES
*/


// press ctrl+z enter ctrl+z, in terminal to signal eof

// pwsh:  cat src/bin/input.txt | cargo run --bin 03d_cp_input2

