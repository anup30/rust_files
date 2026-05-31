// leetcode: 8. String to Integer (atoi) // https://leetcode.com/problems/string-to-integer-atoi/description/
// accepted, april 11, 2026, beats 100%

#![allow(unused)]

struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut started:bool = false;
        let mut neg:bool = false;
        let mut v:Vec<char> = Vec::new();
        let mut res:i64 = 0;
        let mn:i64 = i32::MIN as i64; // -2^31 = -2147483648
        let mx:i64 = i32::MAX as i64; // 2^31 - 1 = 2147483647
        for (_, c) in s.chars().enumerate() {
            if c == ' ' && !started {continue;}
            if !started && c == '-' {neg = true; started = true; continue;}
            if !started && c == '+' {neg = false; started = true; continue;}
            if !c.is_ascii_digit() {break;} // '0' to '9'
            started = true;
            v.push(c);
        }
        let len = v.len();
        for i in 0..len {
            res = res * 10 + (v[i] as i64 - '0' as i64); // '0' == 48
            if neg && res > mx+1 {return i32::MIN;}
            if !neg && res > mx {return i32::MAX;}
        }
        if neg {res = -res;}
        return res as i32;
    }
}

fn main() {
    let v = vec!["42", "  -042", "1337c0d3", "0-1", "words and 987", "-91283472332"]; // Vec<&str>
    for num in v{        
        println!("{}: {}", num, Solution::my_atoi(num.to_string()));
    }
    //println!("{}: {}", "-91283472332", Solution::my_atoi("-91283472332".to_string()));
}
// -91283472332: -1089159116

/*
42: 42
  -042: -42
1337c0d3: 1337
0-1: 0
words and 987: 0
-91283472332: -2147483648
*/

// rustc "src/bin/cp1.rs" --crate-name run_program && .\run_program
