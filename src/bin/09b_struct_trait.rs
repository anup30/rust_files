// struct trait

#[derive(Debug, Clone)] // Copy trait ?

#[allow(dead_code)] // Fields are used via debug formatting // to fix: fields `width` and `height` are never read

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };
    // println!("rect, width: {} height: {}", rect.width, rect.height);    
    println!("rect is {:?}", rect);  // debug format    
    println!("rect is {:#?}", rect); // pretty multi-line format

    let mut rect2 = rect.clone(); //ownership not moved
    rect2.width *= 2;
    println!("rect2 is {:?}", rect2);
    println!("rect is {:?}", rect);
}

// cargo run --bin 09bstruct_trait

/*
Most Rust projects use these standard library traits constantly:

Trait        | What it does                 | Example Use Case
-------------|------------------------------|------------------------------------------
Debug        | Allows the {:?} formatter.   | Printing values for troubleshooting.
Clone        | Adds a .clone() method.      | Creating a deep copy of the data.
Copy         | Makes "assignment" a copy.   | Small types (like coordinates) that shouldn't "move."
PartialEq    | Enables == and !=.           | Checking if two structs have the same values.
Default      | Enables Type::default().     | Providing "zeroed-out" or starting values.
PartialOrd   | Enables <, >, <=, >=.        | Sorting a list of your structs.

*/

/*
trait: is a way to tell the compiler about functionality a particular type has and can share with other types.
Think of a trait as a contract or a standardized interface. If a type "signs" the contract (implements the trait), it guarantees it can perform specific actions.

derive: it's a procedural macro that automatically writes the boilerplate code needed to implement certain traits for your struct or enum.
Without it, you’d have to manually write out the logic for things as simple as printing a struct or comparing two values.

A struct can only derive a trait if every single field inside that struct also implements that trait.
eg. If your struct has a String field, you cannot derive Copy because String does not implement Copy.

If you want == to only care about an id field and ignore a timestamp field, you have to skip the derive and manually implement the trait.

Clone is explicit. You must call .clone(). It can be "expensive" (like copying a whole book).
Copy is implicit. It happens automatically on assignment (a = b). It must be "cheap".

*/

// by Gemini 3
