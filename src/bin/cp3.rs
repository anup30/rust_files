#![allow(unused)]
struct Solution;

impl Solution {
    pub fn reverse(x: i64) -> i32 {
        if x > i32::MAX as i64 || x < i32::MIN as i64 { return 0; }
        let mut x2 = x;
        let mut cur:i64 = 0;
        let mn:i64 = i32::MIN as i64; // -2^31 = -2147483648
        let mx:i64 = i32::MAX as i64; // 2^31 - 1 = 2147483647
        if x == i32::MIN as i64 {return 0;}
        let mut neg:bool;
        if x2<0 {neg = true; x2 *= -1;}
        else {neg = false;}
        let mut v:Vec<i32> = Vec::new();
        let mut b:bool = true;
        while b {
            let tmp = x2%10;
            v.push(tmp as i32);
            x2 /= 10;
            if x2 == 0 {b = false;}
        }
        let len = v.len();
        for i in 0..len {
            cur += v[i] as i64 * 10_i64.pow((len - i - 1) as u32);
        }
        if cur > mx || cur < mn {return 0;}
        if neg {cur *= -1;}
        return cur as i32;
    }
}

fn main() {
    // let v = vec![123, -123, 120, 0, 1534236469]; // Vec<i32>
    // for num in v{        
    //     println!("{}: {}", num, Solution::reverse(num));
    // }
    println!(" {}", Solution::reverse(1534236469));
}

/*
123: 321
-123: -321
120: 21
0: 0
1534236469: 0
*/

// rustc "src/bin/cp3.rs" --crate-name run_program && .\run_program
