// input taking using macro_rules! - scanln and next (can use any)
// idea from: https://codeforces.com/contest/2227/submission/373136651 (by silicalet)
// problem: round 1096 - E. It All Went Sideways, https://codeforces.com/contest/2227/problem/E  

#![allow(unused, non_snake_case, dead_code)]
use std::io::{BufRead, Read, Write, stdout, stdin};


/// Reads an entire line, splits it by whitespace, and parses elements into a Vec<T>
/// Usage: let row = scanln!(i32); // or single element, let st = scanln!(String)[0];

macro_rules! scanln {
    ($t:ty) => {{ // when a macro has ($t:ty), it means you must pass the type inside the macro call
        let ln: String = stdin().lock().lines()
            .skip_while(|x| x.as_ref().unwrap().is_empty())
            .next().unwrap().unwrap();
        let v: Vec<$t> = ln.split_whitespace().map(|x| x.parse().unwrap()).collect();
        v
    }};
}

macro_rules! next {
    () => {{
        use std::io::Read;
        let mut token = stdin().lock().bytes()
            .skip_while(|x| x.as_ref().unwrap().is_ascii_whitespace())
            .take_while(|x| !x.as_ref().unwrap().is_ascii_whitespace())
            .map(|x| x.unwrap() as char)
            .collect::<String>();
        token.parse().unwrap()
    }};
}

fn main() {    
    let T: usize = next!(); // let T: usize = scanln!(usize)[0];
    for _ in 1..=T {
        solve();
    }
}

fn solve(){
    let mut out = stdout().lock();

    let n:usize = scanln!(usize)[0]; // or, let n:usize = next!();
    let a: Vec<usize> = scanln!(usize); // or, let a: Vec<usize> = (0..n).map(|_| next!()).collect();


    let mut suf = vec![0; n]; // suffix minimum array
    suf[n - 1] = a[n - 1];
 
    for i in (0..n - 1).rev() {
        suf[i] = suf[i + 1].min(a[i]);
    }
 
    let sum: usize = a.iter().sum();
    let tmp: usize = suf.iter().sum();
    let b = sum - tmp;
 
    let mut cnt = vec![0usize; n << 1 | 1];
    let mut ans = 0;
 
    for i in 0..n {
        cnt[suf[i]] += 1;
        if cnt[a[i]] < 1 {
            continue;
        }
        ans = ans.max(cnt[a[i]] - 1);
    } 
    writeln!(out, "{}", b + ans);
}



/*
// if input given from terminal, press enter + ctrl+z, in terminal to signal eof

// from project root:
  pwsh7:  cat src/bin/input.txt | cargo run --bin 04b_cp_input2
  pwsh7 :  rustc "src/bin/04b_cp_input2.rs" --crate-name run_program && .\run_program
  pwsh7 :  cat src/bin/input.txt | rustc "src/bin/04b_cp_input2.rs" --crate-name run_program && .\run_program

// from containing folder:
  pwsh5: rustc -O "04b_cp_input2.rs" --crate-name run_program; Get-Content input.txt | .\run_program.exe
  pwsh5: rustc "04b_cp_input2.rs" --crate-name run_program; .\run_program.exe 

// using cmd from containing folder:
  cmd: rustc "04b_cp_input2.rs" --crate-name run_program && .\run_program 
  cmd: rustc "04b_cp_input2.rs" --crate-name run_program && .\run_program < input.txt

*/


/*
testcase:
5
5
1 2 3 2 1
7
5 4 1 1 1 1 3
6
1 2 3 4 5 6
6
4 1 6 3 2 6
7
1 3 2 7 2 3 1

8
12
0
10
18
*/
