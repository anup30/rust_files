// idea from: solution: https://codeforces.com/contest/2217/submission/370167287 (by trycatchcry)
// problem: round 1091 - C. Grid Covering, https://codeforces.com/contest/2217/problem/C
use std::io::{self, Read};
use std::str::FromStr;

fn next<T: FromStr>(it: &mut std::str::SplitWhitespace) -> T
where T::Err: std::fmt::Debug, {
    it.next().unwrap().parse().unwrap()
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}
 
fn solve() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace(); 

    let t: usize = next(&mut it); // it.next().unwrap().parse().unwrap();
    let mut out = String::new();
 
    for _ in 0..t {
        let n: i64 = next(&mut it);
        let m: i64 = next(&mut it);
        let a: i64 = next(&mut it);
        let b: i64 = next(&mut it);
 
        if gcd(n, a) == 1 && gcd(m, b) == 1 && gcd(n, m) <= 2 {
            out.push_str("YES\n");
        } else {
            out.push_str("NO\n");
        }
    }
 
    print!("{}", out);
}
 
fn main() {
    solve();
}


/*
// if input given from terminal, press enter + ctrl+z, in terminal to signal eof

// from project root:
  pwsh7 :  rustc "src/bin/04d_cp_input4.rs" --crate-name run_program && .\run_program
  pwsh7:  cat src/bin/input.txt | cargo run --bin 04d_cp_input4
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
10
1 1 1 1
2 2 1 1
4 2 2 1
6 9 6 7
67 42 42 67
3411 4134 32 23
90234 143124 232 323
69387963 98793214 9791 4324786
985865 578977 899368 447605
1000000000 1000000000 1000000000 1000000000

YES
YES
NO
NO
YES
NO
NO
NO
YES
NO
*/
