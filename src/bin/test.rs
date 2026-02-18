// Printing Structs (Debug Trait)

#[derive(Debug, Clone)] // trait `Debug`, Enables debugging formatting

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