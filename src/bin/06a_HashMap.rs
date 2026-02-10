// HashMap<K, V>  average O(1) get, insert, remove (similar to c++ std::unordered_map), worst case O(N)
// Heap Allocated, Type Homogeneity, explicitly unordered
// https://doc.rust-lang.org/std/collections/struct.HashMap.html
// https://doc.rust-lang.org/rust-by-example/std/hash.html
// The HashMap takes ownership of the keys and values it stores
// alternative: BTreeMap<K, V> (keeps keys sorted O(log n)), IndexMap (keeps insertion order, O(1)), Vec<(K, V)>

// methods: insert , get, remove, clear, len, is_empty, ...

use std::collections::HashMap; // must explicitly import

fn main() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    println!("map: {:?}", scores); // map: {}

    // inserting key-value Pairs
    scores.insert(String::from("Anup"), 7); // insert returns the old value if the key already exists
    scores.insert(String::from("Saiful"), 8);
    scores.insert("Rakib".to_string(), 9);
    println!("now map: {:?}", scores); // now map: {"Anup": 7, "Rakib": 9, "Saiful": 8}
    let anup1: Option<&i32> = scores.get("Anup");
    // ^ not get(&"Anup"), The get method expects a reference to the key type, "Anup" is already a &str, for i32 key use like &1
    let anup2: Option<&i32> = scores.get("anup");
    println!("anup1: {:?}", anup1); // anup1: Some(7)
    println!("anup2: {:?}", anup2); // anup2: None
    if anup1 != None {
        println!("anup1 is not None");
    }
    if anup2 == None {
        println!("anup2 is None");
    }

    if anup1.is_some() {
        println!("We found a score of anup1 = {}", anup1.unwrap());
    }

    if anup2.is_none() {
        //println!("No score found for anup2 = {}", anup2.unwrap()); // panicked, called `Option::unwrap()` on a `None` value
        println!("No score found for anup2 = {:?}", anup2);
    }

    // This only runs if anup1 is Some. It "unwraps" the value into the variable 's'.
    if let Some(s) = scores.get("Saiful") {
        println!("Saiful, The score is: {}", s);
    } else {
        println!("Saiful, The variable was None");
    }

    match anup2 {
        Some(s) => println!("anup2 score: {s}"),
        None => println!("anup2 key not found"),
    }

    println!("{}", scores.entry(String::from("Anup")).or_insert(0)); // insert 0 if key not present

    // println!("{}", scores.entry(String::from("zeba"))); // error
    let _anup_score = scores.get("Anup").copied().unwrap_or(0);
    // ^ without insert 0, if key not there.
    // .get() returns a reference to the value inside the map (&i32). .copied() converts that &i32 into a plain i32 by copying the number.

    // update & remove
    let mut fruits: HashMap<i32, String> = HashMap::new();
    fruits.insert(1, String::from("Apple"));
    fruits.insert(2, String::from("Banana"));

    println!("Before: {:?}", fruits); // {1: "Apple", 2: "Banana"}
    fruits.insert(1, String::from("Mango")); // replace
    println!("After: {:?}", fruits); // {1: "Mango", 2: "Banana"}
    fruits.remove(&1);
    println!("After remove: {:?}", fruits); //  {2: "Banana"}

    fruits.insert(4, String::from("Peach"));
    fruits.insert(3, String::from("Orange"));

    let val: i32 = 2;
    if fruits.contains_key(&val) {
        println!("Key {} exists in fruits map!", val);
    }
    // Iterate
    println!("fruits length = {}", fruits.len());
    println!("Entries:");
    for (key, value) in &fruits {
        // for key in fruits.keys() // for value in fruits.values()
        print!("{}: {}, ", key, value); // 2: Banana, 3: Orange, 4: Peach,
    }
    println!();
    // entries
    scores.entry("Bob".parse().unwrap()).or_insert(30); // inserts because key not present
    scores.entry(String::from("Apple")).or_insert(-1); // inserts because key not present
    scores.entry(String::from("Anup")).or_insert(-1); // not inserted
    println!("{:?}", scores); // {"Apple": -1, "Rakib": 9, "Saiful": 8, "Bob": 30, "Anup": 7}
    scores.entry(String::from("Anup")).and_modify(|v| *v += 3); // update existing value if keys exists in map
    println!("{:?}", scores); // {"Apple": -1, "Rakib": 9, "Saiful": 8, "Bob": 30, "Anup": 10}

    // convert a HashMap to a Vec, sort it, and print
    let mut vec: Vec<(String, i32)> = scores.into_iter().collect(); // moved, scores is destroyed. so use scores.clone(). , explained below
    println!("Converted to Vec (unsorted):");
    for (name, score) in &vec {
        print!("{}: {},  ", name, score);
    }
    println!();
    vec.sort_by(|a, b| a.0.cmp(&b.0)); // sort by key
    println!("sorted by name:");
    println!("{:?}", vec);

    vec.sort_by(|a, b| a.1.cmp(&b.1)); // sort by value
    println!("Sorted by value (score) ascending:");
    println!("{:?}", vec);

    vec.sort_by(|a, b| b.1.cmp(&a.1)); // sort by value
    println!("Sorted by value (score) descending:");
    println!("{:?}", vec);

    // alternative
    //let mut vec2: Vec<(String, i32)> = scores.clone().into_iter().collect(); // error: Value used after being moved[E0382]
    //vec2.sort_by_key(|(name, _)| name.clone());

    // initialize From an array (Rust 1.56+)
    let map = HashMap::from([("Mercury", 0.4), ("Venus", 0.7), ("Earth", 1.0)]);
    println!("map: {:?}", map);

    // initialize From a vector using collect()
    let teams = vec![("中国队", 100), ("美国队", 80)];
    let map: HashMap<_, _> = teams.into_iter().collect();
    println!("map: {:?}", map);
}

// rustc "06.1 HashMap.rs" --crate-name run_program && .\run_program

/*
// Accessing Values  ------------------------------------- example snippet 1
// Method 1: get() - returns Option<&V>
match map.get("key1") {
    Some(value) => println!("Found: {}", value),
    None => println!("Not found"),
}

// Method 2: Indexing - panics if key doesn't exist!
let value = map["key1"];

// Method 3: contains_key()
if map.contains_key("key1") {
    println!("Key exists!");
}
*/

/*
// Updating Values ------------------------------------- example snippet 2
// Method 1: Direct insert (replaces existing)
map.insert("key1", "updated");

// Method 2: get_mut() for in-place modification
if let Some(value) = map.get_mut("key1") {
    *value = "modified";
}
*/

/*
// The Entry API (Most Efficient), avoids multiple hash lookups and is the recommended way for complex updates
let mut stats = HashMap::new();

// Insert only if key doesn't exist
stats.entry("health").or_insert(100);

// Insert with a function (lazy evaluation)
stats.entry("defence").or_insert_with(|| calculate_defence());

// Update existing or insert new
stats.entry("attack")
    .and_modify(|atk| *atk += 10)  // Update if exists
    .or_insert(50);                // Insert if not exists

// Counting example - very common pattern!
let mut letters = HashMap::new();
for ch in "hello world".chars() {
    *letters.entry(ch).or_insert(0) += 1;
}
*/

/*
// Removing Elements ------------------------------------- example snippet 3
// remove() - returns Option<V>
let removed = map.remove("key1"); // Returns Some("value1")

// remove_entry() - returns Option<(K, V)>
let removed = map.remove_entry("key1"); // Returns Some(("key1", "value1"))

// drain() - clears and returns iterator over all items
for (k, v) in map.drain().take(1) {
    println!("Removed: {} -> {}", k, v);
}

/*
// iteration ------------------------------------- example snippet 4
let map = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);

// Iterate over references (most common)
for (key, value) in &map {
    println!("{}: {}", key, value);
}

// Iterate over keys only
for key in map.keys() {
    println!("Key: {}", key);
}

// Iterate over values only
for value in map.values() {
    println!("Value: {}", value);
}

// Mutable iteration
for (key, value) in map.iter_mut() {
    *value *= 2;
}

// Consuming iteration (moves ownership)
let items: Vec<(&str, i32)> = map.into_iter().collect();
*/

// retain() - keeps only elements matching predicate
map.retain(|k, v| k.len() > 3 && *v > 10);
*/

/*
// Custom Key Types, Keys must implement Eq and Hash traits ----------------- example snippet 5
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Player {
    id: u32,
    name: String,
}

let mut scores = HashMap::new();
scores.insert(Player { id: 1, name: "Alice".to_string() }, 100);
scores.insert(Player { id: 2, name: "Bob".to_string() }, 85);

// Lookup
let alice = Player { id: 1, name: "Alice".to_string() };
if let Some(score) = scores.get(&alice) {
    println!("Alice's score: {}", score);
}
*/

/*
// letter counter app ------------------------------------- example snippet 6
use std::collections::HashMap;

fn main() {
    // Word frequency counter
    let text = "the quick brown fox jumps over the lazy dog the fox was quick";

    let mut word_count = HashMap::new();

    for word in text.split_whitespace() {
        *word_count.entry(word).or_insert(0) += 1;
    }

    println!("Word frequencies:");
    for (word, count) in &word_count {
        println!("  {}: {}", word, count);
    }

    // Find most common word
    let most_common = word_count.iter()
        .max_by_key(|&(_, count)| count)
        .map(|(word, _)| word);

    println!("Most common word: {:?}", most_common);
}
*/

/*
Rust’s HashMap uses a hashing algorithm (SipHash) designed to be resistant to HashDoS attacks.
It’s very secure but not the absolute fastest. If you find that hashing is a bottleneck in your specific application,
you can swap out the "Hasher" for a faster one like fxhash or ahash.
---
The Risk: In some languages, if an attacker knows your hashing algorithm,
they can send specifically crafted keys that all result in the same "hash collision."
This turns a fast $O(1)$ lookup into a slow $O(n)$ search, effectively crashing your server.
The Solution: SipHash uses a random "seed" every time your program starts,
making it nearly impossible for an attacker to predict collisions.
*/

/*
In Rust, "moving" is about transferring ownership of data from one variable to another.
Once data is moved, the original variable is considered empty or "invalid,"
and the compiler prevents you from using it to avoid memory bugs.
In your specific example, the culprit is the method .into_iter().

1. Why did it move?
There are three ways to iterate over a collection in Rust, and they behave very differently regarding ownership:
iter(): Borrows the data (returns references &K, &V). The map stays alive.
iter_mut(): Borrows the data mutably (returns &mut K, &mut V). The map stays alive.
into_iter(): Consumes the collection. It takes the String keys and i32 values out of the HashMap and gives them to you.
The HashMap is destroyed in the process.

2. What exactly moved?
In your code:Rust
let mut vec = scores.into_iter().collect();
The entire HashMap structure (the "container") is moved into the iterator.
The String keys (which live on the heap) are moved out of the map and into the Vec.
The i32 values are copied (since they are simple integers) into the Vec.
After this line, scores no longer exists.
It’s like a box you’ve emptied and then thrown into the recycling bin.

3. Fixing the Value used after being moved errorIf you want to turn the map into a vector but
keep using the map later, you have two main options:


Option A:
Clone the map (What you tried)You were on the right track with .clone(),
but you need to make sure you use the clone for the move,
leaving the original intact.Rust
// Clone the map first, then consume the clone
let mut vec = scores.clone().into_iter().collect();

// Now 'scores' is still valid!
println!("{:?}", scores);


Option B:
Iterate by reference (No moving)If you don't need to own the Strings in the vector
(i.e., you're okay with a vector of references), you can do this:Rust
// Use .iter() instead of .into_iter()
let mut vec: Vec<(&String, &i32)> = scores.iter().collect();
// ^ vector doesn't contain the actual data; it contains references (pointers) to the data still living inside the scores map
// sorting it is simply rearranging the "pointers" in the list.

// 'scores' is still valid because we only borrowed the data
println!("{:?}", scores);

4. Why into_iter() is usefulYou might wonder:
"Why would I ever want to destroy my map?"Performance! Moving is extremely fast in Rust.
When you use into_iter(), Rust doesn't have to copy the actual text of the Strings
("Anup", "Saiful", etc.) in memory. It just moves the "pointer" to that text from the Map to
the Vector. Cloning, on the other hand, requires allocating new memory and copying every single
character.

Summary Table:
Method,               Ownership Change,       Map Status After
scores.iter(),         Borrows,                 Still usable
scores.into_iter(),   Moves/Consumes,    Gone (Compiler Error)
scores.clone(),      Copies everything,     Both copies usable

You call...,    You get (Iterator item)...,    "Can stow in Vec<(String, i32)>?"
.into_iter(),    "(String, i32)",               Yes (Matches exactly)
.iter(),        "(&String, &i32)",          "No (It's a reference, not the object)"
.iter_mut(),    "(&mut String, &mut i32)",       No (It's a mut reference)

let mut vec: Vec<(String, i32)> = scores.iter().map(|(k, v)| (k.clone(), *v)).collect();
let mut vec: Vec<(&String, &i32)> = scores.iter().collect(); // vector of references, If you don't need to own the Strings in the vector
let mut vec: Vec<(&String, &mut i32)> = scores.iter_mut().collect(); // (be careful to use it) can change score in map
*/

// powered by step 3.5, kimi K2.5, gemini 3
