// rust struct ownership

struct Owner {
    name: String, // Owns the String
}
struct Borrower<'a> {
    name: &'a str, // Borrows a string slice
}

fn main() {
    let owner = Owner { name: "Anup".to_string() }; // .to_string() or String::from()
    let name = String::from("Barua");
    let borrower = Borrower { name: &name };    
    println!("{} {}", owner.name, borrower.name);

    let name2 = String::from("Shajib Ahmed");
    let borrower2 = func1(&name2);
    println!("in main, borrower2 name {}", borrower2.name);

    println!("{}", func2().name);
    println!("{}", func3().name);

}

// func1 takes a reference and returns Borrower with same lifetime
fn func1<'a>(name: &'a str) -> Borrower<'a> {
    let borrower2 = Borrower { name };
    println!("in func1, borrower2 name: {}", borrower2.name);
    borrower2
}

fn func2() -> Owner {
    let name = String::from("Shajib Bhattacharjee");
    Owner { name }  // Moves ownership, no reference issues
}

fn func3() -> Borrower<'static> {
    let borrower2 = Borrower { name: "Shajib Chaudhuri" };  // &str literal
    borrower2
}

/*
Anup Barua
in func1, borrower2 name: Shajib Ahmed
in main, borrower2 name Shajib Ahmed
Shajib Bhattacharjee
Shajib Chaudhuri
*/

// built with Qwen 3.5