// tle
// #![allow(unused)]
struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::collections::HashMap;
        let len:usize = s.len();
        if len == 0 {return 0;}
        let mut mp: HashMap<char, usize> = HashMap::new();   
        let mut res:usize = 1;
        let mut l:usize = 0;
        let mut r:usize = 0;
        while r<len{
            for i in l..len {
                let c:char = s.chars().nth(i).unwrap();
                if mp.contains_key(&c){                    
                    l = mp.get(&c).copied().unwrap()+1;                   
                    mp.clear();
                    break;
                }else{
                    mp.insert(c, i);                    
                    r= i+1;
                    res = std::cmp::max(res, r - l); // 
                }            
            }
        }
        return res as i32;
    }
}

fn main() {
    let v = vec!["abcabcbb", "bbbbb", "pwwkew" ]; // Vec<&str>  
    for st in v{        
        println!("{}: {}", st,Solution::length_of_longest_substring(st.to_string()));
    }
}

// rustc "src/bin/test3.rs" --crate-name run_program && .\run_program