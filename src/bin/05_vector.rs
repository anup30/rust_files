// vector, resizable, heap-allocated array of the same type. O(1) indexing via [] (with bounds checking)
// https://doc.rust-lang.org/std/vec/struct.Vec.html
// https://doc.rust-lang.org/book/ch08-01-vectors.html
// https://doc.rust-lang.org/rust-by-example/std/vec.html

fn main() {
    let mut v1: Vec<i32> = Vec::new(); // empty vector
    let v2 = vec![1, 2, 3]; // Vec<i32>
    let v3 = vec!["a", "b", "c"]; // Vec<&str>
    let v4 = vec![0; 5]; // [0, 0, 0, 0, 0]
    let _v: Vec<i32> = (0..5).collect(); // [0, 1, 2, 3, 4] // From Iterators
    let v5: Vec<i32> = Vec::with_capacity(10); // allocates space for 10 elements, but len=0
    println!(
        "{} {}",
        v1.is_empty(),
        v2.len() + v3.len() + v4.len() + v5.len()
    );
    v1.push(1);
    v1[0] *= 2;
    let v6: Vec<i32> = (0..5).collect(); // From Iterators
    println!("v6= {:?}", v6); // v6= [0, 1, 2, 3, 4]

    // vector methods: .push() .insert() .append() .extend() .pop() .remove() .truncate() .clear()
    // .get() .first() .last() .contains() .capacity() .reserve()

    // Vectors implement the IntoIterator trait, allowing several ways to iterate.
    // for loop (consumes the vector)
    let v7 = vec![5, 6, 7];
    for val in v7 {
        print!("{}, ", val);
    }
    println!();
    // v7 is moved and cannot be used after this.
    // println!("v7= {:?}", v7);
    // ^ error[E0382]: borrow of moved value: `v7` / Value used after being moved
    // ^ https://doc.rust-lang.org/error_codes/E0382.html

    // Borrowed iteration, iter() → &T , iter_mut() → &mut T
    let mut v8 = vec![10, 11, 12];
    for val in v8.iter() {
        print!("{} ", val); // immutable
    }
    println!();
    for val in v8.iter_mut() {
        *val += 1; // mutable
    }
    println!("v8= {:?}", v8);

    // Sorting
    let mut v9 = vec![4, 1, 2];
    v9.sort_by(|a, b| b.cmp(a)); // descending
    v9.sort(); // ascending
    println!("v9= {:?}", v9);

    // Binary Search: Requires the vector to be sorted.
    match v9.binary_search(&4) {
        Ok(idx) => println!("Found at index {}", idx),
        Err(idx) => println!("Not found, would insert at {}", idx),
    }
    match v9.binary_search(&3) {
        Ok(idx) => println!("Found at index {}", idx),
        Err(idx) => println!("Not found, would insert at {}", idx),
    }

    let vec = vec![1, 2, 3, 4, 5];
    // Iterator methods
    println!("iter methods:");
    let _sum: i32 = vec.iter().sum();
    let _doubled: Vec<i32> = vec.iter().map(|x| x * 2).collect();
    let _evens: Vec<&i32> = vec.iter().filter(|x| *x % 2 == 0).collect();
    // With index
    for (index, value) in vec.iter().enumerate() {
        println!("Index {}: {}", index, value);
    }

    let mut matrix: Vec<Vec<i32>> = Vec::new();
    matrix.push(vec![1, 2, 3]);
    matrix.push(vec![4, 5, 6]);
    // Access: matrix[0][1] -> 2
    println!("matrix= {:?}", matrix);
}

// rustc "05_vector.rs" --crate-name run_program && .\run_program

/*
// example programs:

// 1
fn main() {
    // Create a vector of squares of even numbers up to 10
    let squares: Vec<i32> = (0..=10)
        .filter(|x| x % 2 == 0)
        .map(|x| x * x)
        .collect();

    println!("Squares: {:?}", squares); // [0, 4, 16, 36, 64, 100]

    // Find the maximum square
    if let Some(max) = squares.iter().max() {
        println!("Maximum square: {}", max);
    }

    // Reverse the vector in place
    let mut rev = squares.clone();
    rev.reverse();
    println!("Reversed: {:?}", rev); // Reversed: [100, 64, 36, 16, 4, 0]
    println!("Squares: {:?}", squares); // Squares: [0, 4, 16, 36, 64, 100]
}

//2 Creating Vectors
fn main() {
    // Method 1: Empty vector (needs type annotation if not used immediately)
    let v1: Vec<i32> = Vec::new();

    // Method 2: Using vec![] macro with initial values
    let v2 = vec![1, 2, 3, 4, 5];

    // Method 3: Empty with specific capacity (optimization hint)
    let mut v3 = Vec::with_capacity(10);

    // Method 4: Create with same initial value repeated
    let v4 = vec![0; 5];  // [0, 0, 0, 0, 0]

    println!("{:?}", v2);  // [1, 2, 3, 4, 5]
}

//3 Adding and Removing Elements
fn main() {
    let mut vec = Vec::new();

    // Push elements to the end
    vec.push(1);
    vec.push(2);
    vec.push(3);
    println!("{:?}", vec);  // [1, 2, 3]

    // Pop from the end (returns Option<T>)
    let last = vec.pop();   // Some(3)
    println!("Popped: {:?}, remaining: {:?}", last, vec);

    // Insert at specific index (O(n) - shifts elements)
    vec.insert(0, 100);   // [100, 1, 2]

    // Remove from specific index (O(n) - shifts elements)
    let removed = vec.remove(1);  // removes '1', returns it

    // Extend with iterator
    vec.extend([4, 5, 6]);

    // Append another vector (empties the source)
    let mut other = vec![7, 8];
    vec.append(&mut other);  // vec now [100, 2, 4, 5, 6, 7, 8]
}

//4 Accessing Elements
fn main() {
    let vec = vec![10, 20, 30, 40, 50];

    // Indexing - panics if out of bounds
    let first = vec[0];  // 10
    let second = vec[1]; // 20

    // Using get() - returns Option<&T> (safe)
    match vec.get(100) {
        Some(val) => println!("Found: {}", val),
        None => println!("Index out of bounds"),
    }

    // First and last elements
    if let Some(first) = vec.first() {
        println!("First: {}", first);
    }
    if let Some(last) = vec.last() {
        println!("Last: {}", last);
    }

    // Mutable access
    let mut vec2 = vec![1, 2, 3];
    if let Some(first) = vec2.first_mut() {
        *first = 100;  // [100, 2, 3]
    }
}

//5 Iteration
fn main() {
    let vec = vec![1, 2, 3, 4, 5];

    // Immutable iteration by reference
    for item in &vec {
        println!("{}", item);
    }

    // Mutable iteration
    let mut vec2 = vec![1, 2, 3];
    for item in &mut vec2 {
        *item *= 2;  // Double each element
    }
    println!("{:?}", vec2);  // [2, 4, 6]

    // Consuming iteration (takes ownership)
    let vec3 = vec![1, 2, 3];
    for item in vec3 {
        println!("{}", item);  // vec3 is moved here, can't use after
    }

    // Iterator methods
    let sum: i32 = vec.iter().sum();
    let doubled: Vec<i32> = vec.iter().map(|x| x * 2).collect();
    let evens: Vec<&i32> = vec.iter().filter(|x| *x % 2 == 0).collect();

    // With index
    for (index, value) in vec.iter().enumerate() {
        println!("Index {}: {}", index, value);
    }
}

// 6 Slicing and Range Operations
fn main() {
    let vec = vec![10, 20, 30, 40, 50, 60];

    // Slicing (returns &[T])
    let slice = &vec[1..4];  // [20, 30, 40]
    let first_three = &vec[..3];   // [10, 20, 30]
    let last_three = &vec[3..];    // [40, 50, 60]
    let all = &vec[..];            // [10, 20, 30, 40, 50, 60]

    // Windows (sliding window)
    for window in vec.windows(3) {
        println!("{:?}", window);  // [10,20,30], [20,30,40], etc.
    }

    // Chunks (non-overlapping)
    for chunk in vec.chunks(2) {
        println!("{:?}", chunk);   // [10,20], [30,40], [50,60]
    }

    // Rchunks (chunks from the end)
    for chunk in vec.rchunks(2) {
        println!("{:?}", chunk);   // [50,60], [30,40], [10,20]
    }
}

// 7 Searching and Sorting
fn main() {
    let mut vec = vec![5, 2, 8, 1, 9, 3];

    // Sorting
    vec.sort();  // Ascending: [1, 2, 3, 5, 8, 9]
    vec.sort_by(|a, b| b.cmp(a));  // Descending: [9, 8, 5, 3, 2, 1]

    // Unstable sort (faster, doesn't preserve equal element order)
    vec.sort_unstable();

    // Binary search (vec must be sorted)
    if let Ok(index) = vec.binary_search(&5) {
        println!("Found at index {}", index);
    }

    // Contains check
    if vec.contains(&5) {
        println!("Contains 5");
    }

    // Find with predicate
    let found = vec.iter().find(|&&x| x > 5);  // Some(&8)

    // Position of element
    if let Some(pos) = vec.iter().position(|&x| x == 5) {
        println!("5 is at index {}", pos);
    }

    // Retain (filter in-place)
    vec.retain(|&x| x % 2 == 0);  // Keep only even numbers: [8, 2]
}

// 8 Memory Management
fn main() {
    let mut vec = Vec::with_capacity(100);
    println!("Capacity: {}, Length: {}", vec.capacity(), vec.len());
    // Capacity: 100, Length: 0

    vec.extend(0..50);
    println!("After adding 50 items - Capacity: {}, Length: {}",
             vec.capacity(), vec.len());

    // Shrink to fit actual length
    vec.shrink_to_fit();

    // Reserve additional capacity
    vec.reserve(100);  // Ensure space for 100 more elements

    // Truncate (remove elements from end)
    vec.truncate(10);  // Keep only first 10 elements

    // Clear (remove all, keep capacity)
    vec.clear();
    println!("After clear - Capacity: {}, Length: {}",
             vec.capacity(), vec.len());

    // Into boxed slice (converts to Box<[T]>, can't grow)
    let boxed: Box<[i32]> = vec.into_boxed_slice();
}

// 9 Ownership and References
fn main() {
    let vec = vec![String::from("hello"), String::from("world")];

    // By reference - vec still usable
    print_length(&vec);

    // Cloning - expensive but safe
    let vec2 = vec.clone();

    // Moving - vec is no longer usable
    consume_vec(vec);
    // println!("{:?}", vec);  // ERROR: vec was moved

    // Moving with reuse pattern
    let vec3 = vec![1, 2, 3];
    let vec3 = process_and_return(vec3);  // Shadowing pattern
}

fn print_length(v: &Vec<String>) {
    println!("Length: {}", v.len());
}

fn consume_vec(v: Vec<String>) {
    for s in v {
        println!("{}", s);
    }
}

fn process_and_return(mut v: Vec<i32>) -> Vec<i32> {
    v.push(4);
    v
}

*/

/*
//Borrowing Rules - Rust’s ownership system applies to vectors:
let mut v = vec![1, 2, 3];
let first = &v[0]; // immutable borrow
// v.push(4); // error: cannot borrow `v` as mutable because it is also borrowed as immutable
*/

// powered by Step 3.5 , kimi K2.5
