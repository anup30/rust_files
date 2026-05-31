//

fn main() {
    let mut s = String::from("hello");
    s+=".";
    for _ in 0..10{s+="";}
    s+=".";
    println!("{}",s); // hello..
}



// rustc "src/bin/test1.rs" --crate-name run_program && .\run_program  // from project root
// cargo run --bin test1   // from project root or src/bin/