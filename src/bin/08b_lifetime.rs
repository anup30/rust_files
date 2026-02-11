
// Lifetime annotations

struct User<'a> { // 'a is a lifetime parameter
    name: &'a str,
}

fn main(){
    let s1: String = String::from("short");
    let result: &str;
    {
        //let s2: &str = "longer string"; // it's: &'static str // allows borrowing for the entire program duration
        let s2 = String::from("longer string");
        result = longest(s1.as_str(), s2.as_str()); 
        // ^ if result used in outer scope, error:`s2` does not live long enough, borrowed value does not live long enough
        // String::as_str() -> &str
        println!("The longest string is: {}", result);
    }    
    //println!("The longest string is: {}", result);

    // 'static lifetime: lives for the entire program
    let _s3: &'static str = "static hello";

    let name: String = String::from("Anup");
    let user: User<'_> = User { name: &name };
    // Rust guarantees name lives at least as long as user.
    println!("user.name: {}", user.name);    

    let s = String::from("hello");
    let r = &s;   // borrow
    println!("{}", r); // ok, r is valid here
    dangling_reference_example();
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { // 'a is a lifetime parameter
    if x.len() > y.len() { x } else { y }
    /*
    lifetimes are entirely determined at compile time, Rust never decides lifetimes at runtime.
    The borrow checker compares lifetimes at compile time to ensure safety.
    here, at compile time, Rust does not know whether x or y will be returned.
    So, Rust requires that both x and y live for the entire duration of the function call.
    */
}

fn dangling_reference_example(){
    let r: &String;
    {
        let s: String = String::from("hello");
        r = &s; // r borrows s, reference
        // or, r = s.clone(); // no borrowing, clone() is ownership transfer via copy, r owns its own heap allocation        
        // or, r=s; // move, no clone
        println!("s in inner scope: {}", s); // ok now, not allowed in case of mutable borrow, let r = &mut s;
        println!("r in inner scope: {}", r); // s dies here
    }
    //println!("s in outer scope: {}", s); // error: cannot find value `s` in this scope
    //println!("r in outer scope: {}", r); // dangling reference, Rust rejects this at compile time using lifetimes. // error:  - `s` dropped here while still borrowed
}

/*
Lifetimes vs Ownership distinction:
Concept	       Purpose
Ownership	Who owns the data
Borrowing	Temporary access
Lifetimes	How long borrows are valid

mental model:
- Ownership is about who is responsible for freeing the data.
- Borrowing is about who can access the data.
- Lifetimes are about how long the borrow is valid.
*/


/*
as_str() → You may look at my data while I exist
clone() → Here’s your own copy, do what you want
to_string() → convert to String

String   → owns text
&str     → borrows text

let s: &str = "hello"; // &str is read-only by design // type is &'static str
*/

// powered by chatgpt 5.2