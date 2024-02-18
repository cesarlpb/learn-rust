/* Ejemplos de prints */
#[derive(Debug)]
#[allow(dead_code)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

fn main() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Pretty print
    println!("Pretty print:");
    println!("{:#?}", peter);
    println!();
    println!("Normal print:");
    println!("{:?}", peter);
    println!();
    println!("Print by field:");
    println!("Person {{");
    println!("  Name: {}", peter.name);
    println!("  Age: {}", peter.age);
    println!("}}");
}
