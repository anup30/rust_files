// 

#![allow(unused)]

struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut num = x as i64;
        let mut result: i64 = 0;
        
        while num != 0 {
            result = result * 10 + num % 10;
            num /= 10;
        }
        
        if result > i32::MAX as i64 || result < i32::MIN as i64 {
            0
        } else {
            result as i32
        }
    }
}

fn main() {
    let v = vec![123, -123, 120, 0, 1534236469]; // Vec<i32>
    for num in v{        
        println!("{}: {}", num, Solution::reverse(num));
    }
}

/*
123: 321
-123: -321
120: 21
0: 0
1534236469: 0
*/

// rustc "src/bin/cp2.rs" --crate-name run_program && .\run_program
