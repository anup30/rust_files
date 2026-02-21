use std::io::{ Read, stdin}; 

fn main() {
    let mut handle = stdin().lock();
    let mut buf = String::new();
    handle.read_to_string(&mut buf).unwrap();
    let mut tokens=buf.split_whitespace().map(|x| x.parse::<u32>().unwrap());
    let t=tokens.next().unwrap();
    for _ in 0..t{
        let n=tokens.next().unwrap();
        let mut found=false;
        for _ in 0..n{
            if tokens.next().unwrap() == 67 {
                found=true;
            }
        }
        println!("{}",{
            if found {"Yes"}else{"No"}
        });
 
    }
    
}

/*
2
5
1 7 6 7 67
5
1 3 5 7 8
*/


// rustc "test.rs" --crate-name run_program && .\run_program  // run from src/bin/
// cargo run --bin test  // run from project root  // can run from project root or src/bin/