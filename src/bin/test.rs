//

fn main() {
    let mut v = vec![20, 21, 22];
    {
        let a = &mut v;  // temporary mutable borrow
        a.push(23);
        println!("The answer is {}", a[3]);
    } 
    v.push(24);
    println!("v = {:?}", v);
}
 
