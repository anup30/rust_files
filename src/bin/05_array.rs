// rust array, (see also: array slice)
// Array is a collection of objects of the same type stored in contiguous memory.
// size must be known at compile time, live on the stack

fn main() {
    let mut ar: [i32; 3] = [1, 2, 3];
    // Safe Access: To avoid panics, use the .get(index) method. It returns an Option<&T>.
    match ar.get(3) {
        Some(val) => println!("Value at index 3: {}", val), // Returns Some(&element) if the index is valid.
        None => println!("Index out of bounds!"), // Returns None if the index is out of bounds.
    }
    // itearte over array
    for num in ar {
        print!("{}, ", num);
    }
    println!();

    for i in 0..ar.len() {
        print!("{} ", ar[i]); // 0 based indexed
    }
    println!();

    // using iter() - Immutable borrow (common)
    for element in ar.iter() {
        print!("{} ", element); // Output: 1 2 3 4 5
    }
    println!();

    // using iter_mut() - Mutable borrow (Allows modification)
    for element in ar.iter_mut() {
        *element *= 2;
    }
    ar[2] += 1;
    println!("ar {:?}", ar); // {:?} debug formatting, {:#?} pretty debug // prints: [2, 4, 7]

    let zeros = [0; 10];
    println!("zeros length={}", zeros.len()); // 10

    let fruits = ["apple", "banana", "cherry"]; // elements are &str

    // Using for loop
    for fruit in &fruits {
        println!("Fruit: {}", fruit);
    }
    println!();
    // With index
    for (i, fruit) in fruits.iter().enumerate() {
        println!("Index {}: {}", i, fruit);
    }
    println!();
    // Using while loop
    let mut i = 0;
    while i < fruits.len() {
        println!("Fruit[{}] = {}", i, fruits[i]);
        i += 1;
    }
    println!();
    // array methods: .len() .is_empty() .first() .last()
    // Pass by reference (doesn't consume): print_array(&numbers);
    // Pass mutable reference: modify_array(&mut numbers);

    //let mut arr = [1, 2, 3, 4, 5];
}

// rustc "04_array.rs" --crate-name run_program && .\run_program

// with deepseek 3.2 , glm 4.7
