// rust enum
// enum vs struct: enum represents a choice between different variants('or' relation), struct represents a collection of fields('and' relation),

#[allow(dead_code)]

#[derive(Debug)]
enum Status {Active, Inactive, Pending}

// enum with data
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
}
// impl block for Shape (Methods on Enums)
impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(r) => std::f64::consts::PI * r * r,
            Shape::Rectangle(w, h) => w * h,
        }
    }
}

fn main() {    
    println!("rust enum tutorial");

     
    let condition = Status::Active;
    println!("condition = {:?}", condition); // condition = Active
    
    let msg = Message::Write(String::from("Hello, world!"));
    println!("msg = {:?}", msg); // msg = Write("Hello, world!")
    let msg1 = Message::Quit;
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::Write("hello".to_string());
    let msg4 = Message::ChangeColor(255, 0, 0);
    // println!("msg1 = {:?}, msg2 = {:?}, msg3 = {:?}, msg4 = {:?}", msg1, msg2, msg3, msg4);
    // output: msg1 = Quit, msg2 = Move { x: 10, y: 20 }, msg3 = Write("hello"), msg4 = ChangeColor(255, 0, 0)
    handle_message(msg1);
    handle_message(msg2);
    handle_message(msg3);
    handle_message(msg4);

    let s1 = Shape::Circle(2.5);
    let s2 = Shape::Rectangle(5.0, 1.5);
    println!("s1 area = {:.2}", s1.area());
    println!("s2 area = {:.2}", s2.area());

    // Option<T> — Represents optional values
    let some_number: Option<i32> = Some(5);
    let no_number: Option<i32> = None;
    match some_number {
        Some(n) => println!("Got {}", n),
        None => println!("No value"),
    }
    // Or use if let
    if let Some(n) = no_number {
        println!("Got {}", n);
    }

    // Result<T, E> — Represents success or failure
    let result = divide(10, 2);
    match result {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
    match divide(10, 0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}

fn handle_message(msg: Message) {
    // pattern matching with match
    match msg {
        Message::Quit => println!("Quitting"),
        Message::Move { x, y } => println!("Moving to ({}, {})", x, y),
        Message::Write(text) => println!("Text: {}", text),
        Message::ChangeColor(r, g, b) => println!("Color: ({}, {}, {})", r, g, b),
    }
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}

// cargo run --bin 09a_enum