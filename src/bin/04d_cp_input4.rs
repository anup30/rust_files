// input taking using macro_rules! - next
// idea from: https://codeforces.com/contest/2227/submission/373136651 (by silicalet)
// problem: round 1096 - E. It All Went Sideways, https://codeforces.com/contest/2227/problem/E 

#![allow(unused, non_snake_case, dead_code)]
use std::io::{BufRead, Read, Write, stdout, stdin};

// --- COMPACT I/O MACROS ---

/// Reads the very next whitespace-separated token and parses it into any inferable type
/// Usage: let n: usize = next!(); OR let s: String = next!();
macro_rules! next {
    () => {{ // when a macro has (), it means you must pass nothing inside the macro call.
        stdin().lock().bytes()
            .map(|x| x.unwrap() as char)
            .skip_while(|c| c.is_ascii_whitespace())
            .take_while(|c| !c.is_ascii_whitespace())
            .collect::<String>().parse().unwrap()
    }};
}

fn main() {
    let T: usize = next!();
    for _ in 1..=T {
        solve();
    }
}

fn solve(){
    let mut out = stdout().lock();    
    let n:usize = next!();
    let a: Vec<usize> = (0..n).map(|_| next!()).collect();

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
  pwsh7:  cat src/bin/input.txt | cargo run --bin 04d_cp_input4
  pwsh7 :  rustc "src/bin/04d_cp_input4.rs" --crate-name run_program && .\run_program
  pwsh7 :  cat src/bin/input.txt | rustc "src/bin/04d_cp_input4.rs" --crate-name run_program && .\run_program

// from containing folder:
  pwsh5: rustc -O "04d_cp_input4.rs" --crate-name run_program; Get-Content input.txt | .\run_program.exe
  pwsh5: rustc "04d_cp_input4.rs" --crate-name run_program; .\run_program.exe 

// using cmd from containing folder:
  cmd: rustc "04d_cp_input4.rs" --crate-name run_program && .\run_program 
  cmd: rustc "04d_cp_input4.rs" --crate-name run_program && .\run_program < input.txt

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
