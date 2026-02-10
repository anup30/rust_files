// rust struct
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // This is a method that borrows self immutably
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // This method requires a mutable reference to self
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    // Associated function (doesn't take self)
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let mut rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    rect1.set_width(40);
    println!("The new width is {}", rect1.width);

    let square = Rectangle::square(20); // Calling an associated function
    println!("The area of the square is {} square pixels.", square.area());
}
