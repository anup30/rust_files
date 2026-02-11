// ownership and borrowing system
// These concepts are Rust's most distinctive feature and the key to writing safe, efficient code without a garbage collector.

/*
Ownership is Rust's central memory management mechanism. It enforces a set of rules at compile time that guarantee memory safety without runtime overhead.
The Three Ownership Rules
Every value in Rust has a single "owner" variable. The rules are:
1. Each value has exactly one owner
2. When the owner goes out of scope, the value is dropped (memory freed)
3. Only one owner exists at a time (ownership can be transferred)

Ownership transfer is done by assignment, not by copying.

*/


fn main(){    
    let s1 = String::from("hello"); // s1 owns the String        
    let _s2 = s1; // Ownership moves to s2, s1 is no longer valid

    let v1 = vec![1, 2, 3];  // v1 owns the vector
    let _v2 = v1;             // ownership moves to v2, v1 is now invalid - cannot be used

    //Copy Types
    // Simple types with known size (integers, booleans, floats) implement the Copy trait and are copied instead of moved
    let x = 5;
    let y = x;  // x is still valid - it was copied, not moved
    println!("x = {}, y = {}", x, y);

    // Borrowing
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // &s1 is a reference to s1
    println!("Length of '{}' is {}", s1, len);

    // Multiple immutable references are allowed simultaneously
    let v = vec![1, 2, 3];
    let r1 = &v;  // immutable borrow
    let r2 = &v;  // another immutable borrow - OK!
    println!("{:?}, {:?}", r1, r2);

    // Mutable reference
    let mut v = vec![1, 2, 3];
    let r1 = &mut v;  // mutable borrow, v can't be used while r1 is in scope
    // let r2 = &v;   // error: cannot have immutable borrow while mutable borrow exists
    r1.push(4);
    // println!("v = {:?}", v); // cannot borrow `v` as immutable because it is also borrowed as mutable, immutable borrow occurs here
    println!("r1 = {:?}", r1);
    println!("v = {:?}", v); // ok here, r1 scoped out previous line.

    /*
    The Borrowing Rules -----
    The Rust type system enforces two critical invariants:
    1, A reference cannot outlive its referent
    2, A mutable reference cannot be aliased
    */
    
    // Lifetimes: Ensuring References Stay Valid
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// powered by step 3.5