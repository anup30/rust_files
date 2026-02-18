// ============================================================================
// RUST STRUCT TUTORIAL - Complete Guide
// ============================================================================

// ============================================================================
// PART 1: BASIC STRUCT DEFINITION
// ============================================================================

// A struct (structure) is a custom data type that lets you package together
// multiple related values into a single, meaningful group.

// --- Classic Named Struct ---
// The most common type - each field has a name and type

#![allow(dead_code, unused_variables)] // to silence unused code warnings

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// --- Tuple Struct ---
// Like a tuple, but with a name. Useful for single-purpose wrappers.
struct Point(f64, f64, f64);           // 3D coordinates
struct Color(u8, u8, u8);              // RGB values
struct Meter(f64);                     // Newtype pattern (type safety)

// --- Unit Struct ---
// No fields. Useful for type-level markers or implementing traits.
struct AlwaysEqual;
struct DatabaseConnection;             // Marker type

// ============================================================================
// PART 2: CREATING INSTANCES
// ============================================================================

fn main() {
    // --- Creating a named struct instance ---
    let user1 = User {
        email: String::from("alice@example.com"),
        username: String::from("alice123"),
        active: true,
        sign_in_count: 1,
    };
    
    // --- Field init shorthand ---
    // When variable name matches field name, you can use shorthand
    let email = String::from("bob@example.com");
    let username = String::from("bob456");
    
    let user2 = User {
        email,           // Same as: email: email
        username,        // Same as: username: username
        active: false,
        sign_in_count: 0,
    };
    
    // --- Struct update syntax ---
    // Create new instance from existing one, changing some fields
    let user3 = User {
        email: String::from("charlie@example.com"),
        ..user1          // Use remaining fields from user1
    };
    // NOTE: user1 is now invalid if we moved String fields!
    
    // --- Creating tuple struct instances ---
    let origin = Point(0.0, 0.0, 0.0);
    let black = Color(0, 0, 0);
    let distance = Meter(150.0);
    
    // --- Accessing tuple struct fields ---
    let red_value = black.0;   // Access by index (0, 1, 2...)
    
    // --- Unit struct instances ---
    let _marker = AlwaysEqual;
    
    // ============================================================================
    // PART 3: ACCESSING AND MODIFYING FIELDS
    // ============================================================================
    
    // Access with dot notation
    println!("User2 email: {}", user2.email);
    
    // Mutable binding required to modify fields
    let mut mutable_user = User {
        email: String::from("temp@example.com"),
        username: String::from("temp"),
        active: true,
        sign_in_count: 0,
    };
    
    mutable_user.sign_in_count += 1;
    mutable_user.email = String::from("new@example.com");
    
    // ============================================================================
    // PART 4: METHODS AND ASSOCIATED FUNCTIONS (impl blocks)
    // ============================================================================
    
    // Methods are defined in 'impl' (implementation) blocks
    
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    
    println!("Rectangle area: {}", rect.area());
    println!("Rectangle is square? {}", rect.is_square());
    
    // Associated functions (like constructors)
    let square = Rectangle::square(25);
    println!("Square area: {}", square.area());
    
    // String-based struct demonstration
    let person = Person::new("Alice", 30);
    person.greet();
    
    // ============================================================================
    // PART 5: OWNERSHIP AND LIFETIMES IN STRUCTS
    // ============================================================================
    
    // Structs can hold references, but need lifetime annotations
    let title = String::from("Rust Programming");
    let book = Book {
        title: &title,           // Reference to external data
        page_count: 300,
    };
    println!("Book: {} has {} pages", book.title, book.page_count);
    
    // ============================================================================
    // PART 6: GENERIC STRUCTS
    // ============================================================================
    
    let int_point = Point2D { x: 5, y: 10 };
    let float_point = Point2D { x: 5.0, y: 10.0 };
    let mixed = Point2D { x: 5, y: 10.0 };  // Different types for x and y
    
    // Generic with multiple type parameters
    let both_integer = Point2D { x: 5, y: 10 };
    let both_float = Point2D { x: 5.0, y: 10.0 };
    let integer_and_float = Point2D { x: 5, y: 10.0 };
    
    // ============================================================================
    // PART 7: TRAIT DERIVES AND CUSTOM TRAITS
    // ============================================================================
    
    let p1 = Point2D { x: 1, y: 2 };
    let p2 = Point2D { x: 4, y: 6 };
    
    // Debug trait (derived)
    println!("p1 = {:?}", p1);
    println!("p1 = {:#?}", p1);  // Pretty print
    
    // PartialEq trait (derived) - allows == and !=
    println!("p1 == p2? {}", p1 == p2);
    
    // Clone trait (derived) - explicit deep copy
    let p3 = p1.clone();
    
    // Copy trait (derived) - implicit bitwise copy (only for simple types)
    let p4 = p1;  // p1 is still valid because Point2D<i32> is Copy
    println!("p1 still valid: {:?}", p1);
    
    // ============================================================================
    // PART 8: ADVANCED PATTERNS
    // ============================================================================
    
    // Builder pattern
    let house = HouseBuilder::new()
        .bedrooms(4)
        .bathrooms(2)
        .square_feet(2500)
        .build();
    println!("Built house: {} bed, {} bath, {} sqft", 
             house.bedrooms, house.bathrooms, house.square_feet);
    
    // Type state pattern
    let new_order = Order::new();
    let paid_order = new_order.pay(100.0);
    let shipped_order = paid_order.ship();
    // shipped_order.deliver();  // Would work if we implemented it
    
    // Newtype pattern for type safety
    let m1 = Meter(100.0);
    let m2 = Meter(50.0);
    // let sum = m1 + m2;  // Error! Can't add Meter + Meter directly
    // Must access inner value: m1.0 + m2.0
    
    // ============================================================================
    // PART 9: DESTRUCTURING AND PATTERN MATCHING
    // ============================================================================
    
    let user = User {
        email: String::from("test@example.com"),
        username: String::from("testuser"),
        active: true,
        sign_in_count: 5,
    };
    
    // Destructure the struct
    let User { email, username, active, sign_in_count } = &user;
    println!("Destructured: {} {} {} {}", email, username, active, sign_in_count);
    
    // Destructure with renaming
    let User { email: user_email, username: name, .. } = &user;
    println!("Renamed: {} {}", user_email, name);
    
    // Destructure in match/if let
    if let User { active: true, sign_in_count: count, .. } = &user {
        println!("Active user with {} sign-ins", count);
    }
    
    // ============================================================================
    // PART 10: NESTED STRUCTS AND COMPOSITION
    // ============================================================================
    
    let address = Address {
        street: String::from("123 Main St"),
        city: String::from("Anytown"),
        zip: String::from("12345"),
        country: String::from("USA"),
    };
    
    let employee = Employee {
        name: String::from("John Doe"),
        id: 1001,
        address,
        department: Department::Engineering,
    };
    
    // Access nested fields
    println!("Employee {} lives in {}", employee.name, employee.address.city);
    
    // ============================================================================
    // PART 11: ZERO-SIZED TYPES AND MARKER STRUCTS
    // ============================================================================
    
    let _event = Event::<ClickEvent>::new();
    let _event2 = Event::<HoverEvent>::new();
    
    // These take up no runtime space but provide type safety
    println!("Event size: {} bytes", std::mem::size_of::<Event<ClickEvent>>());
}

// ============================================================================
// SUPPORTING DEFINITIONS FOR EXAMPLES ABOVE
// ============================================================================

// --- Rectangle with methods ---
struct Rectangle {
    width: u32,
    height: u32,
}

// impl: implementation
impl Rectangle {
    // Instance method (takes &self)
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // Method with parameters
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    // Method that takes ownership (rare)
    fn destroy(self) -> u32 {
        self.width * self.height
    }
    
    // Associated function (no self) - often constructors
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
    
    // Getter pattern
    fn is_square(&self) -> bool {
        self.width == self.height
    }
    
    // Mutable method
    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }
}

// --- Person with String handling ---
struct Person {
    name: String,
    age: u32,
}

impl Person {
    // Constructor pattern
    fn new(name: &str, age: u32) -> Person {
        Person {
            name: String::from(name),
            age,
        }
    }
    
    // Method borrowing self immutably
    fn greet(&self) {
        println!("Hello, my name is {} and I'm {} years old.", self.name, self.age);
    }
    
    // Mutable method
    fn have_birthday(&mut self) {
        self.age += 1;
        println!("Happy birthday! Now I'm {}.", self.age);
    }
    
    // Method taking ownership
    fn into_name(self) -> String {
        self.name  // Returns name, Person is consumed
    }
}

// --- Struct with references (requires lifetimes) ---
struct Book<'a> {
    title: &'a str,       // Reference needs lifetime annotation
    page_count: u32,
}

// --- Generic struct ---
// Add these derives (macros) to make the struct usable with Debug, Clone, Copy, and comparison
#[derive(Debug, Clone, Copy, PartialEq)] // ----- trait derive
struct Point2D<T, U> {
    x: T,
    y: U,
}

// Multiple trait bounds on generics
#[derive(Debug, Clone, Copy, PartialEq)]
struct Point2DConcrete<T: std::fmt::Debug + Clone + Copy + PartialEq> {
    x: T,
    y: T,
}

// --- Derived traits demonstration ---
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct Config {
    value: i32,
}

// --- Builder Pattern ---
struct House {
    bedrooms: u32,
    bathrooms: u32,
    square_feet: u32,
}

struct HouseBuilder {
    bedrooms: Option<u32>,
    bathrooms: Option<u32>,
    square_feet: Option<u32>,
}

impl HouseBuilder {
    fn new() -> Self {
        HouseBuilder {
            bedrooms: None,
            bathrooms: None,
            square_feet: None,
        }
    }
    
    fn bedrooms(mut self, n: u32) -> Self {
        self.bedrooms = Some(n);
        self
    }
    
    fn bathrooms(mut self, n: u32) -> Self {
        self.bathrooms = Some(n);
        self
    }
    
    fn square_feet(mut self, sqft: u32) -> Self {
        self.square_feet = Some(sqft);
        self
    }
    
    fn build(self) -> House {
        House {
            bedrooms: self.bedrooms.unwrap_or(3),
            bathrooms: self.bathrooms.unwrap_or(2),
            square_feet: self.square_feet.unwrap_or(2000),
        }
    }
}

// --- Type State Pattern ---
struct Order<State> {
    amount: f64,
    state: std::marker::PhantomData<State>,
}

struct New;
struct Paid;
struct Shipped;
struct Delivered;

impl Order<New> {
    fn new() -> Self {
        Order {
            amount: 0.0,
            state: std::marker::PhantomData,
        }
    }
    
    fn pay(self, amount: f64) -> Order<Paid> {
        Order {
            amount,
            state: std::marker::PhantomData,
        }
    }
}

impl Order<Paid> {
    fn ship(self) -> Order<Shipped> {
        Order {
            amount: self.amount,
            state: std::marker::PhantomData,
        }
    }
}

impl Order<Shipped> {
    fn deliver(self) -> Order<Delivered> {
        Order {
            amount: self.amount,
            state: std::marker::PhantomData,
        }
    }
}

// --- Nested structs ---
struct Address {
    street: String,
    city: String,
    zip: String,
    country: String,
}

enum Department {
    Engineering,
    Sales,
    Marketing,
    HR,
}

struct Employee {
    name: String,
    id: u32,
    address: Address,
    department: Department,
}

// --- Zero-sized types ---
struct ClickEvent;
struct HoverEvent;

struct Event<T> {
    _marker: std::marker::PhantomData<T>,
}

impl<T> Event<T> {
    fn new() -> Self {
        Event {
            _marker: std::marker::PhantomData,
        }
    }
}

// ============================================================================
// PART 12: CONST GENERICS AND ADVANCED FEATURES
// ============================================================================

// Const generics (stable since Rust 1.51)
struct Array<T, const N: usize> {
    data: [T; N],
}

impl<T: Default + Copy, const N: usize> Array<T, N> {
    fn new() -> Self {
        Array {
            data: [T::default(); N],
        }
    }
    
    fn len(&self) -> usize {
        N  // Compile-time constant!
    }
}

// Usage:
// let arr: Array<i32, 10> = Array::new();



/*
// Key Concepts Summary:
Concept                  | Description            | Example                        
------------------------ | ---------------------- | ------------------------------ 
**Named Struct**         | Fields with names      | `struct User { name: String }` 
**Tuple Struct**         | Anonymous fields       | `struct Point(f64, f64)`       
**Unit Struct**          | No fields              | `struct Marker;`               
**Methods**              | Functions on instances | `fn area(&self) -> u32`        
**Associated Functions** | Functions on type      | `fn new() -> Self`             
**Generics**             | Type parameters        | `struct Point<T> { x: T }`     
**Lifetimes**            | Reference validity     | `struct Book<'a>`              
**Traits**               | Behavior definition    | `#[derive(Debug)]`             
**Impl**                 | Implementation block   | `impl Rectangle { ... }`       
**Const Generics**       | Compile-time constants | `struct Array<T, const N: usize>` 
*/

// powered by Kimi K2