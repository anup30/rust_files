// BTreeMap is a sorted map implementation in Rust's standard library (std::collections::BTreeMap) that stores key-value pairs in a self-balancing B-Tree structure.
// O(log n) complexity for insert, delete, and lookup operations, similar to c++ std::map
// ideal when you need ordered iteration, range queries, or predictable traversal, compared to HashMap.
// https://doc.rust-lang.org/std/collections/struct.BTreeMap.html

// BTreeMap methods: insert, get, contains_key, remove, ...
use std::collections::BTreeMap;

fn main() {
    let mut map = BTreeMap::new(); // empty map
    map.insert(2, "two");
    map.insert(1, "one");
    println!("{:?}", map); // {1: "one", 2: "two"}  // "{:#?}"

    //from iterators
    let pairs = vec![(3, "three"), (1, "one"), (2, "two")];
    let _map2: BTreeMap<_, _> = pairs.into_iter().collect();

    // default
    let mut map3: BTreeMap<i32, String> = Default::default(); // or, BTreeMap::new();

    // insert, returns the old value if the key existed, otherwise None.
    let old = map3.insert(1, "one".to_string()); // "one" is &str, "one".to_string() is String
    println!("old1: {:?}", old); // None
    let old = map3.insert(1, String::from("ek"));
    println!("old2: {:?}", old); // Some("one")
    let old = map3.insert(1, "uno".parse().unwrap());
    println!("old3: {:?}", old); // Some("ek")

    // access
    let find: i32 = 2;
    let value = map3.get(&find);
    // ^ Option<&V>, .get(&key): Does not panic. It returns Some(&value) if found, or None if the key is missing.
    // indexing can crash, let value = &map3[&2]; and 2 wasn't there, program would crash
    if let Some(v) = value {
        println!("in map3 for key: {} value: {}", find, v);
    } else {
        println!("not found key: {}", find);
    }

    // removal
    let _removed = map3.remove(&1); // Option<V>
    let _exists: bool = map3.contains_key(&1);
    let _len = map3.len();
    let _empty = map3.is_empty();

    //entry API allows efficient insertion or update without repeated lookups

    map3.entry(2).or_insert("two".to_string()); // Insert if absent, or update if present
    map3.entry(1).or_insert_with(|| {
        // Insert with computed value
        // expensive computation
        "one".to_string()
    });
    map3.entry(1).and_modify(|v| *v = "ONE".to_string()); // Modify existing entry
    println!("map3: {:?}", map3);

    // range Queries
    let mut map4: BTreeMap<i32, &str> = BTreeMap::new();
    map4.insert(1, "a"); // &str vs String -----
    map4.insert(3, "c");
    map4.insert(5, "e");
    map4.insert(7, "g");

    println!("map4: {:?}", map4);
    println!("map4: ranged query:");
    // Iterate over keys from 2 (inclusive) to 6 (exclusive)
    for (k, v) in map4.range(2..6) {
        println!("{}: {}", k, v); // prints 3:c and 5:e
    }
    println!();
    // Forward iteration
    for (k, v) in map4.iter() {
        // ^ or, for (k, v) in &map4{} // map.iter().rev() for backward -----
        println!("{}: {}", k, v);
    }

    let _keys: Vec<_> = map4.keys().collect(); // Get all keys (sorted)
    let _values: Vec<_> = map4.values().collect(); // Get all values (in key order)

    // Immutable references
    let first_key = map4.first_key_value(); // Option<(&K, &V)>
    if let Some((key, value)) = first_key {
        println!("The first entry is {} with value {}", key, value); // The first entry is 1 with value a
    }
    let last_key = map4.last_key_value();
    println!("Last: {:?}", last_key); // Last: Some((7, "g"))
}

// rustc "06b_BTreeMap.rs" --crate-name run_program && .\run_program

/*
// Counting Word Frequencies ------------------------ example app 1
let text = "hello world hello rust";
let mut counts = BTreeMap::new();

for word in text.split_whitespace() {
    *counts.entry(word).or_insert(0) += 1;
}

// Words will be printed in sorted order
for (word, count) in &counts {
    println!("{}: {}", word, count);
}
*/

/*
// Time-Series Data with Range Queries ------------------------ example app 2
use std::collections::BTreeMap;
use std::time::*;

let mut readings: BTreeMap<SystemTime, f64> = BTreeMap::new();
readings.insert(SystemTime::now(), 23.5);
// ... insert more readings

// Find all readings from the last hour
let one_hour_ago = SystemTime::now() - Duration::from_secs(3600);
for (time, temp) in readings.range(one_hour_ago..) {
    println!("{:?}: {}", time, temp);
}
*/

// powered by step 3.5, gemini 3
