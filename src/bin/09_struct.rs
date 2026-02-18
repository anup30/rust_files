// rust struct
// https://doc.rust-lang.org/book/ch05-00-structs.html
// https://doc.rust-lang.org/book/ch18-00-oop.html

struct Rectangle {
    width: f64,
    height: f64,
}

// impl (implementation) block, to add functions (methods) to a struct
impl Rectangle {
    // Associated function (constructor-like)
    fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }
    
    // Method (takes &self)
    fn area(&self) -> f64 {
        self.width * self.height
    }
    
    // Method that modifies self
    fn scale(&mut self, factor: f64) {
        self.width *= factor;
        self.height *= factor;
    }
}

fn main() {
    let mut rect = Rectangle::new(10 as f64, 20 as f64);
    println!("Area: {}", rect.area());  // Output: 200
    
    rect.scale(1.5);
    println!("Scaled area: {}", rect.area());  // Output: 450
}
