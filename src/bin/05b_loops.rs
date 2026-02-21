// for loop

fn main() {
    println!("rust loop example with vector");
    let mut v: Vec<i32> = Vec::new();
    // Insert 10 integers using a loop: each value equals twice its 0-based index
    for i in 0..10 {
        v.push(i as i32 * 2);
    }

    // Print the vector contents using another loop
    println!("v contents:");    
    for i in 0..10 { // loop 0 to 9
        print!("{}, ", v[i]);
    }
    println!();
    for i in 0..=9 { // loop 0 to 9
        print!("{}, ", v[i]);
    }
    println!();

    // Initialize with capacity 100, size 0
    let mut v2: Vec<i32> = Vec::with_capacity(100); 
    for i in 0..10 {
        if i == 3 {
            continue; // skip i = 3
        }
        if i == 7 {
            break; // stop the loop when i reaches 7
        }
        v2.push(i);
    }

    // Print the vector contents using another loop
    println!("v2 contents:");
    for num in &v2 {
        print!("{} ", num);
    }
    println!();
    v2.clear(); 

}

/*
// labeled loops for nested cases:
'outer: for i in 0..5 {
    for j in 0..5 {
        if i * j > 10 {
            break 'outer;   // exits the outer loop
        }
    }
}

// break can return a value from a loop expression
let result = loop {
    // ... some logic
    if condition {
        break 42;   // returns 42 from the loop
    }
};
*/